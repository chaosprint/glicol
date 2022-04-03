extern crate nix;
extern crate libc;
extern crate bela_sys;

use bela_sys::{BelaInitSettings, BelaContext};
use std::{thread, time};
use std::{mem, slice};
use std::marker::PhantomData;

pub mod error;

pub enum DigitalDirection {
    INPUT,
    OUTPUT,
}

/// The `Bela` struct is essentially built to ensure that the type parameter
/// `<T>` is consistent across all invocations of the setup, render, and cleanup
/// functions. This is because `<T>` is the `UserData` of the original Bela
/// library -- we want to ensure that the `UserData` we are initializing with
/// is the exact same as the one we are attempting to access with each function.
/// 
/// TODO: Bela needs to also wrap the various setup, render, and cleanup
/// functions and keep them in the same struct.
/// 
/// Called when audio is initialized.
/// 
/// ```rust
/// pub type SetupFn = FnOnce(&mut Context, T) -> bool;
/// ```
/// 
/// Called on every frame.
/// 
/// ```rust
/// pub type RenderFn = Fn(&mut Context, T);
/// ```
/// 
/// Called when audio is stopped.
/// 
/// ```rust
/// pub type CleanupFn = FnOnce(&mut Context, T) -> bool;
/// ```
pub struct Bela<T> {
    initialized: bool,
    user_data: T,
}

unsafe extern "C" fn render_trampoline<'a, T>(context: *mut BelaContext, user_data: *mut std::os::raw::c_void) 
where T: UserData<'a> + 'a
{
    let mut context = Context::new(context);
    let user_data: &mut T = mem::transmute(user_data);
    user_data.render_fn(&mut context);
}

unsafe extern "C" fn setup_trampoline<'a, T>(context: *mut BelaContext, user_data: *mut std::os::raw::c_void) -> bool
where T: UserData<'a> + 'a
{
    let mut context = Context::new(context);
    let user_data: &mut T = mem::transmute(user_data);
    match user_data.setup_fn(&mut context) {
        Ok(_) => true,
        Err(_) => false,
    }
}

unsafe extern "C" fn cleanup_trampoline<'a, T>(context: *mut BelaContext, user_data: *mut std::os::raw::c_void)
where T: UserData<'a> + 'a
{
    let mut context = Context::new(context);
    let user_data: &mut T = mem::transmute(user_data);
    user_data.cleanup_fn(&mut context);
}

/// The "args" here must include the actual auxiliary task callback!
unsafe extern "C" fn auxiliary_task_trampoline<T>(aux_ptr: *mut std::os::raw::c_void) 
where T: Auxiliary
{
    let auxiliary: &mut T = mem::transmute(aux_ptr);
    let (callback, args) = auxiliary.destructure();
    callback(args);
}

/// Trait for `AuxiliaryTask`s, which run at a lower priority than the audio
/// thread.
/// 
/// An `AuxiliaryTask` must contain both its callback closure and its arguments;
/// this is so that we can capture outer variables in the closure, and also
/// mutate state if we need to in a type-safe way.  
pub trait Auxiliary {
    type Args: ?Sized;

    /// `destructure` should split the Auxiliary into the closure and its
    /// arguments. This is called by the `unsafe extern` trampoline function to
    /// actually run the task at the proper Xenomai priority.
    fn destructure(&mut self) -> (&mut dyn FnMut(&mut Self::Args), &mut Self::Args);
}

impl<T> Auxiliary for Box<T>
where T: Auxiliary + ?Sized
{
    type Args = T::Args;

    fn destructure(&mut self) -> (&mut dyn FnMut(&mut Self::Args), &mut Self::Args) {
        T::destructure(self)
    }
}

pub struct CreatedTask<'a>(bela_sys::AuxiliaryTask, PhantomData<&'a mut ()>);

