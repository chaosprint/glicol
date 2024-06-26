use crate::{Buffer, Input, Message, Node};
use dasp_ring_buffer as ring_buffer;
use hashbrown::HashMap;
type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct AllPassFilterGain {
    gain: f32,
    bufx: Fixed,
    bufy: Fixed,
    sr: usize,
    input_order: Vec<usize>,
}

impl Default for AllPassFilterGain {
    fn default() -> Self {
        Self::new()
    }
}

impl AllPassFilterGain {
    pub fn new() -> Self {
        Self {
            gain: 0.5,
            sr: 44100,
            bufx: ring_buffer::Fixed::from(vec![0.0]),
            bufy: ring_buffer::Fixed::from(vec![0.0]),
            input_order: Vec::new(),
        }
    }

    pub fn delay(self, delay: f32) -> Self {
        let size = if delay == 0.0 {
            3. * self.sr as f32
        } else {
            delay / 1000. * self.sr as f32
        } as usize;
        Self {
            bufx: ring_buffer::Fixed::from(vec![0.0; size]),
            bufy: ring_buffer::Fixed::from(vec![0.0; size]),
            ..self
        }
    }

    pub fn gain(self, gain: f32) -> Self {
        Self { gain, ..self }
    }

    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }
}

impl<const N: usize> Node<N> for AllPassFilterGain {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs[1] {:?}", inputs[1].buffers());
        match inputs.len() {
            1 => {
                for i in 0..N {
                    // println!("{:?}", self.buf);
                    let xn = inputs[&self.input_order[0]].buffers()[0][i];
                    let yn = -self.gain * xn + self.bufx[0] + self.gain * self.bufy[0];

                    // save new input to ring buffer
                    self.bufx.push(xn);
                    self.bufy.push(yn);
                    output[0][i] = yn;
                }
            }
            2 => {
                let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id

                for ((out, xn), mod_buf) in output[0]
                    .iter_mut()
                    .zip(main_input.buffers()[0].iter())
                    .zip(ref_input.buffers()[0].iter())
                {
                    let mut pos = -mod_buf / 1000. * self.sr as f32;
                    while pos < 0. {
                        pos += self.bufx.len() as f32;
                    }
                    let pos_int = pos.floor() as usize;
                    let pos_frac = pos.fract();

                    let xdelay = self.bufx.get(pos_int) * pos_frac
                        + self.bufx.get(pos_int + 1) * (1. - pos_frac);
                    let ydelay = self.bufy.get(pos_int) * pos_frac
                        + self.bufy.get(pos_int + 1) * (1. - pos_frac);

                    let yn = -self.gain * xn + xdelay + self.gain * ydelay;

                    self.bufx.push(*xn);
                    self.bufy.push(yn);
                    *out = yn;
                }
            }
            _ => (),
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => match pos {
                0 => {
                    let delay_n = (value / 1000. * self.sr as f32) as usize;
                    self.bufx.set_first(delay_n);
                    self.bufy.set_first(delay_n);
                }
                1 => self.gain = value,
                _ => {}
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
