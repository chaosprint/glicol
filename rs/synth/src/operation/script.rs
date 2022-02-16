use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use dasp_interpolate::linear::Linear;
use dasp_interpolate::sinc::Sinc;
use rhai::{Engine, EvalAltResult};
use std::sync::Mutex;

use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};
type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Script<const N: usize> {
    buf: Fixed,
    sr: usize,
    code: String,
    engine: rhai::Engine,
}

impl<const N: usize> Script<N> {
    pub fn new() -> Self {
        Self { 
            buf: ring_buffer::Fixed::from(vec![0.0]), 
            sr: 44100, 
            code: "21.0*2.0".to_owned(),
            engine: Engine::new()
        }
    }
    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    pub fn code(self, code: String) -> Self {
        Self {code, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node!( N, self)
    }
}

impl<const N: usize> Node<N> for Script<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {

        let l = inputs.len();
        let has_clock;
        if l >= 1 { 
            has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0
            && inputs[l-1].buffers()[0][1] == 0.;
        } else {
            has_clock = false;
        }

        if l - has_clock as usize > 1 { // has mod
            // let modulator = inputs[0].buffers()[0].clone();
            // let input_sig = inputs[1].buffers()[0].clone();
            for i in 0..N {
                // let result = self.engine.eval::<f64>(&self.code);
                // // output[0][i] = result.unwrap() as f32;
                // // println!("result {:?}", result.unwrap());
                // match result {
                //     Ok(v) => output[0][i] = v as f32,
                //     _ => return ()
                // }
            }
        } else {
            // let input_sig = inputs[0].buffers()[0].clone();
            for i in 0..N {
                let result = self.engine.eval::<f64>(&self.code).unwrap();
                output[0][i] = result as f32;
                // println!("result {:?}", result);
                // match result {
                //     Ok(v) => output[0][i] = v as f32,
                //     _ => panic!("eval not good")
                // }
            }
        }
    }
}