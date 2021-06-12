use super::super::{NodeData, GlicolNodeData,
    BoxedNodeSend, mono_node, Buffer, Input, Node};

pub struct EnvPerc {
    attack: f32,
    decay: f32,
    pos: usize,
    scale: f32,
    sr: usize,
    // sidechain_ids: Vec::<u8>
}

impl EnvPerc {
    pub fn new() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            pos: 0,
            scale: 1.0,
            sr: 44100,
        }
    }

    pub fn attack(self, attack: f32) -> Self {
        Self {attack, ..self}
    }
    pub fn decay(self, decay: f32) -> Self {
        Self {decay, ..self}
    }
    pub fn scale(self, scale: f32) -> Self {
        Self {scale, ..self}
    }
    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }
    pub fn build(self) -> GlicolNodeData {
        mono_node!( self )
    }
}

#[macro_export]
macro_rules! envperc {
    ({$($para: ident: $data:expr),*}) => {
         (
            EnvPerc::new()$(.$para($data))*.build()
        )
    }
}

impl Node<128> for EnvPerc {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {

            let attack_len = (self.attack * self.sr as f32) as usize;
            let decay_len = (self.decay * self.sr as f32) as usize;
            let dur = attack_len + decay_len;      
            let buf = &mut inputs[0].buffers();
    
            for i in 0..128 {
                if buf[0][i] > 0.0 {
                    self.pos = 0;
                    self.scale = buf[0][i];
                }
                if self.pos <= attack_len {
                    output[0][i] = self.pos as f32 / attack_len as f32;
                } else if self.pos > attack_len && self.pos <= dur {
                    output[0][i] = (dur - self.pos) as f32 / decay_len as f32;
                } else {
                    output[0][i] = 0.0
                }
                output[0][i] *= self.scale;
                self.pos += 1;
            }
    }
}