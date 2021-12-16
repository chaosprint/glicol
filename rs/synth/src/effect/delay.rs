use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use dasp_interpolate::linear::Linear;
use dasp_interpolate::sinc::Sinc;

use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};

type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Delay<const N:usize> {
    buf: Fixed,
    sr: usize,
    delay_n: usize,
}

impl<const N:usize> Delay<N> {

    pub fn new() -> Self {
        Self { buf: ring_buffer::Fixed::from(vec![0.0]), sr: 44100, delay_n: 1 }
    }

    pub fn delay(self, delay: f32) -> Self {
        let mut buf;
        if delay == 0.0 {
            buf = ring_buffer::Fixed::from(vec![0.0;  (2. * self.sr as f32) as usize]);
            buf.set_first(buf.len()-1);
            // size = (100.0 * self.sr as f32) as usize;
        } else {
            buf = ring_buffer::Fixed::from(vec![0.0; (delay * self.sr as f32) as usize]);
        };
        Self { buf,..self}
    }

    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node!( N, self)
    }
}

impl<const N:usize> Node<N> for Delay<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {


        let l = inputs.len();
        if l < 1 { return ()};
        let has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0 && inputs[l-1].buffers()[0][1] == 0.;

        // println!("{}{}",l ,has_clock );
        if l - has_clock as usize > 1 { // has mod
            let input_sig = inputs[1].buffers()[0].clone();
            let modulator = inputs[0].buffers()[0].clone();
            let new_delay_n = (modulator[0] * self.sr as f32 ) as usize;
            if new_delay_n == 0 {
                self.buf.set_first(self.buf.len()-1);
                for i in 0..N {
                    output[0][i] = self.buf.push(input_sig[i]);
                }
                return ();
            };
            if self.delay_n != new_delay_n {
                // self.buf.set_first(self.buf.len() - 1 - new_delay_n);

                // this has the performance issue
                // use dasp interpolation, linear or sinc 
                let raw = self.buf.clone().into_raw_parts();
                let mut source = signal::from_iter(raw.1.iter().cloned());

                // Convert the signal's sample rate using `Sinc` interpolation.
                // let ring_buffer = ring_buffer::Fixed::from([[0.0]; 128]);
                // let sinc = Sinc::new(ring_buffer);
                // let new_signal = source.from_hz_to_hz(sinc, self.delay_n as f64, new_delay_n as f64).into_source();

                // let rate = new_delay_n as f64 / self.delay_n as f64;
                let a = source.next();
                let b = source.next();
                let interp = Linear::new(a, b);
                let interp_vec: Vec<_> = source.scale_hz(interp, self.delay_n as f64 / new_delay_n as f64).take(new_delay_n).collect();
                // let ring_buffer = ring_buffer::Fixed::from([[0.0]; 100]);
                // let interp = Sinc::new(ring_buffer);
                // let interp_vec: Vec<_> = new_signal.take(new_delay_n).collect();
                self.buf = Fixed::from(interp_vec);
                let new_head = (raw.0 as f64 / self.delay_n as f64 * new_delay_n as f64) as usize;
                self.buf.set_first(new_head);
                // println!("self.buf before push {:?} len {:?}", self.buf, self.buf.len());
                // self.buf.push(88.0);
                // println!("self.buf after manuall push {:?} len {:?}", self.buf, self.buf.len());



                // implement interpolation directly
                // let raw = self.buf.clone().into_raw_parts();
                // let mut interp_vec = vec![];
                // // self.buf = Fixed::from(vec![0.0; new_delay_n]);
                // // self.buf.set_first(self.buf.len() - 1 - new_delay_n);
                // for i in 0..new_delay_n {
                //     let progress = i as f32 / (new_delay_n - 1) as f32;
                //     let point = progress * (raw.1.len() - 1) as f32;
                //     let left = point.floor() as usize;
                //     let right = point.ceil() as usize;
                //     // let right = left + 1;
                //     let portion = point.fract();
                //     // println!("progress{:?} point {:?} left {:?} right {:?} portion {:?} raw0 {:?} len {:?}", progress, point, left, right, portion, raw.0, raw.1.len());
                //     // let intervalue = match (raw.0 + right > raw.1.len() - 1) || (raw.0 + left > raw.1.len() - 1)  {
                //     //     false => raw.1[raw.0 + left] * portion + raw.1[raw.0 + right] * (1. - portion),
                //     //     true => raw.1[raw.1.len() - 1]
                //     // };
                //     interp_vec.push(raw.1[left] * portion + raw.1[right] * (1. - portion));
                //     // self.buf.push(intervalue);
                // }
                // self.buf = Fixed::from(interp_vec);
                // for i in 0..N {
                //     let position = step + 1 / old_delay;
                //     let left = position.floor();
                //     let right = 1. - left;
                //     output[0][i] = self.buf[i] * left + self.buf[i+1] * right;
                    
                //     // self.buf.push(input_sig[i]);
                // }
                self.delay_n = new_delay_n;
            }

            for i in 0..N {
                output[0][i] = self.buf.push(input_sig[i]);
            }

            
            // println!("self.delay_n {:?}", self.delay_n);
            // self.buf.set_first(0);
            // for i in 0..N {
                // output[0][i] = self.buf[0];
                // self.buf.set_first(0);
                // if input_sig[i] != 0.0 {
                    // println!("self.buf found 1.0 {:?} len {:?}", self.buf, self.buf.len());
                    // output[0][i] = self.buf.push(input_sig[i]);
                    // println!("self.buf after pushing 1.0 {:?} len {:?}", self.buf, self.buf.len());
                // } else {
                // output[0][i] = self.buf.push(input_sig[i]);
                // }

            // }
            // println!("self.buf after push {:?} len {:?}", self.buf, self.buf.len());
        } else {
            for i in 0..N {
                // output[0][i] = self.buf[0];
                // save new input to ring buffer
                output[0][i] = self.buf.push(inputs[0].buffers()[0][i]);
            }
        }
    }
}