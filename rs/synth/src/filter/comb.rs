use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};

type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Comb {
    delay: f32,
    gain: f32,
    feedforward: f32,
    feedback: f32,
    bufx: Fixed,
    bufy: Fixed,
    sr: usize,
    // sidechain_ids: Vec::<u8>
}

impl Comb {
    pub fn new() -> Self {
        Self { 
            bufx: ring_buffer::Fixed::from(vec![0.0]), 
            bufy: ring_buffer::Fixed::from(vec![0.0]),
            delay: 5000.,
            gain: 0.5,
            feedback: 0.5,
            feedforward: 0.5,
            sr: 44100 
        }
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
            ..self
        }
    }

    pub fn gain(self, gain: f32) -> Self {
        Self {gain, ..self}
    }

    pub fn feedforward(self, feedforward: f32) -> Self {
        Self {feedforward, ..self}
    }

    pub fn feedback(self, feedback: f32) -> Self {
        Self {feedback, ..self}
    }

    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData {
        mono_node!(self)
    }
}
    // handle_params!(
    //     {
    //         delay_time: 600.0,
    //         gain: 0.5,
    //         forward: 0.5,
    //         back: 0.5
    //     }, [
    //         (
    //             delay_time, bufx, |d: f32| -> Fixed {
    //                 let size = (d / 1000.0 * 44100.0) as usize;
    //                 ring_buffer::Fixed::from(vec![0.0; size])
    //             }
    //         ), (
    //             delay_time, bufy, |d: f32| -> Fixed {
    //                 let size = (d / 1000.0 * 44100.0) as usize;
    //                 ring_buffer::Fixed::from(vec![0.0; size])
    //             }
    //         )
    //     ]
    // );
#[macro_export]
macro_rules! comb {
    ({$($para: ident: $data:expr),*}) => {
            (
            Comb::new()$(.$para($data))*.build()
        )
    }
}

// TODO: modulation?

impl Node<128> for Comb {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let a = self.gain;
        let b = self.feedforward;
        let c = self.feedback;
        for i in 0..128 {
            let xn = inputs[0].buffers()[0][0];
            let xn_d = self.bufx[0];
            let yn_d = self.bufy[0];
            let yn = a * xn + b * xn_d + c * yn_d;
            self.bufx.push(xn);
            self.bufy.push(yn);
            output[0][i] = yn;
        }
    }
}