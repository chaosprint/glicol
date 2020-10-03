use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend};

pub struct LinRange {
    out_lo: f32,
    out_hi: f32,
    in_lo: f32,
    in_hi: f32
}

impl LinRange {
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {
        
    }
}

// pub struct ExpRange {
//     out_lo: f32,
//     out_hi: f32,
//     in_lo: f32,
//     in_hi: f32
// }