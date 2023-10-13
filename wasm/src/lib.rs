#[macro_use]
extern crate lazy_static;

use std::slice::from_raw_parts_mut;
use std::sync::Mutex;

use glicol::*;

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
// lazy_static! {
//     static ref ENGINE: Mutex<Graph> = Mutex::new(Graph::new(128, 2));
// }

lazy_static! {
    static ref ENGINE: Mutex<Graph3<128, 2>> = Mutex::new(Graph3::<128, 2>::new(512, 512));
}

#[no_mangle]
pub extern "C" fn process(_in_ptr: *mut f32, out_ptr: *mut f32, size: usize, result_ptr: *mut u8) {
    let mut g = ENGINE.lock().unwrap();

    // let c = g.add_node(Box::new(SinOsc::new(440., 44100)));
    // let m = g.add_node(Box::new(Mul::new(0.5)));
    // g.add_edge(c, m).unwrap();
    // g.connect_to_destination(m).unwrap();

    // let _in_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(in_ptr, 128) };
    let result: &mut [u8] = unsafe { from_raw_parts_mut(result_ptr, 256) };

    // let (engine_out, console) = engine.next_block(vec![]);
    let buffer = g.yield_next_buffer().unwrap();

    let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    for i in 0..128 {
        // out_buf[i] = buffer.data[i][0] as f32;
        out_buf[i] = buffer[i] as f32;
        // out_buf[i + 128] = 0.0;
        // out_buf[i + 128] = buffer.data[i][1] as f32;
    }
    // for i in 0..256 {
    //     result[i] = 0
    // }
}

#[no_mangle]
pub extern "C" fn add_sample(
    name_ptr: *mut u8,
    name_len: usize,
    arr_ptr: *mut f32,
    length: usize,
    channels: usize,
    sr: usize,
) {
    // let mut engine = ENGINE.lock().unwrap();
    // let encoded: &mut [u8] = unsafe { from_raw_parts_mut(name_ptr, name_len) };
    // let name = std::str::from_utf8(encoded).unwrap();
    // let sample: &mut [f32] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    // engine.add_sample(name, sample, channels, sr);
    // // engine.update(code);
}

#[no_mangle]
pub extern "C" fn update(arr_ptr: *mut u8, length: usize) {
    //, result_ptr: *mut u8

    // let mut engine = ENGINE.lock().unwrap();
    // let encoded: &mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    // let code = std::str::from_utf8(encoded).unwrap();
    // // let result:&mut [u8] = unsafe { from_raw_parts_mut(result_ptr, 256) };
    // // assert_eq!(code, "o: sin 110");
    // engine.update_with_code(code);
}

#[no_mangle]
pub extern "C" fn send_msg(arr_ptr: *mut u8, length: usize) {
    //, result_ptr: *mut u8

    // let mut engine = ENGINE.lock().unwrap();
    // let encoded: &mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    // let msg = std::str::from_utf8(encoded).unwrap();
    // engine.send_msg(msg);
}

#[no_mangle]
pub extern "C" fn live_coding_mode(io: bool) {
    // let mut engine = ENGINE.lock().unwrap();
    // engine.livecoding = io;
}

#[no_mangle]
pub extern "C" fn set_bpm(bpm: f32) {
    // let mut engine = ENGINE.lock().unwrap();
    // engine.set_bpm(bpm);
    // engine.reset();
}

#[no_mangle]
pub extern "C" fn set_track_amp(amp: f32) {
    // let mut engine = ENGINE.lock().unwrap();
    // engine.set_track_amp(amp);
}

#[no_mangle]
pub extern "C" fn set_sr(sr: f32) {
    // let mut engine = ENGINE.lock().unwrap();
    // engine.set_sr(sr as usize);

    let mut g = ENGINE.lock().unwrap();
    g.update_order();
    // let c = g.add_node(Box::new(SinOsc::new(440., sr as u32)));
    // let m = g.add_node(Box::new(Mul::new(0.1)));
    // let m2 = g.add_node(Box::new(Mul::new(0.9)));
    // let m3 = g.add_node(Box::new(Mul::new(0.9)));
    // let m4 = g.add_node(Box::new(Mul::new(0.9)));
    // g.add_edge(c, m).unwrap();
    // g.add_edge(m, m2).unwrap();
    // g.add_edge(m2, m3).unwrap();
    // g.add_edge(m3, m4).unwrap();
    // g.connect_to_destination(m).unwrap();
}

#[no_mangle]
pub extern "C" fn set_seed(seed: f32) {
    // let mut engine = ENGINE.lock().unwrap();
    // engine.set_seed(seed as usize);
}

#[no_mangle]
pub extern "C" fn reset() {
    // let mut engine = ENGINE.lock().unwrap();
    // engine.reset();
}
