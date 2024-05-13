#[macro_use]
extern crate lazy_static;
use std::slice::from_raw_parts_mut;
use std::sync::Mutex;

use glicol::{Engine, EngineError};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn alloc_uint8array(length: usize) -> *mut u8 {
    let mut arr = Vec::<u8>::with_capacity(length);
    let ptr = arr.as_mut_ptr();
    std::mem::forget(arr);
    ptr
}

lazy_static! {
    static ref ENGINE: Mutex<Engine<128>> = Mutex::new(Engine::<128>::new());
}

/// # Safety
///
/// - in_ptr must be aligned and non-null
/// - out_ptr must aligned and non-null
#[no_mangle]
pub unsafe extern "C" fn process(in_ptr: *mut f32, out_ptr: *mut f32, size: usize) {
    let mut engine = ENGINE.lock().unwrap();

    let _in_buf: &mut [f32] = unsafe { from_raw_parts_mut(in_ptr, 128) };

    let engine_out = engine.next_block(vec![]);

    let out_buf: &mut [f32] = unsafe { from_raw_parts_mut(out_ptr, size) };

    out_buf[..128].copy_from_slice(&engine_out[0][..128]);
    out_buf[128..].copy_from_slice(&engine_out[1][..128]);
}

/// # Safety
///
/// - name_ptr must be aligned and non-null
/// - arr_ptr must aligned and non-null
#[no_mangle]
pub unsafe extern "C" fn add_sample(
    name_ptr: *mut u8,
    name_len: usize,
    arr_ptr: *mut f32,
    length: usize,
    channels: usize,
    sr: usize,
) {
    let mut engine = ENGINE.lock().unwrap();
    let encoded: &mut [u8] = unsafe { from_raw_parts_mut(name_ptr, name_len) };
    let name = std::str::from_utf8(encoded).unwrap();
    let sample: &mut [f32] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    engine.add_sample(name, sample, channels, sr);
    // engine.update(code);
}

/// # Safety
///
/// - arr_ptr must be aligned and non-null
/// - result_ptr must be aligned and non-null
#[no_mangle]
pub unsafe extern "C" fn update(arr_ptr: *mut u8, length: usize, result_ptr: *mut u8) {
    //, result_ptr: *mut u8
    let mut engine = ENGINE.lock().unwrap();
    let encoded: &mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let code = std::str::from_utf8(encoded).unwrap();

    // assert_eq!(code, "o: sin 110");

    if let Err(e) = engine.update_with_code(code) {
        let result: &mut [u8] = unsafe { from_raw_parts_mut(result_ptr, RES_BUFFER_SIZE) };
        write_err_to_buf(e, result);
    }
}

/// # Safety
///
/// - arr_ptr must be aligned and non-null
#[no_mangle]
pub unsafe extern "C" fn send_msg(arr_ptr: *mut u8, length: usize) {
    //, result_ptr: *mut u8

    let mut engine = ENGINE.lock().unwrap();
    let encoded: &mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
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

const RES_BUFFER_SIZE: usize = 256;

fn write_err_to_buf(err: EngineError, result: &mut [u8]) {
    result[0] = match err {
        EngineError::ParsingError(_) => 1,
        EngineError::NonExistSample(_) => 2,
        EngineError::NonExistReference(_) => 3,
    };

    let error = match err {
        EngineError::ParsingError(v) => {
            let location = match v.location {
                pest::error::InputLocation::Pos(u) => u,
                pest::error::InputLocation::Span((s, _)) => s,
            };
            let (line, col) = match v.line_col {
                pest::error::LineColLocation::Pos(u) => u,
                pest::error::LineColLocation::Span(u, _) => u,
            };
            let (positives, negatives) = match &v.variant {
                pest::error::ErrorVariant::ParsingError {
                    positives,
                    negatives,
                } => (positives, negatives),
                _ => panic!("unknown parsing error"),
            };

            format!(
                "pos[{:?}], line[{:?}], col[{:?}], positives{:?}, negatives{:?}",
                location, line, col, positives, negatives
            )
        }
        EngineError::NonExistSample(v) => format!("There is no sample named {v}"),
        EngineError::NonExistReference(v) => format!("There is no reference named {v}"),
    };

    let s = error.as_bytes();
    let max_idx = s.len().min(RES_BUFFER_SIZE) - 2;
    result[2..][..max_idx].copy_from_slice(s);
    for byte in &mut result[2 + max_idx..] {
        *byte = 0;
    }
}
