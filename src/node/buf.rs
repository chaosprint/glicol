use dasp_graph::{Buffer, Input, Node};
use super::super::{HashMap, Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct Buf {
    // pub sig: Vec< Box<dyn Signal<Frame=[f32;1]> + 'static + Send>>,
    // pub sig: Box<dyn Signal<Frame=[f32;1]> + Send>,
    pub sample: &'static[f32],
}
// samples: &'static[f32]
impl Buf {
    pub fn new(
        paras: &mut Pairs<Rule>,
        samples_dict: &HashMap<String, &'static[f32]>,
    ) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        // let mut paras = paras.next().unwrap().into_inner();
        // let para_a: String = paras.next().unwrap().as_str().to_string()
        // .chars().filter(|c| !c.is_whitespace()).collect();

        let key = paras.as_str();

        let sample = samples_dict[key];

        Ok((NodeData::new1(BoxedNodeSend::new(Self{
            // sig: Vec::new(),
            sample
        })), vec![]))
    }
}

impl Node for Buf {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        // output[0].silence();
        // if inputs.len() > 0 {
            
            // each input value is between 0-1
        let input_buf = &mut inputs[0].buffers();

        for i in 0..64 {
            let len = self.sample.len() - 1;
            let index = input_buf[0][i] * len as f32;

            output[0][i] = match index {
                x if x == 0.0 => self.sample[0],
                x if x == len as f32 => self.sample[len],
                x if x > 0.0 && x < len as f32 => {
                    let left = x.floor();
                    let right = x.ceil();
                    self.sample[left as usize] * (x - left) + self.sample[right as usize] * (right - x)
                },
                _ => 0.0
            };
        }
    }
}