impl<'a, T: UserData<'a> + 'a> Bela<T> {
    pub fn new(user_data: T) -> Self {
        Bela {
            initialized: false,
            user_data,
        }
    }

    pub fn run(&mut self, settings: &mut InitSettings) -> Result<(), error::Error> {
        self.init_audio(settings)?;
        self.start_audio()?;
        while !self.should_stop() {
            thread::sleep(time::Duration::new(0, 1000));
        }

        self.stop_audio();
        self.cleanup_audio();

        Ok(())
    }

    pub fn set_render<F: 'a>(&mut self, func: &'a mut F) 
    where F: FnMut(&mut Context, T::Data),
          for<'r, 's> F: FnMut(&'r mut Context, &'s mut T::Data)
    {
        self.user_data.set_render_fn(func);
    }

    pub fn set_setup<F: 'a>(&mut self, func: &'a mut F) 
    where F: FnMut(&mut Context, T::Data) -> bool,
          for<'r, 's> F: FnMut(&'r mut Context, &'s mut T::Data) -> Result<(), error::Error>
    {
        self.user_data.set_setup_fn(Some(func));
    }

    pub fn set_cleanup<F: 'a>(&mut self, func: &'a mut F) 
    where F: FnMut(&mut Context, T::Data),
          for<'r, 's> F: FnMut(&'r mut Context, &'s mut T::Data)
    {
        self.user_data.set_cleanup_fn(Some(func));
    }

    pub fn init_audio(&mut self, settings: &mut InitSettings) -> Result<(), error::Error> {
        settings.settings.setup = Some(setup_trampoline::<T>);
        settings.settings.render = Some(render_trampoline::<T>);
        settings.settings.cleanup = Some(cleanup_trampoline::<T>);
        let out = unsafe {
            let ptr: *mut std::os::raw::c_void = mem::transmute(&mut self.user_data);
            bela_sys::Bela_initAudio(settings.settings_ptr(), ptr)
        };

        match out {
            0 => { 
                self.initialized = true;
                Ok(())
            },
            _ => Err(error::Error::Init),
        }
    }

    pub fn start_audio(&self) -> Result<(), error::Error> {
        if !self.initialized { 
            return Err(error::Error::Start); 
        }

        let out = unsafe {
            bela_sys::Bela_startAudio()
        };

        match out {
            0 => Ok(()),
            _ => Err(error::Error::Start),
        }
    }

    pub fn should_stop(&self) -> bool {
        unsafe {
            bela_sys::gShouldStop != 0
        }
    }

    /// Takes a _mutable reference_ to the task, because we must be ensured that
    /// the task is unique and that it does not move.
    /// 
    /// I highly recommend ONLY USING STACK-ALLOCATED CLOSURES AS TASKS. This
    /// particular implementation is wildly unsafe, but if you use a stack
    /// closure it _should_ be possible to avoid a segfault. See the
    /// auxiliary_task example for a demo.
    pub fn create_auxiliary_task<'b, 'c, A: 'b>(task: &'c mut A, priority: i32, name: &'static str) -> CreatedTask<'b>
    where A: Auxiliary
    {
        let task_ptr = task as *const _ as *mut std::os::raw::c_void;

        let aux_task = unsafe {
            bela_sys::Bela_createAuxiliaryTask(
                Some(auxiliary_task_trampoline::<A>),
                priority,
                name.as_bytes().as_ptr(),
                task_ptr,
            )
        };

        CreatedTask(aux_task, PhantomData)
    }

    pub fn schedule_auxiliary_task(task: &CreatedTask) -> Result<(), error::Error> 
    {
        let res = unsafe {
            bela_sys::Bela_scheduleAuxiliaryTask(task.0)
        };

        match res {
            0 => Ok(()),
            _ => Err(error::Error::Task),
        }
    }

    pub fn stop_audio(&self) {
        unsafe { bela_sys::Bela_stopAudio(); }
    }

    pub fn cleanup_audio(&self) {
        unsafe { bela_sys::Bela_cleanupAudio(); }
    }
}

