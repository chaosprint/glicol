extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

use std::{sync::Mutex, collections::HashMap, slice::from_raw_parts_mut};

use pest::Parser;
#[derive(Parser)]
#[grammar = "quaver.pest"]
pub struct QParser;

mod engine;
use engine::{QuaverSignal, Event, QuaverLoop};
use engine::instrument::{Sampler, QuaverFunction};
// instrument::Oscillator,
// use engine::effect::LPF;

use dasp::{signal};
use dasp::signal::Signal;

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
    static ref ENGINE: Mutex<engine::Engine> = Mutex::new(engine::Engine::new());
}

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

    let mut samples_dict = HashMap::new();

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
        samples_dict.insert(name, sample_array);
    };

    // read the code from the text editor
    let encoded:&mut [u8] = unsafe { from_raw_parts_mut(arr_ptr, length) };
    let quaver_code = std::str::from_utf8(encoded).unwrap();

    // parse the code
    let lines = QParser::parse(Rule::block, quaver_code)
    .expect("unsuccessful parse")
    .next().unwrap();

    // add function to Engine HashMap Function Chain Vec accordingly
    for line in lines.into_inner() {

        let mut ref_name = "~";
        let mut func_chain = Vec::<Box<dyn Signal<Frame=f64> + 'static + Send>>::new(); // init Chain

        // match line.as_rule() {
        //     Rule::line => {
        let inner_rules = line.into_inner();
        // let mut func_vec = Vec::<Box<dyn QuaverFunction + 'static + Send>>::new();

        for element in inner_rules {
            match element.as_rule() {
                Rule::reference => {
                    ref_name = element.as_str();
                },
                Rule::chain => {
                    for func in element.into_inner() {
                        let mut inner_rules = func.into_inner();
                        let name: &str = inner_rules.next().unwrap().as_str();
                        match name {
                            "sin" => {
                                let mut paras = inner_rules.next().unwrap().into_inner();
                                let freq = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                let sig = signal::rate(48000.0).const_hz(freq).sine();
                                func_chain.push(Box::new(sig));

                            },
                            "mul" => {
                                
                            },
                            "loop" => {
                                let mut q_loop = QuaverLoop::new();

                                let mut paras = inner_rules
                                .next().unwrap().into_inner();

                                let seq = paras.next().unwrap();
                                let mut compound_index = 0;
                                let seq_by_space: Vec<pest::iterators::Pair<Rule>> = 
                                seq.clone().into_inner().collect();

                                for compound in seq.into_inner() {
                                    let mut shift = 0;
            
                                    // calculate the length of seq
                                    let compound_vec: Vec<pest::iterators::Pair<Rule>> = 
                                    compound.clone().into_inner().collect(); 
            
                                    for note in compound.into_inner() {
                                        if note.as_str().parse::<i32>().is_ok() {
                                            let seq_shift = 1.0 / seq_by_space.len() as f64 * 
                                            compound_index as f64;
                                            
                                            let note_shift = 1.0 / compound_vec.len() as f64 *
                                            shift as f64 / seq_by_space.len() as f64;
            
                                            let d = note.as_str().parse::<i32>().unwrap() as f64;
                                            let pitch = 2.0f64.powf((d - 69.0) / 12.0) * 440.0;

                                            let mut event = Event::new();
                                            event.relative_time = seq_shift + note_shift;
                                            event.pitch = pitch;

                                            // better to push a events, right?
                                            q_loop.events.push(event);
                                        }
                                        shift += 1;
                                    }
                                    compound_index += 1;
                                }
                                // 直接推送到大列表中，反正都要用到
                                // func_chain.functions.push(Box::new(q_loop));
                            },
                            "sampler" => {
                                let mut paras = inner_rules.next().unwrap().into_inner();
                                let symbol = paras.next().unwrap().as_str();
                                // sig.ins.push(
                                //     Box::new(
                                //         Sampler::new(samples_dict[symbol].clone())
                                //     )
                                // );
                                // func_chain.functions.push(
                                //     Box::new(Sampler::new(samples_dict[symbol].clone()))
                                // );
                            },
                            "lpf" => {
                            },
                            "lfo" => {
                            },
                            _ => unreachable!()
                        }
                    }
                },
                _ => unreachable!()
            }
        }
        engine.chains.insert(ref_name.to_string(), func_chain); // sig: sig_chain
    };
    // engine.phase = 0;
}