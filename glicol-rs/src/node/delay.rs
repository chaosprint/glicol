use dasp_graph::{Buffer, Input, Node};
// use dasp_signal::{self as signal, Signal};
use dasp_ring_buffer as ring_buffer;
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct Delay {
    buf: ring_buffer::Fixed<Vec<f32>>
}

impl Delay {
    pub fn new(paras: &mut Pairs<Rule>) -> 
    Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        let delay = paras.as_str().to_string();
        match delay.parse::<f32>() {
            Ok(value) => {
                // const v: usize = value as usize;
                Ok((NodeData::new1(BoxedNodeSend::new( Self {
                    buf: ring_buffer::Fixed::from(vec![0.0; value as usize])
                })), vec![]))
            },
            Err(_) => {
                Ok((NodeData::new1(BoxedNodeSend::new( Self {
                    buf: ring_buffer::Fixed::from(vec![0.0; 88200])
                })), vec![delay]))
            }
        }
        // let sig = signal::noise(0);
    }
}

impl Node for Delay {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        for i in 0..64 {
            // println!("{:?}", self.buf);
            output[0][i] = self.buf[0];
            // save new input to ring buffer
            self.buf.push(inputs[0].buffers()[0][i]);
        }
    }
}