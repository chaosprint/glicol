// use dasp_signal::{self as signal, Signal};
use dasp_graph::{Buffer, Input, Node};

// pub enum Env {
//     Perc(f64, f64, u32),
//     // Adsr(f64, f64, f64, f64),
//     // List(Vec::<f64>)
// }

pub struct EnvPerc {
    attack: f64,
    decay: f64,
    pos: u32,
    scale: f32,
}

impl EnvPerc {
    pub fn new(attack: f64, decay: f64, pos: u32, scale: f32) -> Self {
        Self {
            attack,
            decay,
            pos,
            scale
        }
    }
}

impl Node for EnvPerc {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        
        if inputs.len() > 0 {
            // all reset in new process
            let attack_len = (self.attack * 44100.0) as u32;
            // assert!(self.attack>0.009);
            // assert_eq!(attack_len, 441);
            let decay_len = (self.decay * 44100.0) as u32;
            // assert!(self.decay>0.09);
            // assert_eq!(attack_len, 4410);
            let dur = attack_len + decay_len;      
            let buf = &mut inputs[0].buffers();
            // let mut imp: f32 = 0.0;

            // assert_eq!(self.decay, 1.0);
            // experiment
            // if self.pos <= attack_len {
            //     output[0].iter_mut().for_each(|s| *s = self.pos as f32 / attack_len as f32)
            // } else if self.pos > attack_len && self.pos <= dur {
            //     output[0].iter_mut().for_each(|s| *s =  (dur - self.pos) as f32 / decay_len as f32)
            // } else {
            //     output[0].iter_mut().for_each(|s| *s = 0.0)
            // };
            // self.pos += 64;
            // self.pos = self.pos % 88200;
            // end of experiment

            for i in 0..64 {
                // output[0][i] = buf[0][i];

                if buf[0][i] > 0.0 {
                    self.pos = 0;
                    self.scale = buf[0][i];
                }

                if self.pos <= attack_len {
                    output[0][i] = self.pos as f32 / attack_len as f32;
                } else if self.pos > attack_len && self.pos <= dur {
                    output[0][i] = (dur - self.pos) as f32 / decay_len as f32;
                    // output[0][i] = 1.0 - ((self.pos - attack_len) as f32 / decay_len as f32);
                } else {
                    output[0][i] = 0.0
                }
                output[0][i] *= self.scale;

                // if self.pos < 441 {
                //     output[0][i] = self.pos as f32 / 441.0 * buf[0][i];
                // } else if self.pos < 44541 {
                //     output[0][i] = (1.0 - (self.pos - 441) as f32 / 48000.0) * buf[0][i];
                // } else {
                //     output[0][i] = 0.0
                // }
               
                self.pos += 1;
                // self.pos = self.pos % 441000;
            }
        }
    }            // Env::List(v) => {}
}

// env!()

// pub struct Adsr {
//     pub adsr: (f64, f64, f64, f64)
// }

// impl Adsr {
//     pub fn new( adsr: (f64, f64, f64, f64)) -> Self {
//         Self { adsr }
//     }
// };