/// Wraps `BelaContext`
pub struct Context {
    context: *mut BelaContext,
}

impl Context {
    pub fn new(context: *mut BelaContext) -> Context {
        Context {
            context
        }
    }

    pub fn context_mut_ptr(&mut self) -> *mut BelaContext {
        let ptr: *mut BelaContext = self.context;
        ptr
    }

    pub fn context_ptr(&self) -> *const BelaContext {
        let ptr: *mut BelaContext = self.context;
        ptr
    }

    /// Access the audio output slice
    ///
    /// Mutably borrows self so that (hopefully) we do not have multiple mutable
    /// pointers to the audio buffer available simultaneously.
    pub fn audio_out(&mut self) -> &mut [f32] {
        unsafe {
            let context = self.context_mut_ptr();
            let n_frames = (*context).audioFrames;
            let n_channels = (*context).audioOutChannels;
            let audio_out_ptr = (*context).audioOut as *mut f32;
            slice::from_raw_parts_mut(audio_out_ptr, (n_frames * n_channels) as usize)
        }
    }

    /// Access the audio input slice
    /// 
    /// Immutably borrows self and returns an immutable buffer of audio in data.
    pub fn audio_in(&self) -> &[f32] {
        unsafe {
            let context = self.context_ptr();
            let n_frames = (*context).audioFrames;
            let n_channels = (*context).audioInChannels;
            let audio_in_ptr = (*context).audioIn as *const f32;
            slice::from_raw_parts(audio_in_ptr, (n_frames * n_channels) as usize)
        }
    }

    /// Access the digital input/output slice
    ///
    /// Mutably borrows self so that (hopefully) we do not have multiple mutable
    /// pointers to the digital buffer available simultaneously.
    pub fn digital(&mut self) -> &mut [u32] {
        unsafe {
            let context = self.context_ptr();
            let n_frames = (*context).digitalFrames;
            let n_channels = (*context).digitalChannels;
            let digital_ptr = (*context).digital as *mut u32;
            slice::from_raw_parts_mut(digital_ptr, (n_frames * n_channels) as usize)
        }
    }

    /// Access the analog output slice
    ///
    /// Mutably borrows self so that (hopefully) we do not have multiple mutable
    /// pointers to the analog buffer available simultaneously.
    pub fn analog_out(&mut self) -> &mut [f32] {
        unsafe {
            let context = self.context_ptr();
            let n_frames = (*context).analogFrames;
            let n_channels = (*context).analogOutChannels;
            let analog_out_ptr = (*context).analogOut as *mut f32;
            slice::from_raw_parts_mut(analog_out_ptr, (n_frames * n_channels) as usize)
        }
    }

    /// Access the analog input slice
    pub fn analog_in(&self) -> &[f32] {
        unsafe {
            let n_frames = (*self.context).analogFrames;
            let n_channels = (*self.context).analogInChannels;
            let analog_in_ptr = (*self.context).analogIn as *mut f32;
            slice::from_raw_parts(analog_in_ptr, (n_frames * n_channels) as usize)
        }
    }

    pub fn audio_frames(&self) -> usize {
        unsafe {
            (*self.context).audioFrames as usize
        }
    }

    pub fn audio_in_channels(&self) -> usize {
        unsafe {
            (*self.context).audioInChannels as usize
        }
    }

    pub fn audio_out_channels(&self) -> usize {
        unsafe {
            (*self.context).audioOutChannels as usize
        }
    }

    pub fn  audio_sample_rate(&self) -> f32 {
        unsafe {
            (*self.context).audioSampleRate
        }
    }

    pub fn analog_frames(&self) -> usize {
      unsafe {
          (*self.context).analogFrames as usize
      }
    }

    pub fn analog_in_channels(&self) -> usize {
        unsafe {
            (*self.context).analogInChannels as usize
        }
    }

    pub fn analog_out_channels(&self) -> usize {
        unsafe {
            (*self.context).analogOutChannels as usize
        }
    }

