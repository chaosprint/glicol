extern crate dasp;
use dasp::slice::ToFrameSlice;
use dasp::interpolate::linear::Linear;
use dasp::signal::{self as signal, Signal, FromIterator, interpolate::Converter};
use std::{iter::Cloned, slice::Iter};

pub struct QuaverEvent {
    pitch: f64,
    time: f64,
    sig: Converter<FromIterator<Cloned<Iter<'static, [f32; 1]>>>, Linear<[f32; 1]>>
}

pub trait QuaverFunction {
    fn run(self, sig: QuaverEvent) -> QuaverEvent;
}

pub trait Instrument {
    fn new_sig(&self, pitch: f64) -> Converter<FromIterator<Cloned<Iter<'static, [f32; 1]>>>, Linear<[f32; 1]>>;
    // fn set_pitch(&mut self, pitch: f32);
    // fn yield_current_sample(&mut self, phase: usize) -> f32;
    // fn get_pitch(&self) -> f32;
    fn get_dur(&self, pitch: f64) -> usize;
    // fn renew_sig(&mut self);
    // fn box_clone(&self) -> Box<dyn Instrument + 'static + Send>;
    // fn clone(&self) -> Self where Self: Sized;
}

#[derive(Clone)]
pub struct Sampler {
    sample_array: &'static[f32],
}

impl Sampler {
    pub fn new(sample_array: &'static[f32]) -> Sampler {
        Sampler {
            sample_array
        }
    }
}

// impl QuaverFunction for Sampler {
//     fn run(self, x: QuaverSignal) -> QuaverSignal {

//     }
// }

// impl Instrument for Sampler {

//     fn new_sig(&self, pitch: f64) -> 
//     Converter<FromIterator<Cloned<Iter<'static, [f32; 1]>>>, Linear<[f32; 1]>> {
//         let f: &[[f32;1]] = self.sample_array.to_frame_slice().unwrap();
//         let mut source = signal::from_iter(f.iter().cloned());
//         let a = source.next();
//         let b = source.next();
//         let interp = Linear::new(a, b);
//         // let interp = Linear::from_source(&mut source);
//         let sig = source.scale_hz(interp, pitch / 261.6255653005986);
//         sig
//     }
//     fn get_dur(&self, pitch: f64) -> usize {
//         (self.sample_array.len() as f64 / (pitch / 261.6255653005986)) as usize
//     }
// }
    // fn box_clone(&self) -> Box<dyn Instrument + 'static + Send> {
    //     Box::new(self.clone())
    // }

    // fn get_pitch(&self) -> f32 {
    //     self.pitch
    // }

    // fn set_pitch(&mut self, pitch: f32) {
    //     self.pitch = pitch;
    //     self.renew_sig();
    // }

    // fn yield_current_sample(&mut self, pha: usize) -> f32 {
    //     self.sig.next()[0]
    // }

    // fn renew_sig(&mut self) {
    //     let f: &[[f32;1]] = self.sample_array.to_frame_slice().unwrap();
    //     let mut source = signal::from_iter(f.iter().cloned());
    //     let interp = Linear::from_source(&mut source);
    //     let sig = source.scale_hz(interp, self.pitch as f64 / 261.63);
    //     self.sig = sig;
    // }
// }


// impl Clone for Box<dyn Instrument + 'static + Send> {
//     fn clone(&self) -> Box<dyn Instrument + 'static + Send> {
//         self.box_clone()
//     }
// }

// #[derive(Clone)]
// pub struct Oscillator {
//     waveshape: u8,
//     init_phase: usize,
//     pub freq: f32,
//     mul: f32,
//     add: f32,
//     // sig: FromIterator<Cloned<Iter<'static, [f32; 1]>>>,
// }

// impl Oscillator {
//     pub fn new(waveshape: u8, init_phase: usize, freq: f32, mul: f32, add: f32) -> Oscillator {
//         // let sig = signal::gen(||[0.0 as f32]);
//         Oscillator {
//             waveshape,
//             init_phase,
//             freq,
//             mul,
//             add,
//             // sig: sig
//         }
//     }
// }

// impl Instrument for Oscillator {

//     fn box_clone(&self) -> Box<dyn Instrument + 'static + Send> {
//         Box::new(self.clone())
//     }

//     fn set_pitch(&mut self, pitch: f32) {
//         self.freq = pitch
//     }

//     fn yield_current_sample(&mut self, phase: usize) -> f32 {
//         use std::f32::consts::PI;
//         let time = phase as f32 / 44100.0;
//         match self.waveshape {
//             0 => (2.0 * PI * time * self.freq).sin() * self.mul + self.add,
//             1 => {
//                 let mut o:f32 = 0.0;
//                 for i in 1..50 {
//                     o += (2.0 * PI * time * self.freq * i as f32).sin() / i as f32
//                 };
//                 o * self.mul + self.add
//             },
//             _ => 0.0
//         }
//     }
//     fn get_dur(&self) -> usize {
//         0
//     }
// }