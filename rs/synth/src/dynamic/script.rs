use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use dasp_interpolate::linear::Linear;
use dasp_interpolate::sinc::Sinc;
use rhai::{Engine, Array, Scope, Dynamic, EvalAltResult};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};
type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Script<const N: usize> {
    buf: Fixed,
    sr: usize,
    code: String,
    scope: Scope<'static>,
    engine: Engine,
    phase: usize,
}

impl<const N: usize> Script<N> {
    pub fn new() -> Self {
        let mut phase: usize = 0;
        let mut scope = Scope::new();
        // scope.push("x1", 0.1)
        // .push("x2", 0.2)
        // .push("y1", 0.2)
        // .push("y2", 0.2)
        // .push("phase", phase as f64);
        Self { 
            buf: ring_buffer::Fixed::from(vec![0.0]), 
            sr: 44100, 
            code: "21.0*2.0".to_owned(),
            engine: Engine::new(),
            scope,
            phase
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

        // let ast = self.engine.compile_with_scope(&mut self.scope, &self.code).unwrap();
        // let result: Array = self.engine.eval_ast_with_scope(&mut self.scope, &ast).unwrap();
        let ast = self.engine.compile(&self.code).unwrap();
        let result: Array = self.engine.eval_ast(&ast).unwrap();
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
                output[0][i] = result[i].as_float().unwrap() as f32;
                // println!("result {:?}", result);
                // match result {
                //     Ok(v) => output[0][i] = v as f32,
                //     _ => panic!("eval not good")
                // }
            }
        }
        self.phase += N;
    }
   
}