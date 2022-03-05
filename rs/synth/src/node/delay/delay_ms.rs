use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use dasp_interpolate::{
    Interpolator,
    sinc::Sinc,
    // linear::Linear,
};
// use dasp_interpolate::

type Fixed = ring_buffer::Fixed<Vec<f32>>;

#[derive(Debug, Clone)]
pub struct DelayMs {
    buf: Fixed,
    sr: usize,
    delay_n: usize,
}

impl DelayMs {
    pub fn new() -> Self {
        Self { buf: ring_buffer::Fixed::from(vec![0.0]), sr: 44100, delay_n: 0 }
    }
    pub fn delay(self, delay: f32) -> Self {
        let mut buf;
        if delay == 0.0 {
            buf = ring_buffer::Fixed::from(vec![0.0; self.sr * 2]);
            buf.set_first(buf.len()-1);
            // size = (100.0 * self.sr as f32) as usize;
        } else {
            buf = ring_buffer::Fixed::from(vec![0.0; (delay / 1000. * self.sr as f32) as usize * 2 ]);
        };
        Self { buf,..self}
    }
    
    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    impl_to_boxed_nodedata!();
}


impl<const N: usize> Node<N> for DelayMs {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                for i in 0..N {
                    output[0][i] = self.buf.push(inputs[0].buffers()[0][i]);
                }
            },
            2 => {
                let new_delay_n = (inputs[0].buffers()[0][0] / 1000. * self.sr as f32 ) as usize;

                // this will cause some padding 0.0 if the new_delay_n is longer
                // or lost some previous samples if the new_delay_n is shorter
                self.buf.set_first(new_delay_n);
                let interp = Sinc::new(self.buf.clone());
                for i in 0..N {
                    let pos = i as f64 / N as f64;
                    let x = interp.interpolate(pos);
                    output[0][i] = x;
                    let _ = self.buf.push(inputs[1].buffers()[0][i]);
                }

                // let x = interp.interpolate(0.1);                               

                // let new_delay_n = (inputs[0].buffers()[0][0] / 1000. * self.sr as f32 ) as usize;
                // if new_delay_n == 0 {
                //     self.buf.set_first(self.buf.len()-1);
                //     for i in 0..N {
                //         output[0][i] = self.buf.push(inputs[1].buffers()[0][i]);
                //     }
                //     return ();
                // };
                // if self.delay_n != new_delay_n {
                //     let raw = self.buf.clone().into_raw_parts();
                //     let mut source = signal::from_iter(raw.1.iter().cloned());
                //     let a = source.next();
                //     let b = source.next();
                //     let interp = Linear::new(a, b);
                //     let interp_vec: Vec<_> = source.scale_hz(interp, self.delay_n as f64 / new_delay_n as f64).take(new_delay_n).collect();
                //     self.buf = Fixed::from(interp_vec);
                //     let new_head = (raw.0 as f64 / self.delay_n as f64 * new_delay_n as f64) as usize;
                //     self.buf.set_first(new_head);
                //     self.delay_n = new_delay_n;
                // }
                // for i in 0..N {
                //     output[0][i] = self.buf.push(inputs[1].buffers()[0][i]);
                // }
            }
            _ => {return ()}
        }
    }

    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(v) => {
                match v.0 {
                    0 => { self.buf.set_first(v.1 as usize) },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}