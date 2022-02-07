
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct TriOsc<const N: usize> {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<N>,
    sr: usize,
}

impl<const N: usize> TriOsc<N> {
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
                    let t = self.sr as f32 / self.freq;
                    let phase = self.phase_n as f32 / (t/4.);
                    let y = match phase.floor() as u8 {
                        0 => phase.fract(),
                        1 => 1.0 - phase.fract(),
                        2 => - phase.fract(),
                        3 => phase.fract() - 1.,
                        _ => 0.0
                    };
                    // let period = self.sr as f32 / self.freq;
                    // let quater_period = (period / 4.) as usize;
                    // let half_period = (period / 2.) as usize;
                    // let y_abs = (self.phase_n % quater_period) as f32 / quater_period as f32;
                    // let is_quater = ((self.phase_n % half_period) >= quater_period) as u8 as f32;// 1.0 or 0
                    // let is_half =  (self.phase_n >= half_period ) as u8 as f32;// 1.0 or 0
                    // let y = (y_abs * (is_quater * -2. + 1.) + (1. * is_quater)) * (is_half * -2. + 1.);
                    output[0][i] = y;
                    self.phase_n += 1;
                    if self.phase_n >= t as usize {
                        self.phase_n -= t as usize;
                    }
                }
            },
            1 => {
                // in standalone mode, no mechanism to prevent double processing
                // basic fm
                if has_clock {
                    let mut clock = inputs[1].buffers()[0][0] as usize;
                    let mut period = self.sr as f32 / self.freq;
                    period = period.max(2.0);
                    for i in 0..N {
                        output[0][i] = (clock % period as usize) as f32
                        / period *2.0-1.0;
                        clock += 1;
                    }
                } else {
                    for i in 0..N {
                        let mod_buf = &mut inputs[0].buffers();
                        if mod_buf[0][i] != 0.0 {
                            self.freq = mod_buf[0][i];
                        };
                        let t = self.sr as f32 / self.freq;
                        let phase = self.phase_n as f32 / (t/4.);
                        let y = match phase.floor() as u8 {
                            0 => phase.fract(),
                            1 => 1.0 - phase.fract(),
                            2 => - phase.fract(),
                            3 => phase.fract() - 1.,
                            _ => 0.0
                        };
                        output[0][i] = y;
                        self.phase_n += 1;
                        if self.phase_n >= t as usize {
                            self.phase_n -= t as usize;
                        }
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
                    let mut period = self.sr as f32 / self.freq;
                    period = period.max(2.0);
                    output[0][i] = (clock % period as usize) as f32
                    / period *2.0-1.0;
                    clock += 1;
                }
            },
            _ => return ()
        }
    }
}