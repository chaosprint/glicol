use dasp_graph::{Buffer, Input, Node};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, stereo_node};

pub struct Balance {
    balance: f32
}
impl Balance {
    pub fn new(balance: f32) -> GlicolNodeData {
        stereo_node!( Self { balance } )
    }
}

impl Node<128> for Balance {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // let _clock = inputs[2].clone();
        let left = inputs[1].buffers()[0].clone();
        let right = inputs[0].buffers()[0].clone();
        output[0] = left;
        output[1] = right;
        output[0].iter_mut().for_each(|s| *s = *s * (1.0 - self.balance));
        output[1].iter_mut().for_each(|s| *s = *s * self.balance);
    }
}

#[macro_export]
macro_rules! balance {
    () => { // controlled by modulator, no need for value
        Balance::new(0.5)
    };

    ($data: expr) => {
        Balance::new($data)
    };
}