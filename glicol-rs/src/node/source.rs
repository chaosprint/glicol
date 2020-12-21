use dasp_graph::{Buffer, Input, Node};
use dasp_slice::add_in_place;
use super::super::{Pairs, Rule, NodeData, 
    NodeResult, BoxedNodeSend, EngineError, handle_params};

pub struct ConstSig {
    val: f32
}

impl ConstSig {
    pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {
        let val = match paras.as_str().parse::<f32>() {
            Ok(v) => v,
            Err(_) => {return Err(EngineError::ParameterError)}
        };
        Ok(
            (NodeData::new1(
                BoxedNodeSend::new(
                    Self {
                        val
                    }
                )
            ), vec![])
        )
    }
}

impl Node for ConstSig {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        for o in output {
            o.iter_mut().for_each(|s| *s = self.val);
        }
    }
}