
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct SquOsc<const N: usize> {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<N>,
    sr: usize,
    phase: f32,
}

impl<const N: usize> SquOsc<N> {
    pub fn new() -> Self {
        Self {
            freq: 440.,
            phase_n: 0,
            clock: 0,
            buffer: Buffer::<N>::default(),
            sr: 44100,
            phase: 0.,
        }
    }
    pub fn freq(self, freq: f32) -> Self {
        Self {freq, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node! { N, self }
    }
}

impl<const N: usize> Node<N> for SquOsc<N> {
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
                    if (self.phase <= std::f32::consts::PI) {
                        output[0][i] = 1.0;
                    } else {
                        output[0][i] = -1.0;
                    }

                    self.phase += self.freq / self.sr as f32;
                    if self.phase > 2. * std::f32::consts::PI {
                        self.phase -= 2. * std::f32::consts::PI
                    }
                }
            },
            1 => {
                // in standalone mode, no mechanism to prevent double processing
                // basic fm
                if has_clock {
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
                        if (self.phase <= std::f32::consts::PI) {
                            output[0][i] = 1.0;
                        } else {
                            output[0][i] = -1.0;
                        }

                        self.phase += mod_buf[0][i] / self.sr as f32;
                        if self.phase > 2. * std::f32::consts::PI {
                            self.phase -= 2. * std::f32::consts::PI
                        }
                    }
                    self.buffer = output[0].clone();
                    self.clock = clock;
                } else {
                    for i in 0..N {
                        if (self.phase <= std::f32::consts::PI) {
                            output[0][i] = 1.0;
                        } else {
                            output[0][i] = -1.0;
                        }

                        self.phase +=self.freq / self.sr as f32;
                        if self.phase > 2. * std::f32::consts::PI {
                            self.phase -= 2. * std::f32::consts::PI
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
                    if (self.phase <= std::f32::consts::PI) {
                        output[0][i] = 1.0;
                    } else {
                        output[0][i] = -1.0;
                    }

                    self.phase += mod_buf[0][i] / self.sr as f32;
                    if self.phase > 2. * std::f32::consts::PI {
                        self.phase -= 2. * std::f32::consts::PI
                    }
                }
                self.buffer = output[0].clone();
                self.clock = clock;
            },
            _ => return ()
        }
    }
}