    pub fn  analog_sample_rate(&self) -> f32 {
        unsafe {
            (*self.context).analogSampleRate
        }
    }

    pub fn digital_frames(&self) -> usize {
      unsafe {
          (*self.context).digitalFrames as usize
      }
    }

    pub fn digital_channels(&self) -> usize {
        unsafe {
            (*self.context).digitalChannels as usize
        }
    }

    pub fn  digital_sample_rate(&self) -> f32 {
        unsafe {
            (*self.context).digitalSampleRate
        }
    }

    pub fn audio_frames_elapsed(&self) -> usize {
        unsafe {
            (*self.context).audioFramesElapsed as usize
        }
    }

    pub fn multiplexer_channels(&self) -> usize {
      unsafe {
          (*self.context).multiplexerChannels as usize
      }
    }

    pub fn multiplexer_starting_channels(&self) -> usize {
      unsafe {
          (*self.context).multiplexerStartingChannel as usize
      }
    }

    pub fn multiplexer_analog_in(&self) -> &mut [f32] {
        unsafe {
            let n_frames = (*self.context).analogFrames;
            let n_channels = (*self.context).multiplexerChannels;
            let analog_in_ptr = (*self.context).multiplexerAnalogIn as *mut f32;
            slice::from_raw_parts_mut(analog_in_ptr, (n_frames * n_channels) as usize)
        }
    }

    pub fn multiplexer_enabled(&self) -> u32 {
        unsafe {
            (*self.context).audioExpanderEnabled
        }
    }

    pub fn flags(&self) -> u32 {
        unsafe {
            (*self.context).flags
        }
    }

    // Returns the value of a given digital input at the given frame number
    pub fn digital_read(&mut self, frame: usize, channel: usize) -> u32 {
        let digital = self.digital();
        (digital[frame] >> (channel + 16)) & 1
    }

    // Sets a given digital output channel to a value for the current frame and all subsequent frames
    pub fn digital_write(&mut self, frame: usize, channel: usize, value: usize) {
        let digital = self.digital();
        for i in frame..digital.len() {
            if value != 0 {
                digital[i] |= 1 << (channel + 16);
            } else {
                digital[i] &= !(1 << (channel + 16));
            }
        }
    }

    // Sets a given digital output channel to a value for the current frame only
    pub fn digital_write_once(&mut self, frame: usize, channel: usize, value: usize) {
        let digital = self.digital();
        if value != 0 {
            digital[frame] |= 1 << (channel + 16);
        } else {
            digital[frame] &= !(1 << (channel + 16));
        }
    }

    // Sets the direction of a digital pin for the current frame and all subsequent frames
    pub fn pin_mode(&mut self, frame: usize, channel: usize, mode: DigitalDirection) {
        let digital = self.digital();
        for i in frame..digital.len() {
            match mode {
                DigitalDirection::INPUT => { digital[i] |= 1 << channel; }
                DigitalDirection::OUTPUT => { digital[i] &= !(1 << channel); }
            }
        }
    }

    // Sets the direction of a digital pin for the current frame only
    pub fn pin_mode_once(&mut self, frame: usize, channel: usize, mode: DigitalDirection) {
        let digital = self.digital();
        match mode {
            DigitalDirection::INPUT => { digital[frame] |= 1 << channel; }
            DigitalDirection::OUTPUT => { digital[frame] &= !(1 << channel); }
        }
    }
}

