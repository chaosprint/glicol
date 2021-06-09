use dasp_graph::{Buffer, Input, Node};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};

pub struct Plate {
    mix: f32,
    predelay: f32
    // sr: usize,
}

impl Plate {
    pub fn new() -> Self {
        Self {
            mix: 0.1,
            predelay: 0.3,
        }
    }

    pub fn build(self) -> GlicolNodeData {
        let wet = chain!([]);
        let modulator = chain![sin_osc!{freq: 0.1}, mul!(5.5), add!(29.5) in g];





        // graph!{
        //     out: [sin_osc!{freq: 440.}, mul(_side)],
        //     _side: [sin_osc!{freq: 10.1}, mul(0.3), add!(0.5)],
        // }
        // edges![(out[1], )]
        mono_node!(self)
    }
}

impl Node<128> for Plate {
    fn process(&mut self, inputs: &[Input<128>], &mut outputs: &[Buffer<128>]) {
    }
}