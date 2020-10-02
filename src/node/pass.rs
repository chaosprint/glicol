use dasp_graph::{Buffer, Input, Node};
use super::super::{NodeData, BoxedNodeSend};

pub struct Pass {}
impl Pass {
    pub fn new(name: &str) -> (NodeData<BoxedNodeSend>, Vec<String>) {

        // let mut paras = paras.next().unwrap().into_inner();
        let destination: String = name.to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();
        // println!("destination {}", name);
        // let sig = signal::noise(0);
        (NodeData::new1(BoxedNodeSend::new( Self {
        })), vec![destination])
    }
}

impl Node for Pass {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        output[0] = inputs[0].buffers()[0].clone();
    }
}