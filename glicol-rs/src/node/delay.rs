use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError, handle_params};

type Fixed = ring_buffer::Fixed<Vec<f32>>;

#[allow(dead_code)]
pub struct DelayN {
    sidechain_ids: Vec<u8>,
    n: f32,
    buf: Fixed
}

#[allow(dead_code)]
impl DelayN {
    handle_params!({
        n: 44100.0
    }, [(n, buf, |delay: f32|->Fixed {
        ring_buffer::Fixed::from(vec![0.0; delay as usize])
    })]);
}

impl Node for DelayN {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        for i in 0..64 {
            output[0][i] = self.buf[0];
            // save new input to ring buffer
            self.buf.push(inputs[0].buffers()[0][i]);
        }
    }
}

#[allow(dead_code)]
pub struct Delay {
    sidechain_ids: Vec<u8>,
    delay: f32,
    buf: Fixed
}

#[allow(dead_code)]
impl Delay {
    handle_params!({
        delay: 5000.0
    }, [(delay, buf, |d: f32|->Fixed {
            let size = (d / 1000.0 * 44100.0) as usize;
            ring_buffer::Fixed::from(vec![0.0; size])
        })
    ]);

    // pub fn new(paras: &mut Pairs<Rule>) -> 
    // Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
    //     let delay = paras.as_str().to_string();
    //     match delay.parse::<f32>() {
    //         Ok(value) => {
    //             // const v: usize = value as usize;
    //             Ok((NodeData::new1(BoxedNodeSend::new( Self {
    //                 buf:  ring_buffer::Fixed::from(vec![0.0; value as usize]),
    //                 delay: 0.1,
    //                 sidechain_ids: vec![]
    //             })), vec![delay]))
    //         },
    //         Err(_) => {
    //             Ok((NodeData::new1(BoxedNodeSend::new( Self {
    //                 buf: ring_buffer::Fixed::from(vec![0.0; 88200]),
    //                 delay: 0.1,
    //                 sidechain_ids: vec![]
    //             })), vec![delay]))
    //         }
    //     }
    //     // let sig = signal::noise(0);
    // }
    
}

impl Node for Delay {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        match self.sidechain_ids.len() {
            0 => {
                for i in 0..64 {
                    output[0][i] = self.buf[0];
                    // save new input to ring buffer
                    self.buf.push(inputs[0].buffers()[0][i]);
                }
            },
            1 => {
                let input_sig = inputs[1].buffers()[0].clone();
                let modulator = inputs[0].buffers()[0].clone();
                let delay_len = (modulator[0] / 1000.0 * 44100.0 ) as usize;
                self.buf.set_first(self.buf.len() - delay_len);
                for i in 0..64 {
                    output[0][i] = self.buf[0];
                    self.buf.push(input_sig[i]);
                }
            },
            _ => {
                unimplemented!()
            }
        };
    }
}