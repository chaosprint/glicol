use dasp_graph::{Buffer, Input, Node};
use super::super::{HashMap, Pairs, Rule, NodeData, 
    NodeResult, BoxedNodeSend, EngineError};

pub struct Buf {
    // pub sig: Vec< Box<dyn Signal<Frame=[f32;1]> + 'static + Send>>,
    // pub sig: Box<dyn Signal<Frame=[f32;1]> + Send>,
    pub sample: &'static[f32],
}

impl Buf {
    pub fn new(
        paras: &mut Pairs<Rule>,
        samples_dict: &HashMap<String, &'static[f32]>,
    ) -> NodeResult {
        let p = paras.next().unwrap();
        let pos = (p.as_span().start(), p.as_span().end());
        let key = p.as_str();
        if !samples_dict.contains_key(key) {
            return Err(EngineError::SampleNotExistError(pos))
        }
        let sample = samples_dict[key];
        Ok((NodeData::new1(BoxedNodeSend::new(Self{
            sample
        })), vec![]))
    }
}

impl Node<128> for Buf {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // output[0].silence();
        // if inputs.len() > 0 {
        // each input value is between 0-1
        let input_buf = &mut inputs[0].buffers();
        let len = self.sample.len() - 1;

        for i in 0..128 {
            let index = input_buf[0][i] * len as f32;
            output[0][i] = match index {
                x if x == 0.0 => self.sample[0],
                x if x == len as f32 => self.sample[len],
                x if x > 0.0 && x < len as f32 => {
                    let left = x.floor();
                    let right = x.ceil();
                    self.sample[left as usize] * (x - left)
                    + self.sample[right as usize] * (right - x)
                },
                _ => 0.0
            };
        }
    }
}