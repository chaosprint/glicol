use dasp_signal::{self as signal, Signal};
use dasp_graph::{Buffer, Input, Node};

pub struct SinOsc {
    // pub freq: f64,
    // pub sig: Sine<ConstHz>
    freq: f32,
    phase: f32,
    diff: f32,
    has_mod: bool
    // pub sig: Box<dyn Signal<Frame=f64> + Send>,
}

impl SinOsc {
    pub fn new(freq: String, phase: f32, diff: f32) -> Self {
        if freq.parse::<f32>().is_ok() {
            return Self { 
                freq: freq.parse::<f32>().unwrap(),
                phase, diff, has_mod: false 
            }
        } else {
            return Self { 
                freq: 0.0,
                phase, diff, has_mod: true 
            }
        }
        
    }
}

impl Node for SinOsc {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        
        // let freq = self.freq.parse::<f32>();
        if self.has_mod {
            if inputs.len() > 0 {
                // panic!();
                    // let buf = &mut inputs[0].buffers();
                let mod_buf = &mut inputs[0].buffers();
                // panic!();
                for i in 0..64 {
                    // output[0][i] = (2.0*std::f32::consts::PI*mod_buf[0][i]/44100.0).sin();
                    output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();

                    if mod_buf[0][i] != 0.0 { // doesn't make sense to have 0 freq
                        self.diff = mod_buf[0][i] / 44100.0;    
                    }
                    self.phase += self.diff;
                    // self.phase += 440.0 / 44100.0;
                    if self.phase > 1.0 {
                        self.phase -= 1.0
                    }
                }
            }
        } else {

            for i in 0..64 {
                output[0][i] = self.phase.sin();
                self.phase += self.freq / 44100.0 * 2.0 * std::f32::consts::PI;
                // self.phase += 220.0 / 44100.0;
                if self.phase > 2.0 * std::f32::consts::PI {
                    self.phase -= 2.0 * std::f32::consts::PI
                }
            }
        }
    }
}

pub struct Impulse {
    sig: Box<dyn Signal<Frame=f32> + Send>,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

impl Impulse {
    pub fn new(freq: f64) -> Self {
        let p = (44100.0 / freq) as usize;
        let mut i: usize = 0;
        let s = signal::gen_mut(move || {
            let imp = (i % p == 0) as u8;
            i += 1;
            imp as f32
        });
        Self {
            sig: Box::new(s)
        }
    }
}

impl Node for Impulse {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for o in output {
            o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
        // output[0].iter_mut().for_each(|s| *s = self.sig.next());
    }
}