pub trait UserData<'a> {
    type Data;

    fn render_fn(&mut self, &mut Context);
    fn set_render_fn(&mut self, &'a mut dyn FnMut(&mut Context, &mut Self::Data));
    fn setup_fn(&mut self, &mut Context) -> Result<(), error::Error>;
    fn set_setup_fn(&mut self, Option<&'a mut dyn FnMut(&mut Context, &mut Self::Data) -> Result<(), error::Error>>);
    fn cleanup_fn(&mut self, &mut Context);
    fn set_cleanup_fn(&mut self, Option<&'a mut dyn FnMut(&mut Context, &mut Self::Data)>);
}

pub struct AppData<'a, D: 'a> {
    pub data: D,
    render: &'a mut dyn FnMut(&mut Context, &mut D),
    setup: Option<&'a mut dyn FnMut(&mut Context, &mut D) -> Result<(), error::Error>>,
    cleanup: Option<&'a mut dyn FnMut(&mut Context, &mut D)>,
}

impl<'a, D> AppData<'a, D> {
    pub fn new(data: D, 
        render: &'a mut dyn FnMut(&mut Context, &mut D), 
        setup: Option<&'a mut dyn FnMut(&mut Context, &mut D) -> Result<(), error::Error>>, 
        cleanup: Option<&'a mut dyn FnMut(&mut Context, &mut D)>) -> AppData<'a, D> 
    {
        AppData {
            data,
            render,
            setup,
            cleanup,
        }
    }
}

impl<'a, D> UserData<'a> for AppData<'a, D> {
    type Data = D;

    fn render_fn(&mut self, context: &mut Context) {
        let AppData {
            render,
            data,
            ..
        } = self;

        render(context, data)
    }

    fn set_render_fn(&mut self, callback: &'a mut (dyn FnMut(&mut Context, &mut D) + 'a)) {
        self.render = callback;
    }

    fn setup_fn(&mut self, context: &mut Context) -> Result<(), error::Error> {
        let AppData {
            setup,
            data,
            ..
        } = self;

        match setup {
            Some(f) => f(context, data),
            None => Ok(()),
        }
    }

    fn set_setup_fn(&mut self, callback: Option<&'a mut (dyn FnMut(&mut Context, &mut D) -> Result<(), error::Error> + 'a)>) {
        self.setup = callback;
    }

    fn cleanup_fn(&mut self, context: &mut Context) {
        let AppData {
            cleanup,
            data,
            ..
        } = self;

        match cleanup {
            Some(f) => f(context, data),
            None => (),
        };
    }

    fn set_cleanup_fn(&mut self, callback: Option<&'a mut (dyn FnMut(&mut Context, &mut D) + 'a)>) {
        self.cleanup = callback;
    }
}

/// Safe wrapper for `BelaInitSettings`, which sets initial parameters for the
/// Bela system.
pub struct InitSettings {
    settings: BelaInitSettings,
}

impl InitSettings {
    pub fn settings_ptr(&mut self) -> *mut BelaInitSettings {
        &mut self.settings
    }

    /// Get number of analog frames per period (buffer). Number of audio frames
    /// depends on relative sample rates of the two. By default, audio is twice
    /// the sample rate, so has twice the period size.
    pub fn period_size(&self) -> usize {
        self.settings.periodSize as usize
    }

    /// Set number of analog frames per period (buffer). Number of audio frames
    /// depends on relative sample rates of the two. By default, audio is twice
    /// the sample rate, so has twice the period size.
    pub fn set_period_size(&mut self, size: usize) {
        self.settings.periodSize = size as i32
    }

    /// Get whether to use the analog input and output
    pub fn use_analog(&self) -> bool {
        match self.settings.useAnalog {
            0 => false,
            _ => true,
        }
    }

    /// Set whether to use the analog input and output
    pub fn set_use_analog(&mut self, use_analog: bool) {
        self.settings.useAnalog = match use_analog {
            true => 1,
            false => 0,
        };
    }

    /// Get whether to use the digital input and output
    pub fn use_digital(&self) -> bool {
        match self.settings.useDigital {
            0 => false,
            _ => true,
        }
    }

    /// Set whether to use the digital input and output
    pub fn set_use_digital(&mut self, use_digital: bool) {
        self.settings.useDigital = match use_digital {
            true => 1,
            false => 0,
        };
    }

