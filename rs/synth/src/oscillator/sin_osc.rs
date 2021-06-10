use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

/// Sine Wave Oscillator Node Builder
pub struct SinOsc {
    freq: f32,
    phase: f32,
    clock: usize,
    buffer: Buffer<128>,
    sr: usize,
}

impl SinOsc {
    pub fn new() -> Self {
        Self {
            freq: 0.01,
            phase: 0.,
            clock: 0,
            buffer: Buffer::<128>::default(),
            sr: 44100,
        }
    }

    pub fn freq(self, freq: f32) -> Self {
        Self {freq, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData {
        mono_node! ( self )
    }
}


/// The inputs.len() has two possible situation
/// One is using Glicol as a standalone audio lib
/// This will be zero, or any
/// We should find out a way to differ standalone and live coding
/// This is by seeing if the first input is a clock
/// The clock is like [12345, 0, 0, 0, ...] * 128
impl Node<128> for SinOsc {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        
        let min_user_input = 0;
        let l = inputs.len();
        // println!("sin l is {}", l);
        let max_user_input = 1;
        if l < min_user_input {return ()};
        let has_clock = match l {
            0 => false,
            _ => inputs[l-1].buffers()[0][0] % 128. == 0. 
            && inputs[l-1].buffers()[0][1] == 0.
        };
        
        match l {
            0 => {
                for i in 0..128 {
                    output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                    self.phase += self.freq / self.sr as f32;
                    if self.phase > 1.0 {
                        self.phase -= 1.0
                    }
                }
            },
            1 => {
                // in standalone mode, no mechanism to prevent double processing
                // basic fm
                if has_clock {
                    for i in 0..128 {
                        output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                        self.phase += self.freq / self.sr as f32;
                        if self.phase > 1.0 {
                            self.phase -= 1.0
                        }
                    }
                } else {
                    let mod_buf = &mut inputs[0].buffers();
                    for i in 0..128 {
                        output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                        self.phase += mod_buf[0][i] / self.sr as f32;
                        if self.phase > 1.0 {
                            self.phase -= 1.0
                        }
                    }
                }
            },
            2 => {
                // has clock input or somehow mistakenly connected by users
                let clock = inputs[1].buffers()[0][0] as usize;
                // avoid process twice
                // without this, when use this node to control two different things
                // the phase += will be called more than once and cause errors and mess
                if self.clock != 0 && self.clock == clock {
                    output[0] = self.buffer.clone();
                    return ()
                };
                let mod_buf = &mut inputs[0].buffers();
                // println!("{:?}", mod_buf[0]);
                for i in 0..128 {
                    output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                    self.phase += mod_buf[0][i] / self.sr as f32;
                    if self.phase > 1.0 {
                        self.phase -= 1.0
                    }
                }
                self.buffer = output[0].clone();
                self.clock = clock;
            },
            _ => return ()
        }
        // println!("output from sin {:?}", output);
    }
}