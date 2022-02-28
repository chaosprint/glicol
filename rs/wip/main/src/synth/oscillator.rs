use glicol_synth::{Buffer, Input, Node, NodeData, BoxedNodeSend};

#[derive(Debug, Copy, Clone)]
pub struct SinOsc<const N: usize> {
    freq: f32,
    phase: f32,
    sr: usize,
}

impl<const N: usize> SinOsc<N> {
    pub fn new() -> Self {
        Self {
            freq: 1.0,
            phase: 0.,
            sr: 44100,
        }
    }

    pub fn freq(self, freq: f32) -> Self {
        Self {freq, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( self ) )
    }
}

impl<const N: usize> Node<N> for SinOsc<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        match inputs.len() {

            0 => {
                for i in 0..N {
                    output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                    self.phase += self.freq / self.sr as f32;
                    if self.phase > 1.0 {
                        self.phase -= 1.0
                    }
                }
            },
            1 => {
                    let mod_buf = &mut inputs[0].buffers();
                    for i in 0..N {
                        output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                        self.phase += mod_buf[0][i] / self.sr as f32;
                        if self.phase > 1.0 {
                            self.phase -= 1.0
                        }
                    }
            }
            _ => return ()
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 && info.1.parse::<f32>().is_ok() {
            self.freq = info.1.parse::<f32>().unwrap();
        }
    }
}