
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct Phasor<const N: usize> {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<N>,
    sr: usize,
}

impl<const N: usize> Phasor<N> {
    pub fn new() -> Self {
        Self {
            freq: 0.01,
            phase_n: 0,
            clock: 0,
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

impl<const N: usize> Node<N> for Phasor<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let l = inputs.len();
        let has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0 && inputs[l-1].buffers()[0][1] == 0.;
        match l {
            0 => {
                for i in 0..N {
                    let period = (self.sr as f32 / self.freq) as usize;
                    let out = self.phase_n % period;
                    output[0][i] = out as f32 / period as f32;
                    self.phase_n += 1;

                }
            },
            1 => {
                if has_clock {
                    for i in 0..N {
                        let period = (self.sr as f32 / self.freq) as usize;
                        let out = self.phase_n % period;
                        output[0][i] = out as f32 / period as f32;
                        self.phase_n += 1;
    
                    }
                } else {
                    for i in 0..N {
                        let mod_buf = &mut inputs[0].buffers();
                        if mod_buf[0][i] != 0.0 {
                            self.freq = mod_buf[0][i];
                        };
                        let period = (self.sr as f32 / self.freq) as usize;
                        let out = self.phase_n % period;
                        output[0][i] = out as f32 / period as f32;
                        self.phase_n += 1;
                    }
                }
            },
            2 => {
                let mut clock = inputs[1].buffers()[0][0] as usize;
                for i in 0..N {
                    let mod_buf = &mut inputs[0].buffers();
                    if mod_buf[0][i] != 0.0 {
                        self.freq = mod_buf[0][i];
                    };
                    let period = (self.sr as f32 / self.freq) as usize;
                    let out = clock % period;
                    output[0][i] = out as f32 / period as f32;
                    clock += 1;
                }
            },
            _ => ()
        }
    }
}