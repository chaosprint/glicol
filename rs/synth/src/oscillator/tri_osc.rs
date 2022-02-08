
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct TriOsc<const N: usize> {
    freq: f32,
    clock: usize,
    buffer: Buffer<N>,
    sr: usize,
    phase: f32,
    inc: f32,
}

impl<const N: usize> TriOsc<N> {
    pub fn new() -> Self {
        Self {
            freq: 0.01,
            clock: 0,
            buffer: Buffer::<N>::default(),
            sr: 44100,
            phase: 0.,
            inc: 0.,
        }
    }
    pub fn freq(self, freq: f32) -> Self {
        Self {freq, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node! {
            N,
            self
        }
    }
}

impl<const N: usize> Node<N> for TriOsc<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
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
                for i in 0..N {
                    let v = -1.0 + (self.phase * 2.);

                    output[0][i] = 2.0 * (v.abs() - 0.5);
                    self.phase += self.freq / self.sr as f32;

                    if self.phase > 1. {
                        self.phase -= 1.
                    }
                }
            },
            1 => {
                // in standalone mode, no mechanism to prevent double processing
                // basic fm
                if has_clock {
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
                    for i in 0..N {
                        if mod_buf[0][i] != 0. {
                            self.inc = mod_buf[0][i]
                        };
                        let v = -1.0 + (self.phase * 2.);

                        output[0][i] = 2.0 * (v.abs() - 0.5);
                        self.phase += self.inc / self.sr as f32;

                        if self.phase > 1. {
                            self.phase -= 1.
                        }
                    }
                    self.buffer = output[0].clone();
                    self.clock = clock;
                } else {
                    for i in 0..N {
                        let v = -1.0 + (self.phase * 2.);

                        output[0][i] = 2.0 * (v.abs() - 0.5);
                        self.phase += self.freq  / self.sr as f32;

                        if self.phase > 1. {
                            self.phase -= 1.
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
                for i in 0..N {
                    if mod_buf[0][i] != 0. {
                        self.inc = mod_buf[0][i]
                    };
                    let v = -1.0 + (self.phase * 2.);

                    output[0][i] = 2.0 * (v.abs() - 0.5);
                    self.phase += self.inc / self.sr as f32;

                    if self.phase > 1. {
                        self.phase -= 1.
                    }
                }
                self.buffer = output[0].clone();
                self.clock = clock;

                // let mut clock = inputs[1].buffers()[0][0] as usize;
                // for i in 0..N {
                //     let mod_buf = &mut inputs[0].buffers();
                //     if mod_buf[0][i] != 0.0 {
                //         self.freq = mod_buf[0][i];
                //     };
                //     let mut period = self.sr as f32 / self.freq;
                //     period = period.max(2.0);
                //     output[0][i] = (clock % period as usize) as f32
                //     / period *2.0-1.0;
                //     clock += 1;
                // }
            },
            _ => return ()
        }
    }
}