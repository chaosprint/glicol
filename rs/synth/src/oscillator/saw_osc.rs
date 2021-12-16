
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct SawOsc<const N: usize> {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<N>,
    sr: usize,
}

impl<const N: usize> SawOsc<N> {
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
                    let period = self.sr as f32 / self.freq;
                    output[0][i] = ( self.phase_n % period as usize) as f32
                    / period *2.0-1.0;
                    self.phase_n += 1;
                }
            },
            1 => {
                // in standalone mode, no mechanism to prevent double processing
                // basic fm
                if has_clock {
                    let mut clock = inputs[1].buffers()[0][0] as usize;
                    let period = self.sr as f32 / self.freq;
                    for i in 0..N {
                        output[0][i] = (clock % period as usize) as f32
                        / period *2.0-1.0;
                        clock += 1;
                    }
                } else {
                    for i in 0..N {
                        let period = self.sr as f32 / self.freq;
                        output[0][i] = ( self.phase_n % period as usize) as f32
                        / period *2.0-1.0;
                        self.phase_n += 1;
                    }
                }
            },
            2 => {
                // has clock input or somehow mistakenly connected by users
                let mut clock = inputs[1].buffers()[0][0] as usize;
                for i in 0..N {
                    let mod_buf = &mut inputs[0].buffers();
                    if mod_buf[0][i] != 0.0 {
                        self.freq = mod_buf[0][i];
                    };
                    let period = self.sr as f32 / self.freq;
                    output[0][i] = (clock % period as usize) as f32
                    / period *2.0-1.0;
                    clock += 1;
                }
            },
            _ => return ()
        }
    }
}