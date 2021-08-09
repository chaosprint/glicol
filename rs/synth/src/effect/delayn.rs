use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use super::super::{NodeData, BoxedNodeSend, mono_node, GlicolNodeData};

type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct DelayN<const N:usize> {
    buf: Fixed
}

impl<const N:usize> DelayN<N> {
    pub fn new(n: usize) -> GlicolNodeData<N> {
        mono_node!( N, Self {buf: ring_buffer::Fixed::from(vec![0.0; n])} )
    }
}

#[macro_export]
macro_rules! delayn {
    ($data: expr) => {
        DelayN::new($data);
    };
}

impl<const N:usize> Node<N> for DelayN<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.buf[0];
            self.buf.push(inputs[0].buffers()[0][i]);
        }
        // println!("{:?} {:?} self.buf{:?}",inputs[0].buffers()[0], output[0], self.buf);
    }
}