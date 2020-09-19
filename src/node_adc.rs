
use dasp_graph::{Buffer, Input, Node};

pub struct Adc {}

impl Node for Adc {
    fn process(&mut self, _inputs: &[Input], _output: &mut [Buffer]) {
        // output[0] = inputs[0].buffers()[0].clone();
        // output[0].iter_mut().for_each(|s| *s = 0.5);
    }
}