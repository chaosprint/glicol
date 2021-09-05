use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};

type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct AllpassDecay<const N: usize> {
    delay: f32,
    decay: f32,
    bufx: Fixed,
    bufy: Fixed,
    sr: usize,
}

impl<const N: usize> AllpassDecay<N> {

    pub fn new() -> Self {
        Self {
            delay: 0.5,
            decay: 2.0,
            sr: 44100,
            bufx: ring_buffer::Fixed::from(vec![0.0]),
            bufy: ring_buffer::Fixed::from(vec![0.0])
        }
    }

    pub fn decay(self, decay: f32) -> Self {
        Self { decay,..self}
    }

    pub fn delay(self, delay: f32) -> Self {
        let size;
        if delay == 0.0 {
            size = (10.0 * self.sr as f32) as usize;
        } else {
            size = (delay * self.sr as f32) as usize
        };
        Self { 
            bufx: ring_buffer::Fixed::from(vec![0.0; size]),
            bufy: ring_buffer::Fixed::from(vec![0.0; size]),
            ..self}
    }

    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node!( N, self )
    }
}

impl<const N: usize> Node<N> for AllpassDecay<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // TODO: modulation?
        let a = (0.001_f32.log10() * (self.delay / self.decay)).exp();
        for i in 0..N {
            // println!("{:?}", self.buf);
            let xn = inputs[0].buffers()[0][i];
            let yn = -a as f32 * xn + self.bufx[0] + a as f32 * self.bufy[0];
            // save new input to ring buffer
            self.bufx.push(xn);
            self.bufy.push(yn);
            output[0][i] = yn;
        }
    }
}