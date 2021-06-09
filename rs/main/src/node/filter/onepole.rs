use dasp_graph::{Buffer, Input, Node};
use super::super::super::{NodeData, BoxedNodeSend, mono_node, GlicolNodeData};

pub struct OnePole {
    a: f32,
    y1: f32
}

impl OnePole {
    pub fn new(a: f32) -> GlicolNodeData {
        mono_node!(Self{a, y1: 0.0})
    }
}

#[macro_export]
macro_rules! onepole {
    ($data: expr) => {
        OnePole::new($data);
    };
}

impl Node<128> for OnePole {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {

        let l = inputs.len();
        if l < 1 { return ()};
        let has_clock = inputs[l-1].buffers()[0][0] % 128. == 0. && inputs[l-1].buffers()[0][1] == 0.;

        if l - has_clock as usize > 1 { // has mod
            let modulator = inputs[0].buffers()[0].clone();
            let input_sig = inputs[1].buffers()[0].clone();
            for i in 0..128 {
                let y = input_sig[i] + modulator[i] * self.y1;
                output[0][i] = y;
                self.y1 = y;
            }
        } else {
            let input_sig = inputs[0].buffers()[0].clone();
            for i in 0..128 {
                let y = input_sig[i] + self.a * self.y1;
                output[0][i] = y;
                self.y1 = y;
            }
        }
    }
}