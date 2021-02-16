use dasp_graph::{Buffer, Input, Node};
use super::super::{NodeData, BoxedNodeSend, NodeResult};

pub struct Pass {}
impl Pass {
    pub fn new(name: &str) -> NodeResult {

        // let mut paras = paras.next().unwrap().into_inner();
        let destination: String = name.to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();
        // println!("destination {}", name);
        Ok((NodeData::new1(BoxedNodeSend::new( Self {
        })), vec![destination]))
    }
}

impl Node<128> for Pass {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // println!("{}", inputs.len());
        output[0] = inputs[0].buffers()[0].clone();
    }
}