use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct Mul {
    pub mul: f32,
    has_mod: bool
}
impl Mul {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        // let mut paras = paras.next().unwrap().into_inner();
        // println!("{:?}", paras.as_str());
        let mul: String = paras.as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let is_float = mul.parse::<f32>();
        if is_float.is_ok() {
            Ok((NodeData::new1(BoxedNodeSend::new(Self {mul: is_float.unwrap(), has_mod: false})),
            vec![]))
        } else {
            Ok((NodeData::new1(BoxedNodeSend::new(Self {mul: 0.0, has_mod: true})),
            vec![mul]))
        }
    }
}
impl Node for Mul {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if !self.has_mod {
            // if inputs.len() > 0 {
            assert_eq!(inputs.len(), 1);
            // let buf = &mut inputs[0].buffers();
            // output[0] = buf[0].clone();
            output[0] = inputs[0].buffers()[0].clone();
            output[0].iter_mut().for_each(|s| *s = *s * self.mul as f32);
            // }
        } else {
            // if inputs.len() > 1 {
            assert!(inputs.len() > 1);
            let buf = &mut inputs[0].buffers();
            let mod_buf = &mut inputs[1].buffers();
            for i in 0..64 {
                output[0][i] = mod_buf[0][i] * buf[0][i];
            }
            // }
        }
    }
}

pub struct Add {
    pub inc: f32,
    has_mod: bool
}
impl Add {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError>  {
        let inc: String = paras.as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let is_float = inc.parse::<f32>();

        if is_float.is_ok() {
            Ok((NodeData::new1(BoxedNodeSend::new(Self {inc: is_float.unwrap(), has_mod: false})),
            vec![]))
        } else {
            Ok((NodeData::new1(BoxedNodeSend::new(Self {inc: 0.0, has_mod: true})),
            vec![inc]))
        }
    }
}
impl Node for Add {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if self.has_mod {
            assert!(inputs.len() > 1);
            let buf = &mut inputs[0].buffers();
            let mod_buf = &mut inputs[1].buffers();
            for i in 0..64 {
                output[0][i] = mod_buf[0][i] + buf[0][i];
            }
        } else {
            assert_eq!(inputs.len(), 1);
            output[0] = inputs[0].buffers()[0].clone();
            output[0].iter_mut().for_each(|s| *s = *s + self.inc as f32);
        }
        // if inputs.len() > 0 {
    }
}