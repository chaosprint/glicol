use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use dasp_ring_buffer as ring_buffer;
type Fixed = ring_buffer::Fixed<Vec<f32>>;
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct DelayMs {
    buf: Vec<Fixed>,
    sr: usize,
    input_order: Vec<usize>,
    delay_n: usize,
}

impl DelayMs {
    pub fn new() -> Self {
        Self { buf: vec![], delay_n: 1, sr: 44100, input_order: vec![] }
    }
    
    pub fn delay(self, delay: f32, chan: u8) -> Self {
        let mut buf;
        let delay_n = (delay / 1000. * self.sr as f32) as usize;

        if delay_n == 0 {
            buf = vec![];
            for _ in 0..chan {
                buf.push(Fixed::from(vec![0.0; 1]))
            }
        } else {
            buf = vec![];
            for _ in 0..chan {
                buf.push(Fixed::from(vec![0.0; delay_n]))
            }            
        };
        Self { buf, delay_n, ..self}
    }
    
    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    impl_to_boxed_nodedata!();
}


impl<const N: usize> Node<N> for DelayMs {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => { // no modulation
                let main_input = inputs.values_mut().next().unwrap();
                if self.delay_n == 0 { // equal to a pass node
                    for i in 0..N {
                        output[0][i] = main_input.buffers()[0][i];
                    }
                } else {
                    if main_input.buffers().len() == 1 {
                        for i in 0..N {
                            output[0][i] = self.buf[0].push(main_input.buffers()[0][i]);
                        }
                    } else if main_input.buffers().len() == 2 {
                        for i in 0..N {
                            output[0][i] = self.buf[0].push(main_input.buffers()[0][i]);
                            output[1][i] = self.buf[1].push(main_input.buffers()[1][i]);
                        }
                    } else {
                    }
                }
            },
            2 => {
                let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id

                let mod_buf = &mut ref_input.buffers();
                for i in 0..N {
                    let mut pos = - mod_buf[0][i] / 1000. * self.sr as f32;
                    while pos < 0. {
                        pos += self.buf[0].len() as f32;
                    };
                    let pos_int = pos.floor() as usize;
                    let pos_frac = pos.fract();
                    if main_input.buffers().len() == 1 {
                        output[0][i] = self.buf[0].get(pos_int) * pos_frac + self.buf[0].get(pos_int+1) * (1.-pos_frac);
                        self.buf[0].push(main_input.buffers()[0][i]);
                    } else if main_input.buffers().len() == 2 {
                        output[0][i] = self.buf[0].get(pos_int) * pos_frac + self.buf[0].get(pos_int+1) * (1.-pos_frac);
                        self.buf[0].push(main_input.buffers()[0][i]);
                        output[1][i] = self.buf[1].get(pos_int) * pos_frac + self.buf[1].get(pos_int+1) * (1.-pos_frac);
                        self.buf[1].push(main_input.buffers()[1][i]);
                    } else {

                    }

                    // output[1][i] = self.buf2.get(pos_int) * pos_frac + self.buf2.get(pos_int+1) * (1.-pos_frac);
                    
                    // self.buf2.push(main_input.buffers()[1][i]);
                }
            }
            _ => {return ()}
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {
                        let delay_n = (value / 1000. * self.sr as f32) as usize;
                        self.delay_n = delay_n;

                        if delay_n == 0 {
                            self.buf = vec![];
                        } else {
                            let chan = self.buf.len();
                            self.buf = vec![];
                            for _ in 0..chan {
                                self.buf.push(Fixed::from(vec![0.0; delay_n]))
                            }
                        };
                    },
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}