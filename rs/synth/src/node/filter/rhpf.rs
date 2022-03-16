use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct ResonantHighPassFilter {
    cutoff: f32,
    q: f32,
    x0: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    sr: usize,
    input_order: Vec<usize>,
}

impl ResonantHighPassFilter {
    pub fn new() -> Self {
        Self {
            cutoff: 20.,
            q: 1.0,
            x0: 0.,
            x1: 0.,
            x2: 0.,
            y1: 0.,
            y2: 0.,
            sr: 44100,
            input_order: vec![]
        }
    }
    pub fn cutoff(self, cutoff: f32) -> Self {
        Self {cutoff, ..self}
    }

    pub fn q(self, q: f32) -> Self {
        Self {q, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    impl_to_boxed_nodedata!();
}



impl<const N: usize> Node<N> for ResonantHighPassFilter {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("\n\ninputs[1] \n\n {:?}\n\n", inputs[1].buffers());
        match inputs.len() {
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                let theta_c = 2.0 * std::f32::consts::PI * self.cutoff / self.sr as f32;
                let d = 1.0 / self.q;
                let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
                let gama = (0.5 + beta) * theta_c.cos();
                let a0 = (0.5 + beta + gama) / 2.0;
                let a1 = -0.5 - beta - gama;
                let a2 = (0.5 + beta + gama) / 2.0;
                let b1 = -2.0 * gama;
                let b2 = 2.0 * beta;
                for i in 0..N {
                    let x0 = main_input.buffers()[0][i];
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            },
            2 => {
                let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id
                
                let theta_c = 2.0 * std::f32::consts::PI * ref_input.buffers()[0][0] / self.sr as f32;
                let d = 1.0 / self.q;
                let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
                let gama = (0.5 + beta) * theta_c.cos();
                let a0 = (0.5 + beta + gama) / 2.0;
                let a1 = -0.5 - beta - gama;
                let a2 = (0.5 + beta + gama) / 2.0;
                let b1 = -2.0 * gama;
                let b2 = 2.0 * beta;
    
                for i in 0..N {
                    let x0 = main_input.buffers()[0][i];
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            },
            _ => {return ()}
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {self.cutoff = value},
                    1 => {self.q = value},
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
        
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}