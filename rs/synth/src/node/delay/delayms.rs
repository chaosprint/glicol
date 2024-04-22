use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
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

impl Default for DelayMs {
    fn default() -> Self {
        Self::new()
    }
}

impl DelayMs {
    pub fn new() -> Self {
        Self {
            buf: vec![],
            delay_n: 1,
            sr: 44100,
            input_order: vec![],
        }
    }

    pub fn delay(self, delay: f32, chan: u8) -> Self {
        let delay_n = ((delay / 1000. * self.sr as f32) as usize).max(1);
        let buf = vec![Fixed::from(vec![0.0; delay_n]); chan as usize];

        Self {
            buf,
            delay_n,
            ..self
        }
    }

    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }

    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for DelayMs {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        let main_input = inputs.values().next().unwrap();
        if !(1..=2).contains(&main_input.buffers().len()) {
            return;
        }

        match inputs.len() {
            1 => {
                // no modulation
                match self.delay_n {
                    // equal to a pass node
                    0 => output[0].copy_from_slice(&main_input.buffers()[0]),
                    _ =>  {
                        let iter = self.buf.iter_mut().zip(output.iter_mut()).zip(main_input.buffers());

                        for ((fixed, out_buf), main_buf) in iter {
                            for (out, main) in out_buf.iter_mut().zip(main_buf.iter()) {
                                *out = fixed.push(*main);
                            }
                        }
                    }
                }
            }
            2 => {
                let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id

                let mod_buf = &mut ref_input.buffers();
                for i in 0..N {
                    let mut pos = -mod_buf[0][i] / 1000. * self.sr as f32;
                    while pos < 0. {
                        pos += self.buf[0].len() as f32;
                    }
                    let pos_int = pos.floor() as usize;
                    let pos_frac = pos.fract();

                    let iter = self.buf.iter_mut().zip(output.iter_mut()).zip(main_input.buffers());

                    for ((fixed, out_buf), main_buf) in iter {
                        out_buf[i] = fixed.get(pos_int) * pos_frac + fixed.get(pos_int + 1) * (1. - pos_frac);
                        fixed.push(main_buf[i]);
                    }
                }

                // output[1][i] = self.buf2.get(pos_int) * pos_frac + self.buf2.get(pos_int+1) * (1.-pos_frac);

                // self.buf2.push(main_input.buffers()[1][i]);
            }
            _ => (),
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(0, value) => {
                let delay_n = (value / 1000. * self.sr as f32) as usize;
                self.delay_n = delay_n;

                if delay_n == 0 {
                    self.buf.clear();
                } else {
                    let chan = self.buf.len();
                    self.buf = vec![Fixed::from(vec![0.0; delay_n]); chan];
                };
            },
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
