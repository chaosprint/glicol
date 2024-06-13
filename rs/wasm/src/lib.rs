use glicol::Engine;
use std::cell::RefCell;
use std::slice::from_raw_parts_mut;

// Use WeeAlloc as the global allocator for the WebAssembly module.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

thread_local! {
    static ENGINE: RefCell<Engine<128>> = RefCell::new(Engine::<128>::new());
}

/// # Safety
///
/// - in_ptr must be aligned and non-null
/// - out_ptr must aligned and non-null
/// - result_ptr must be aligned and nonnull
#[no_mangle]
pub unsafe extern "C" fn process(
    in_ptr: *mut f32,
    out_ptr: *mut f32,
    size: usize,
    result_ptr: *mut u8,
) {
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();

        let _in_buf: &mut [f32] = unsafe { from_raw_parts_mut(in_ptr, 128) };
        let result: &mut [u8] = unsafe { from_raw_parts_mut(result_ptr, 256) };

        let (engine_out, console) = engine.next_block(vec![]);

        let out_buf: &mut [f32] = unsafe { from_raw_parts_mut(out_ptr, size) };

        out_buf[..128].copy_from_slice(&engine_out[0][..128]);
        out_buf[128..].copy_from_slice(&engine_out[1][..128]);
        result[..256].copy_from_slice(&console);
    });
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
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        let encoded: &mut [u8] = unsafe { from_raw_parts_mut(name_ptr, name_len) };
        let name = std::str::from_utf8(encoded).unwrap();
        let sample: &mut [f32] = unsafe { from_raw_parts_mut(arr_ptr, length) };
        engine.add_sample(name, sample, channels, sr);
    });
}

/// # Safety
///
/// - arr_ptr must be aligned and non-null
#[no_mangle]
pub unsafe extern "C" fn update(arr_ptr: *mut u8, length: usize) {
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        let encoded: &mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
        let code = std::str::from_utf8(encoded).unwrap();
        engine.update_with_code(code);
    });
}

/// # Safety
///
/// - arr_ptr must be aligned and non-null
#[no_mangle]
pub unsafe extern "C" fn send_msg(arr_ptr: *mut u8, length: usize) {
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        let encoded: &mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
        let msg = std::str::from_utf8(encoded).unwrap();
        engine.send_msg(msg);
    });
}

#[no_mangle]
pub extern "C" fn live_coding_mode(io: bool) {
    ENGINE.with(|engine| {
        engine.borrow_mut().livecoding = io;
    });
}

#[no_mangle]
pub extern "C" fn set_bpm(bpm: f32) {
    ENGINE.with(|engine| {
        engine.borrow_mut().set_bpm(bpm);
    });
}

#[no_mangle]
pub extern "C" fn set_track_amp(amp: f32) {
    ENGINE.with(|engine| {
        engine.borrow_mut().set_track_amp(amp);
    });
}

#[no_mangle]
pub extern "C" fn set_sr(sr: f32) {
    ENGINE.with(|engine| {
        engine.borrow_mut().set_sr(sr as usize);
    });
}

#[no_mangle]
pub extern "C" fn set_seed(seed: f32) {
    ENGINE.with(|engine| {
        engine.borrow_mut().set_seed(seed as usize);
    });
}

#[no_mangle]
pub extern "C" fn reset() {
    ENGINE.with(|engine| {
        engine.borrow_mut().reset();
    });
}
