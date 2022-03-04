#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex};
use std::{slice::from_raw_parts_mut};

use glicol::Engine;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

#[no_mangle]
pub extern "C" fn alloc_uint8array(length: usize) -> *mut f32 {
    let mut arr = Vec::<u8>::with_capacity(length);
    let ptr = arr.as_mut_ptr();
    std::mem::forget(arr);
    ptr as *mut f32
}

#[no_mangle]
pub extern "C" fn alloc_uint32array(length: usize) -> *mut f32 {
    let mut arr = Vec::<u32>::with_capacity(length);
    let ptr = arr.as_mut_ptr();
    std::mem::forget(arr);
    ptr as *mut f32
}

lazy_static! {
    static ref ENGINE:Mutex<Engine<'static, 128>> = Mutex::new(Engine::<128>::new());
}

#[no_mangle]
pub extern "C" fn process(in_ptr: *mut f32, out_ptr: *mut f32, size: usize) {
    let mut engine = ENGINE.lock().unwrap();

    let _in_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(in_ptr, 128) };

    let engine_out = engine.next_block();
    let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };

    for i in 0..128{
        out_buf[i] = engine_out[0][i] as f32;
        out_buf[i+128] = engine_out[1][i] as f32;       
    };
    // let mut a = [0; 3];
    // console[0] = (sum * 100.0) as u8;
    // return console.as_mut_ptr();
    // engine.process128(out_ptr, size);s
}

// #[no_mangle]
// pub extern "C" fn run(
//     arr_ptr: *mut u8, length: usize,
//     samples_ptr: *mut *mut f32, samples_len: usize,
//     lengths_ptr: *mut *mut usize, lengths_len: usize,
//     names_ptr: *mut *mut u8, names_len: usize,
//     names_len_ptr: *mut *mut usize, names_len_len: usize
//     ) {

//     let mut engine = ENGINE.lock().unwrap();
    
//     // an array containing all pointers of samples
//     let samples: &mut [*mut f32] = unsafe {
//         from_raw_parts_mut(samples_ptr, samples_len)};
//     let lengths: &mut [*mut usize] = unsafe {
//         from_raw_parts_mut(lengths_ptr, lengths_len)};
//     let names: &mut [*mut u8] = unsafe {
//         from_raw_parts_mut(names_ptr, names_len)};
//     let names_len: &mut [*mut usize] = unsafe {
//         from_raw_parts_mut(names_len_ptr, names_len_len)};
    
//     // save samples in a HashMap
//     for i in 0..samples.len() {
//         let sample_array: &'static[f32] = unsafe {
//             from_raw_parts_mut(samples[i], lengths[i] as usize)};
//         // let st = unsafe {from_raw_parts_mut(samples[i], lengths[i] as usize)};
//         // let sample_array = 
//         let name_encoded:&mut [u8] = unsafe {
//             from_raw_parts_mut(names[i], names_len[i] as usize) };
//         let name = std::str::from_utf8(name_encoded).unwrap();
//         engine.samples_dict.insert(name.to_string(), sample_array);
//     };

//     // read the code from the text editor
//     let encoded:&'static mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
//     let code: &'static str = std::str::from_utf8(encoded).unwrap();
//     engine.set_code(code);
//     // engine.update();
// }

#[no_mangle]
pub extern "C" fn add_sample(
    name_ptr: *mut u8,
    name_len: usize,
    arr_ptr: *mut f32, 
    length: usize, 
    channels: usize
) {
    let mut engine = ENGINE.lock().unwrap();
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(name_ptr, name_len) };
    let name = std::str::from_utf8(encoded).unwrap();
    let sample:&mut [f32] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    engine.add_sample(name, sample, channels);
    // engine.update(code);
}


#[no_mangle]
pub extern "C" fn update(arr_ptr: *mut u8, length: usize) {
    let mut engine = ENGINE.lock().unwrap();
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let code = std::str::from_utf8(encoded).unwrap();
    engine.update(code);
}

// #[no_mangle]
// pub extern "C" fn run_without_samples(arr_ptr: *mut u8, length: usize) {
//     let mut engine = ENGINE.lock().unwrap();
//     let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
//     let code = std::str::from_utf8(encoded).unwrap();
//     engine.set_code(code);
// }

// #[no_mangle]
// pub extern "C" fn reset() {
//     let mut engine = ENGINE.lock().unwrap();
//     engine.reset();
// }

// #[no_mangle]
// pub extern "C" fn set_bpm(bpm: f32) {
//     let mut engine = ENGINE.lock().unwrap();
//     engine.bpm = bpm;
//     // engine.reset();
// }

// #[no_mangle]
// pub extern "C" fn set_track_amp(amp: f32) {
//     let mut engine = ENGINE.lock().unwrap();
//     engine.set_track_amp(amp);
// }

// #[no_mangle]
// pub extern "C" fn set_sr(sr: f32) {
//     let mut engine = ENGINE.lock().unwrap();
//     engine.set_sr(sr as usize);
// }

// #[no_mangle]
// pub extern "C" fn set_seed(seed: f32) {
//     let mut engine = ENGINE.lock().unwrap();
//     engine.set_seed(seed as usize);
// }