    pub fn num_audio_in_channels(&self) -> usize {
        self.settings.numAudioInChannels as usize
    }

    pub fn set_num_audio_in_channels(&mut self, num: usize) {
        self.settings.numAudioInChannels = num as i32;
    }

    pub fn num_audio_out_channels(&self) -> usize {
        self.settings.numAudioOutChannels as usize
    }

    pub fn set_num_audio_out_channels(&mut self, num: usize) {
        self.settings.numAudioOutChannels = num as i32;
    }

    pub fn num_analog_in_channels(&self) -> usize {
        self.settings.numAnalogInChannels as usize
    }

    pub fn set_num_analog_in_channels(&mut self, num: usize) {
        self.settings.numAnalogInChannels = num as i32;
    }

    pub fn num_analog_out_channels(&self) -> usize {
        self.settings.numAnalogOutChannels as usize
    }

    pub fn set_num_analog_out_channels(&mut self, num: usize) {
        self.settings.numAnalogOutChannels = num as i32;
    }

    pub fn num_digital_channels(&self) -> usize {
        self.settings.numDigitalChannels as usize
    }

    pub fn set_num_digital_channels(&mut self, num: usize) {
        self.settings.numDigitalChannels = num as i32;
    }

    pub fn begin_muted(&self) -> bool {
        match self.settings.beginMuted {
            0 => false,
            _ => true
        }
    }

    pub fn set_begin_muted(&mut self, val: bool) {
        self.settings.beginMuted = match val {
            true => 1,
            false => 0
        };
    }

    pub fn dac_level(&self) -> f32 {
        self.settings.dacLevel
    }

    pub fn set_dac_level(&mut self, val: f32) {
        self.settings.dacLevel = val;
    }

    pub fn adc_level(&self) -> f32 {
        self.settings.adcLevel
    }

    pub fn set_adc_level(&mut self, val: f32) {
        self.settings.adcLevel = val;
    }

    pub fn pga_gain(&self) -> [f32; 2] {
        self.settings.pgaGain
    }

    pub fn set_pga_gain(&mut self, val: [f32; 2]) {
        self.settings.pgaGain = val;
    }

    pub fn headphone_level(&self) -> f32 {
        self.settings.headphoneLevel
    }

    pub fn set_headphone_level(&mut self, val: f32) {
        self.settings.headphoneLevel = val;
    }

    pub fn num_mux_channels(&self) -> usize {
        self.settings.numMuxChannels as usize
    }

    pub fn set_num_mux_channels(&mut self, val: usize) {
        self.settings.numMuxChannels = val as i32;
    }

    pub fn audio_expander_inputs(&self) -> usize {
        self.settings.audioExpanderInputs as usize
    }

    pub fn set_audio_expander_inputs(&mut self, val: usize) {
        self.settings.audioExpanderInputs = val as u32;
    }

    pub fn audio_expander_outputs(&self) -> usize {
        self.settings.audioExpanderOutputs as usize
    }

    pub fn set_audio_expander_outputs(&mut self, val: usize) {
        self.settings.audioExpanderOutputs = val as u32;
    }

    pub fn pru_number(&self) -> usize {
        self.settings.pruNumber as usize
    }

    pub fn set_pru_number(&mut self, val: usize) {
        self.settings.pruNumber = val as i32;
    }

    pub fn pru_filename(&self) -> [u8; 256] {
        self.settings.pruFilename
    }

    pub fn set_pru_filename(&mut self, val: [u8; 256]) {
        self.settings.pruFilename = val;
    }

    pub fn detect_underruns(&self) -> bool {
        match self.settings.detectUnderruns {
            0 => false,
            _ => true
        }
    }

    pub fn set_detect_underruns(&mut self, val: bool) {
        self.settings.detectUnderruns = match val {
            true => 1,
            false => 0
        };
    }

    pub fn verbose(&self) -> bool {
        match self.settings.verbose {
            0 => false,
            _ => true
        }
    }

