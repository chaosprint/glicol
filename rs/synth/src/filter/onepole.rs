use dasp_graph::{Buffer, Input, Node};
use super::super::{NodeData, BoxedNodeSend, mono_node, GlicolNodeData};

pub struct OnePole<const N: usize> {
    a: f32,
    y1: f32
}

impl<const N: usize> OnePole<N> {
    pub fn new(a: f32) -> GlicolNodeData<N> {
        mono_node!( N, Self{a, y1: 0.0})
    }
}

#[macro_export]
macro_rules! onepole {
    ($data: expr) => {
        OnePole::new($data);
    };
}

impl<const N: usize> Node<N> for OnePole<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {

        let l = inputs.len();
        if l < 1 { return ()};
        let has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0 && inputs[l-1].buffers()[0][1] == 0.;

        if l - has_clock as usize > 1 { // has mod
            let modulator = inputs[0].buffers()[0].clone();
            let input_sig = inputs[1].buffers()[0].clone();
            for i in 0..N {
                let y = input_sig[i] + modulator[i] * self.y1;
                output[0][i] = y;
                self.y1 = y;
            }
        } else {
            let input_sig = inputs[0].buffers()[0].clone();
            for i in 0..N {
                let y = input_sig[i] + self.a * self.y1;
                output[0][i] = y;
                self.y1 = y;
            }
        }
    }
}