extern crate freeverb;
use freeverb::Freeverb;

use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct FreeVerbNode {
    freeverb: Freeverb,
}
impl FreeVerbNode {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        let para_a: String = paras.next().unwrap().as_str().to_string();
        let para_b: String = paras.next().unwrap().as_str().to_string();
        let para_c: String = paras.next().unwrap().as_str().to_string();

        let room_size = para_a.parse::<f64>()?;
        let dampening = para_b.parse::<f64>()?;
        let wet = para_c.parse::<f64>()?;

        // println!("{},{},{}", room_size, dampening, wet);

        let mut freeverb = Freeverb::new(44100);
        freeverb.set_wet(wet);
        freeverb.set_width(0.5);
        freeverb.set_dampening(dampening);
        freeverb.set_room_size(room_size);

        // println!("{:?}", freeverb.tick((1.0, 1.0)));
        Ok((NodeData::new1(BoxedNodeSend::new( Self {
            freeverb: freeverb
        })), vec![]))
    }
}

impl Node for FreeVerbNode {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        // output[0] = inputs[0].buffers()[0].clone();
        let buf = inputs[0].buffers()[0].clone();
        for i in 0..64 {
            let (out0, _out1) = self.freeverb.tick(
                (buf[i] as f64, 0.0)
            );
            output[0][i] = out0 as f32;
        }
        // println!("{:?},{:?}", buf, output[0]);
    }
}