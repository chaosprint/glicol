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
pub extern "C" fn alloc_uint8array(length: usize) -> *mut u8 {
    let mut arr = Vec::<u8>::with_capacity(length);
    let ptr = arr.as_mut_ptr();
    std::mem::forget(arr);
    ptr
}

lazy_static! {
    static ref ENGINE:Mutex<Engine<128>> = Mutex::new(Engine::<128>::new());
}

#[no_mangle]
pub extern "C" fn process(in_ptr: *mut f32, out_ptr: *mut f32, size: usize, result_ptr: *mut u8) {
    let mut engine = ENGINE.lock().unwrap();

    let _in_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(in_ptr, 128) };
    let result:&mut [u8] = unsafe { from_raw_parts_mut(result_ptr, 256) };
    
    let (engine_out, console) = engine.next_block(vec![]);

    let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    for i in 0..128 {
        out_buf[i] = engine_out[0][i] as f32;
        out_buf[i+128] = engine_out[1][i] as f32;       
    };
    for i in 0..256 {
        result[i] = console[i]
    }
}

#[no_mangle]
pub extern "C" fn add_sample(
    name_ptr: *mut u8,
    name_len: usize,
    arr_ptr: *mut f32, 
    length: usize, 
    channels: usize,
    sr: usize
) {
    let mut engine = ENGINE.lock().unwrap();
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(name_ptr, name_len) };
    let name = std::str::from_utf8(encoded).unwrap();
    let sample:&mut [f32] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    engine.add_sample(name, sample, channels, sr);
    // engine.update(code);
}


#[no_mangle]
pub extern "C" fn update(arr_ptr: *mut u8, length: usize) { //, result_ptr: *mut u8
    
    let mut engine = ENGINE.lock().unwrap();
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let code = std::str::from_utf8(encoded).unwrap();
    // let result:&mut [u8] = unsafe { from_raw_parts_mut(result_ptr, 256) };
    // assert_eq!(code, "o: sin 110");
    engine.update_with_code(code);
}

#[no_mangle]
pub extern "C" fn send_msg(arr_ptr: *mut u8, length: usize) { //, result_ptr: *mut u8
    
    let mut engine = ENGINE.lock().unwrap();
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let msg = std::str::from_utf8(encoded).unwrap();
    engine.send_msg(msg);
}

#[no_mangle]
pub extern "C" fn live_coding_mode(io: bool) {
    let mut engine = ENGINE.lock().unwrap();
    engine.livecoding = io;
}

#[no_mangle]
pub extern "C" fn set_bpm(bpm: f32) {
    let mut engine = ENGINE.lock().unwrap();
    engine.set_bpm(bpm);
    // engine.reset();
}

#[no_mangle]
pub extern "C" fn set_track_amp(amp: f32) {
    let mut engine = ENGINE.lock().unwrap();
    engine.set_track_amp(amp);
}

#[no_mangle]
pub extern "C" fn set_sr(sr: f32) {
    let mut engine = ENGINE.lock().unwrap();
    engine.set_sr(sr as usize);
}

#[no_mangle]
pub extern "C" fn set_seed(seed: f32) {
    let mut engine = ENGINE.lock().unwrap();
    engine.set_seed(seed as usize);
}

#[no_mangle]
pub extern "C" fn reset() {
    let mut engine = ENGINE.lock().unwrap();
    engine.reset();
}