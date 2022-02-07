
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct SawOsc<const N: usize> {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<N>,
    phase: f32,
    inc: f32,
    sr: usize,
}

impl<const N: usize> SawOsc<N> {
    pub fn new() -> Self {
        Self {
            freq: 0.01,
            phase_n: 0,
            clock: 0,
            phase: 0.0,
            inc: 0.,
            buffer: Buffer::<N>::default(),
            sr: 44100,
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

impl<const N: usize> Node<N> for SawOsc<N> {
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
                    output[0][i] = self.phase / std::f32::consts::PI - 1.;
                    self.phase += self.freq * 2. * std::f32::consts::PI / self.sr as f32;
                    if self.phase > 2. * std::f32::consts::PI {
                        self.phase -= 2. * std::f32::consts::PI
                    }
                }
            },
            1 => {
                // in standalone mode, no mechanism to prevent double processing
                // basic fm
                if has_clock {
                    let mut clock = inputs[1].buffers()[0][0] as usize;
                    if self.clock != 0 && self.clock == clock {
                        output[0] = self.buffer.clone();
                        return ()
                    };
    
                    let mod_buf = &mut inputs[0].buffers();

                    for i in 0..N {
                        output[0][i] = self.phase / std::f32::consts::PI - 1.;
                        if mod_buf[0][i] != 0. {
                            self.inc = mod_buf[0][0]
                        };
                        self.phase +=  self.inc * 2. * std::f32::consts::PI / self.sr as f32;
                        if self.phase > 2. * std::f32::consts::PI {
                            self.phase -= 2. * std::f32::consts::PI
                        }
                    }
                    self.buffer = output[0].clone();
                    self.clock = clock;
                } else {
                    for i in 0..N {
                        output[0][i] = self.phase / std::f32::consts::PI - 1.;
                        self.phase += self.freq * 2. * std::f32::consts::PI / self.sr as f32;
                        if self.phase > 2. * std::f32::consts::PI {
                            self.phase -= 2. * std::f32::consts::PI
                        }
                    }
                }
            },
            2 => {
                // has clock input or somehow mistakenly connected by users
                let mut clock = inputs[1].buffers()[0][0] as usize;
                if self.clock != 0 && self.clock == clock {
                    output[0] = self.buffer.clone();
                    return ()
                };

                let mod_buf = &mut inputs[0].buffers();
                
                for i in 0..N {
                    output[0][i] = self.phase / std::f32::consts::PI - 1.;
                    if mod_buf[0][i] != 0. {
                        self.inc = mod_buf[0][0]
                    };
                    self.phase += self.inc * 2. * std::f32::consts::PI / self.sr as f32; // only count the first input for modulation
                    if self.phase > 2. * std::f32::consts::PI {
                        self.phase -= 2. * std::f32::consts::PI
                    }
                }
                self.buffer = output[0].clone();
                self.clock = clock;
                
                // deprecated...
                // for i in 0..N {
                //     let mod_buf = &mut inputs[0].buffers();
                //     if mod_buf[0][i] != 0.0 {
                //         self.freq = mod_buf[0][i];
                //     };
                //     // let mut period = self.sr as f32 / self.freq; // e.g. 1039.6039604
                //     // period = period.max(2.0); // clip too high freq

                //     // problematic
                //     // output[0][i] = (clock % period as usize) as f32 / period /* this range should be between 0-1 */
                //     // *2.0-1.0;
                //     // output[0][i] = 
                //     clock += 1;
                // }
            },
            _ => return ()
        }
    }
}