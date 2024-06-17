#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, MutexGuard};

use glicol::{Engine, EngineError};
use wasm_bindgen::prelude::wasm_bindgen;

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

fn get_engine() -> MutexGuard<'static, Engine<128>> {
    ENGINE.lock().unwrap_or_else(|e| e.into_inner())
}

#[wasm_bindgen]
pub fn process(size: usize) -> Vec<f32> {
    let mut engine = get_engine();

    let engine_out = engine.next_block(vec![]);

    let mut out_buf = vec![0.; size];
    let half_size = size / 2;
    out_buf[..half_size].copy_from_slice(&engine_out[0][..half_size]);
    out_buf[half_size..].copy_from_slice(&engine_out[1][..half_size]);

    out_buf
}

#[wasm_bindgen]
pub fn add_sample(
    name: String,
    sample: Box<[f32]>,
    channels: usize,
    sr: usize
) {
    let mut engine = ENGINE.lock().unwrap();
    let leaked_sample = Box::leak(sample);
    engine.add_sample(&name, leaked_sample, channels, sr);
    // engine.update(code);
}

#[wasm_bindgen]
pub fn update(code: String) -> Vec<u8> {
    console_error_panic_hook::set_once();

    let mut res = vec![0; RES_BUFFER_SIZE];
    if let Err(e) = get_engine().update_with_code(&code) {
        write_err_to_buf(e, &mut res);
    }

    res
}

#[wasm_bindgen]
pub fn send_msg(msg: String) { //, result_ptr: *mut u8
    get_engine().send_msg(&msg);
}

#[wasm_bindgen]
pub fn live_coding_mode(io: bool) {
    get_engine().livecoding = io;
}

#[wasm_bindgen]
pub fn set_bpm(bpm: f32) {
    get_engine().set_bpm(bpm);
    // engine.reset();
}

#[wasm_bindgen]
pub fn set_track_amp(amp: f32) {
    get_engine().set_track_amp(amp);
}

#[wasm_bindgen]
pub fn set_sr(sr: f32) {
    get_engine().set_sr(sr as usize);
}

#[wasm_bindgen]
pub fn set_seed(seed: f32) {
    get_engine().set_seed(seed as usize);
}

#[wasm_bindgen]
pub fn reset() {
    get_engine().reset();
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
                pest::error::InputLocation::Span((s, _)) => s
            };
            let (line, col) = match v.line_col {
                pest::error::LineColLocation::Pos(u) => u,
                pest::error::LineColLocation::Span(u, _) => u
            };
            let (positives, negatives) = match &v.variant {
                pest::error::ErrorVariant::ParsingError {
                    positives,
                    negatives,
                } => (positives, negatives),
                _ => panic!("unknown parsing error")
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
