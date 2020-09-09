extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, Arc};
use std::{slice::from_raw_parts_mut};

mod engine;
// use engine::{QuaverSignal, Event, QuaverLoop};
// use engine::instrument::{Sampler, QuaverFunction};
// instrument::Oscillator,
// use engine::effect::LPF;

// use dasp::{signal};
// use dasp::signal::Signal;
// use engine::{SinOsc, Mul, Add, Impulse, Sampler, Looper};
// use dasp_graph::{NodeData, BoxedNodeSend};
// Buffer, Input, Node, , BoxedNode 
// use petgraph::graph::{NodeIndex};

#[no_mangle] // to send buffer to JS
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

#[no_mangle] // for receiving the String from JS
pub extern "C" fn alloc_uint8array(length: usize) -> *mut f32 {
    let mut arr = Vec::<u8>::with_capacity(length);
    let ptr = arr.as_mut_ptr();
    std::mem::forget(arr);
    ptr as *mut f32
}

#[no_mangle] // for receiving the String from JS
pub extern "C" fn alloc_uint32array(length: usize) -> *mut f32 {
    let mut arr = Vec::<u32>::with_capacity(length);
    let ptr = arr.as_mut_ptr();
    std::mem::forget(arr);
    ptr as *mut f32
}

lazy_static! {
    static ref ENGINE:Arc<Mutex<engine::Engine>> = Arc::new(Mutex::new(engine::Engine::new()));
    // static ref ENGINE:Mutex<engine::Engine> = Mutex::new(engine::Engine::new());
}

// Mutex<engine::Engine>
#[no_mangle]
pub extern "C" fn process(out_ptr: *mut f32, size: usize) {
    let mut engine = ENGINE.lock().unwrap();
    engine.process(out_ptr, size);
}

#[no_mangle]
pub extern "C" fn create_new_track(
    arr_ptr: *mut u8, length: usize,
    samples_ptr: *mut *mut f32, samples_len: usize,
    lengths_ptr: *mut *mut usize, lengths_len: usize,
    names_ptr: *mut *mut u8, names_len: usize,
    names_len_ptr: *mut *mut usize, names_len_len: usize
    ) {

    let mut engine = ENGINE.lock().unwrap();
    
    // an array containing all pointers of samples
    let samples: &mut [*mut f32] = unsafe {
        from_raw_parts_mut(samples_ptr, samples_len)};
    let lengths: &mut [*mut usize] = unsafe {
        from_raw_parts_mut(lengths_ptr, lengths_len)};
    let names: &mut [*mut u8] = unsafe {
        from_raw_parts_mut(names_ptr, names_len)};
    let names_len: &mut [*mut usize] = unsafe {
        from_raw_parts_mut(names_len_ptr, names_len_len)};
    
    // save samples in a HashMap
    for i in 0..samples.len() {
        let sample_array: &'static[f32] = unsafe {from_raw_parts_mut(samples[i], lengths[i] as usize)};
        // let st = unsafe {from_raw_parts_mut(samples[i], lengths[i] as usize)};
        // let sample_array = 
        let name_encoded:&mut [u8] = unsafe { from_raw_parts_mut(names[i], names_len[i] as usize) };
        let name = std::str::from_utf8(name_encoded).unwrap();
        engine.samples_dict.insert(name.to_string(), sample_array);
    };

    // read the code from the text editor
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let quaver_code = std::str::from_utf8(encoded).unwrap();
    // push the code to engine
    engine.code = quaver_code.to_string();
    engine.update = true;

        // engine.chains.insert(ref_name.to_string(), func_chain); // sig: sig_chain
    // engine.phase = 0;
}