use dasp_graph::{Buffer, Input, Node};
use super::super::*;

pub struct ResonantLowPassFilter {
    cutoff: f32,
    q: f32,
    x0: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl ResonantLowPassFilter {
    pub fn new() -> Self {
        Self {
            cutoff: 20.,
            q: 1.0,
            x0: 0.,
            x1: 0.,
            x2: 0.,
            y1: 0.,
            y2: 0.,
        }
    }
    pub fn cutoff(self, cutoff: f32) -> Self {
        Self {cutoff, ..self}
    }

    pub fn q(self, q: f32) -> Self {
        Self {q, ..self}
    }
    pub fn build(self) -> GlicolNodeData {
        mono_node! { self }
    }
}

#[macro_export]
macro_rules! rlpf {
    ({$($para: ident: $data:expr),*}) => {
         (
            ResonantLowPassFilter::new()$(.$para($data))*.build()
        )
    }
}

impl Node<128> for ResonantLowPassFilter {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        if inputs.len() == 1 {
            let theta_c = 2.0 * std::f32::consts::PI * self.cutoff / 44100.0;
            let d = 1.0 / self.q;
            let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
            let gama = (0.5 + beta) * theta_c.cos();
            let a0 = (0.5 + beta - gama) / 2.0;
            let a1 = 0.5 + beta - gama;
            let a2 = (0.5 + beta - gama) / 2.0;
            let b1 = -2.0 * gama;
            let b2 = 2.0 * beta;
            for i in 0..128 {
                let x0 = inputs[0].buffers()[0][i];
                let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 
                - b1 * self.y1 - b2 * self.y2;

                output[0][i] = y;
                self.x2 = self.x1;
                self.x1 = x0;
                self.y2 = self.y1;
                self.y1 = y;
            }
        } else {
            let theta_c = 2.0 * std::f32::consts::PI * inputs[0].buffers()[0][0] / 44100.0;
            let d = 1.0 / self.q;
            let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
            let gama = (0.5 + beta) * theta_c.cos();
            let a0 = (0.5 + beta - gama) / 2.0;
            let a1 = 0.5 + beta - gama;
            let a2 = (0.5 + beta - gama) / 2.0;
            let b1 = -2.0 * gama;
            let b2 = 2.0 * beta;

            for i in 0..128 {
                let x0 = inputs[1].buffers()[0][i];
                let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                output[0][i] = y;
                self.x2 = self.x1;
                self.x1 = x0;
                self.y2 = self.y1;
                self.y1 = y;
            }
        }
    }
}
