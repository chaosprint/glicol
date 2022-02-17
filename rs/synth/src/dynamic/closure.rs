use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use dasp_interpolate::linear::Linear;
use dasp_interpolate::sinc::Sinc;
use rhai::{Engine, Array, Scope, Dynamic, EvalAltResult, Func};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};
type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Closure<const N: usize> {
    buf: Fixed,
    sr: usize,
    code: String,
    engine: Engine,
    phase: usize,
}

impl<const N: usize> Closure<N> {
    pub fn new() -> Self {
        let mut phase: usize = 0;
        let mut scope = Scope::new();
        Self { 
            buf: ring_buffer::Fixed::from(vec![0.0]), 
            sr: 44100, 
            code: "21.0*2.0".to_owned(),
            engine: Engine::new(),
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

impl<const N: usize> Node<N> for Closure<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {

        let l = inputs.len();
        if l < 1 { return ()};
        let has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0 && inputs[l-1].buffers()[0][1] == 0.;

        let func = Func::<(Array), Array>::create_from_script(
            &self.engine,         // the 'Engine' is consumed into the closure
            &self.code,         // the script, notice number of parameters must match
            "process"          // the entry-point function name
        ).unwrap();

        let arr = Array::new();
        for i in 0..N {
            arr.push(Dynamic::from_float(inputs[0].buffers()[0][i]));
        }
        let out = func(arr).unwrap();
        for i in 0..N {
            output[0][i] = out[i].as_float().unwrap();
        }
        self.phase += N;
    }
}