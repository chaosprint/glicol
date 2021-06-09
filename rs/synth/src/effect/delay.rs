use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};

type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Delay {
    buf: Fixed,
    sr: usize
}

impl Delay {

    pub fn new() -> Self {
        Self { buf: ring_buffer::Fixed::from(vec![0.0]), sr: 44100 }
    }

    pub fn delay(self, delay: f32) -> Self {
        let size;
        if delay == 0.0 {
            size = (10.0 * self.sr as f32) as usize;
        } else {
            size = (delay * self.sr as f32) as usize
        };
        Self { buf: ring_buffer::Fixed::from(vec![0.0; size]),..self}
    }

    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData {
        mono_node!(self)
    }
}

#[macro_export]
macro_rules! delay {
    ({$($para: ident: $data:expr),*}) => {
         (
            Delay::new()$(.$para($data))*.build()
        )
    }
}

impl Node<128> for Delay {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let l = inputs.len();
        if l < 1 { return ()};
        let has_clock = inputs[l-1].buffers()[0][0] % 128. == 0. && inputs[l-1].buffers()[0][1] == 0.;

        // println!("{}{}",l ,has_clock );
        if l - has_clock as usize > 1 { // has mod
            let input_sig = inputs[1].buffers()[0].clone();
            let modulator = inputs[0].buffers()[0].clone();
            let delay_len = (modulator[0] / 1000.0 * self.sr as f32 ) as usize;
            self.buf.set_first(self.buf.len() - delay_len);
            for i in 0..128 {
                output[0][i] = self.buf[0];
                self.buf.push(input_sig[i]);
            }
        } else {               
            for i in 0..128 {
                output[0][i] = self.buf[0];
                // save new input to ring buffer
                self.buf.push(inputs[0].buffers()[0][i]);
            }
        }
    }
}