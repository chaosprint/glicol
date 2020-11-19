// extern crate pest;
// #[macro_use]
// extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex};
use std::{slice::from_raw_parts_mut};

// mod engine;
extern crate glicol;
use glicol::Engine;

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
    // static ref ENGINE:Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::new()));
    static ref ENGINE:Mutex<Engine> = Mutex::new(Engine::new());
}

#[no_mangle] // 64 f32 float // -> *mut [u8; 256] 
pub extern fn process_u8(out_ptr: *mut u8) {
    let mut engine = ENGINE.lock().unwrap();
    // engine.set_code("~ss: sin 440".to_string());
    // engine.update();
    let buf = engine.gen_next_buf_64(); // float *const [f32; 64]
    // let mut bytes: [u8; 256] = [0; 256];
    let out_buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(out_ptr, 256) };
    for i in 0..64 {
        // let b = 0.5_f32.to_le_bytes();
        // assert!(buf[i] == 0.0);
        let b = buf[i].to_le_bytes();
        for j in 0..4 {
            out_buf[i*4 + j] = b[j];
        }
    };
    // &mut out_buf
}

// Mutex<engine::Engine>
#[no_mangle]
pub extern "C" fn process(out_ptr: *mut f32, size: usize) -> usize {
    let mut engine = ENGINE.lock().unwrap();

    // error handling here
    // no need to use Result here
    // simply guarantee this is outputting 128 samples array
    let wave_buf = engine.gen_next_buf_128();

    let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    for i in 0..size {
        out_buf[i] = wave_buf[i] as f32
    };
    return 0;
    // engine.process128(out_ptr, size);s
}

#[no_mangle]
pub extern "C" fn run(
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
    let encoded:&'static mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let code: &'static str = std::str::from_utf8(encoded).unwrap();
    engine.set_code(code);
    engine.update();
}

#[no_mangle]
pub extern "C" fn update(arr_ptr: *mut u8, length: usize) {
    let mut engine = ENGINE.lock().unwrap();
    // assert!(engine.elapsed_samples > 44100, "update clock is starting from zero");
    // read the code from the text editor
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let code = std::str::from_utf8(encoded).unwrap();
    // push the code to engine
    engine.set_code(code);
    engine.update();
}

#[no_mangle]
pub extern "C" fn run_without_samples(arr_ptr: *mut u8, length: usize) {
    let mut engine = ENGINE.lock().unwrap();
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let code = std::str::from_utf8(encoded).unwrap();
    engine.set_code(code);
    engine.update();
    engine.make_graph();
}

#[no_mangle]
pub extern "C" fn reset() {
    let mut engine = ENGINE.lock().unwrap();
    engine.reset();
}