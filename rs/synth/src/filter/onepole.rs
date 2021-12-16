use dasp_graph::{Buffer, Input, Node};
use super::super::{NodeData, BoxedNodeSend, mono_node, GlicolNodeData};

pub struct OnePole<const N: usize> {
    a: f32,
    b: f32,
    y1: f32
}

impl<const N: usize> OnePole<N> {
    pub fn new(rate: f32) -> GlicolNodeData<N> {
        let b = (-2.0 * std::f32::consts::PI * rate).exp();
        let a = 1.0 - b;
        mono_node!( N, Self{a, b, y1: 0.0})
    }
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
                self.b = (-2.0 * std::f32::consts::PI * modulator[i]).exp();
                self.a = 1. - self.b;
                let y = input_sig[i] * self.a + self.b * self.y1;
                output[0][i] = y;
                self.y1 = y;
            }
        } else {
            let input_sig = inputs[0].buffers()[0].clone();
            for i in 0..N {
                let y = input_sig[i] * self.a + self.b * self.y1;
                output[0][i] = y;
                self.y1 = y;
            }
        }
    }
}