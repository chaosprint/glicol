use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use super::super::{NodeData, BoxedNodeSend, mono_node, GlicolNodeData};

type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct DelayN {
    buf: Fixed
}

impl DelayN {
    pub fn new(n: usize) -> GlicolNodeData {
        mono_node!( Self {buf: ring_buffer::Fixed::from(vec![0.0; n])} )
    }
}

#[macro_export]
macro_rules! delayn {
    ($data: expr) => {
        DelayN::new($data);
    };
}

impl Node<128> for DelayN {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        for i in 0..128 {
            output[0][i] = self.buf[0];
            self.buf.push(inputs[0].buffers()[0][i]);
        }
        println!("{:?} {:?} self.buf{:?}",inputs[0].buffers()[0], output[0], self.buf);
    }
}