use dasp_graph::{Buffer, Input, Node};
use dasp_slice::add_in_place;
use apodize;
use super::super::{Pairs, Rule, NodeData, 
    NodeResult, BoxedNodeSend, GlicolNodeData, mono_node, Para};

pub struct MonoSum {}

impl MonoSum {
    pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {
        let inputs: Vec<String> = paras.as_str()
        .split(" ").map(|a|a.to_string()).collect();
        // println!("{:?}", inputs);
        Ok(
            (NodeData::new1(
                BoxedNodeSend::new(
                    Self {}
                )
            ), inputs)
        )
    }
}

impl Node<128> for MonoSum {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let n = inputs.len();
        // clock input[n-1]
        output[0].silence();

        for i in 0..(n-1) {
            let in_buffer = inputs[i].buffers().clone();
            add_in_place(&mut output[0], &in_buffer[0]);
            // for i in 0..64 {
                // output[0][i] += in_buffer[0][i];
            // }
        }
    }
}