    pub fn set_verbose(&mut self, val: bool) {
        self.settings.verbose = match val {
            true => 1,
            false => 0
        };
    }

    pub fn enable_led(&self) -> bool {
        match self.settings.enableLED {
            0 => false,
            _ => true
        }
    }

    pub fn set_enable_led(&mut self, val: bool) {
        self.settings.enableLED = match val {
            true => 1,
            false => 0
        };
    }

    pub fn enable_cape_button_monitoring(&self) -> bool {
        match self.settings.enableCapeButtonMonitoring {
            0 => false,
            _ => true
        }
    }

    pub fn set_enable_cape_button_monitoring(&mut self, val: bool) {
        self.settings.enableCapeButtonMonitoring = match val {
            true => 1,
            false => 0
        };
    }

    pub fn high_performance_mode(&self) -> bool {
        match self.settings.highPerformanceMode {
            0 => false,
            _ => true
        }
    }

    pub fn set_high_performance_mode(&mut self, val: bool) {
        self.settings.highPerformanceMode = match val {
            true => 1,
            false => 0
        };
    }

    pub fn interleave(&self) -> bool {
        match self.settings.interleave {
            0 => false,
            _ => true
        }
    }

    pub fn set_interleave(&mut self, val: bool) {
        self.settings.interleave = match val {
            true => 1,
            false => 0
        };
    }

    pub fn analog_outputs_persist(&self) -> bool {
        match self.settings.analogOutputsPersist {
            0 => false,
            _ => true
        }
    }

    pub fn set_analog_outputs_persist(&mut self, val: bool) {
        self.settings.analogOutputsPersist = match val {
            true => 1,
            false => 0
        };
    }

    pub fn uniform_sample_rate(&self) -> bool {
        match self.settings.uniformSampleRate {
            0 => false,
            _ => true
        }
    }

    pub fn set_uniform_sample_rate(&mut self, val: bool) {
        self.settings.uniformSampleRate = match val {
            true => 1,
            false => 0
        };
    }

    pub fn audio_thread_stack_size(&self) -> usize {
        self.settings.audioThreadStackSize as usize
    }

    pub fn set_audio_thread_stack_size(&mut self, num: usize) {
        self.settings.audioThreadStackSize = num as u32;
    }

    pub fn auxiliary_task_stack_size(&self) -> usize {
        self.settings.auxiliaryTaskStackSize as usize
    }

    pub fn set_auxiliary_task_stack_size(&mut self, num: usize) {
        self.settings.auxiliaryTaskStackSize = num as u32;
    }

    pub fn codec_i2c_address(&self) -> usize {
        self.settings.codecI2CAddress as usize
    }

    pub fn set_codec_i2c_address(&mut self, num: usize) {
        self.settings.codecI2CAddress = num as i32;
    }

    pub fn amp_mute_pin(&self) -> usize {
        self.settings.ampMutePin as usize
    }

    pub fn set_amp_mute_pin(&mut self, num: usize) {
        self.settings.ampMutePin = num as i32;
    }

    pub fn receive_port(&self) -> usize {
        self.settings.receivePort as usize
    }

    pub fn set_receive_port(&mut self, num: usize) {
        self.settings.receivePort = num as i32;
    }

    pub fn transmit_port(&self) -> usize {
        self.settings.transmitPort as usize
    }

    pub fn set_transmit_port(&mut self, num: usize) {
        self.settings.transmitPort = num as i32;
    }

    pub fn server_name(&self) -> [u8; 256] {
        self.settings.serverName 
    }

    pub fn set_server_name(&mut self, val: [u8; 256]) {
        self.settings.serverName = val;
    }
}

impl Default for InitSettings {
    fn default() -> InitSettings {
        let settings = unsafe {
            let mut settings: BelaInitSettings = mem::uninitialized();
            bela_sys::Bela_defaultSettings(&mut settings);
            settings
        };

        InitSettings {
            settings
        }
    }
}
