use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

// pub enum Env {
//     Perc(f64, f64, u32),
//     // Adsr(f64, f64, f64, f64),
//     // List(Vec::<f64>)
// }

pub struct EnvPerc {
    attack: f32,
    decay: f32,
    pos: u32,
    scale: f32,
}

impl EnvPerc {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<
    (NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        let para_a: String = paras.next().unwrap().as_str().to_string();
        // .chars().filter(|c| !c.is_whitespace()).collect();

        let para_b: String = paras.next().unwrap().as_str().to_string();
        // .chars().filter(|c| !c.is_whitespace()).collect();

        let attack = para_a.parse::<f32>()?;
        let decay = para_b.parse::<f32>()?;

        Ok((NodeData::new1( BoxedNodeSend::new( Self {
            attack: attack,
            decay: decay,
            pos: 0,
            scale: 1.0
        })), vec![]))
    }
}

impl Node for EnvPerc {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        let attack_len = (self.attack * 44100.0) as u32;
        let decay_len = (self.decay * 44100.0) as u32;
        let dur = attack_len + decay_len;      
        let buf = &mut inputs[0].buffers();

        for i in 0..64 {
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