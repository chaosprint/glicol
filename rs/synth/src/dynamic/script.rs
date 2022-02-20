use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use dasp_interpolate::linear::Linear;
use dasp_interpolate::sinc::Sinc;
use rhai::{Engine, Array, Scope, Dynamic, EvalAltResult, OptimizationLevel};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};
type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Script<const N: usize> {
    // buf: Fixed,
    sr: usize,
    code: String,
    backup: String,
    scope: Scope<'static>,
    engine: Engine,
    phase: usize,
}

impl<const N: usize> Script<N> {
    pub fn new() -> Self {
        let mut phase: usize = 0;
        let mut scope = Scope::new();
        let mut o = Vec::<Dynamic>::with_capacity(128);
        scope.push("phase", phase as f32)
        .push("x0", 0.0)
        .push("x1", 0.0)
        .push("x2", 0.0)
        .push("y0", 0.0)
        .push("y1", 0.0)
        .push("y2", 0.0)
        .push("a", 0.0)
        .push("b", 0.0)
        .push("c", 0.0)
        .push("d", 0.0)
        .push("e", 0.0)
        .push("f", 0.0)
        .push("g", 0.0)
        .push("h", 0.0)
        .push("i", 0.0)
        .push("j", 0.0)
        .push("k", 0.0)
        .push("l", 0.0)
        .push("m", 0.0)
        .push("n", 0.0)
        .push("o", 0.0)
        .push("p", 0.0)
        .push("q", 0.0)
        .push("r", 0.0)
        .push("s", 0.0)
        .push("t", 0.0)
        .push("u", 0.0)
        .push("v", 0.0)
        .push("w", 0.0)
        .push("x", 0.0)
        .push("y", 0.0)
        .push("z", 0.0)
        .push("output", o);
        

        let mut engine = Engine::new();
        engine.set_optimization_level(OptimizationLevel::Full);
        // engine.register_type::<Fixed>()
        // .register_fn("len", Fixed::len )
        // .register_fn("push", Fixed::push )
        // .register_fn("get", Fixed::get )
        // .register_fn("set_first", Fixed::set_first );
        // let mut buf = Fixed::from(vec![0.0; 1024]);
        // scope.push("rb", buf.clone());

        Self {
            // buf,
            sr: 44100, 
            code: "output.pad(128, 0.0);output".to_owned(),
            backup: "output.pad(128, 0.0);output".to_owned(),
            engine,
            scope,
            phase,
        }
    }
    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    pub fn code(self, code: String) -> Self {
        Self {code, ..self}
    }

    pub fn build(mut self) -> GlicolNodeData<N> {
        self.scope.push("sr", self.sr as f32);
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
        
        if l - has_clock as usize >= 1 {
            let mut arr = Array::new();
            for i in 0..N {
                arr.push(Dynamic::from_float(inputs[0].buffers()[0][i]));
            }
            // let mut arr = inputs[0].buffers()[0].to_vec().iter().map(|x|Dynamic::from_float(*x)).collect::<Dynamic>();
            self.scope.set_or_push("input", arr);
        }
        let result = match self.engine.eval_with_scope::<Array>(&mut self.scope, &self.code) {
            Ok(v) => {
                if v.len() >= N {
                    let mut ok = true;
                    for i in 0..N {
                        if !v[i].as_float().is_ok() {
                            ok = false;
                        }
                    };
                    if ok {
                        v 
                    } else {
                        self.engine.eval_with_scope::<Array>(&mut self.scope, &self.backup).unwrap()
                    }
                } else {
                    self.engine.eval_with_scope::<Array>(&mut self.scope, &self.backup).unwrap()
                }
            },
            _ => {
                self.engine.eval_with_scope::<Array>(&mut self.scope, &self.backup).unwrap()
            }
        };

        for i in 0..N {
            output[0][i] = match result[i].as_float() {
                Ok(v) => v,
                _ => return ()
            };
        }
        self.phase += N;
    }
}