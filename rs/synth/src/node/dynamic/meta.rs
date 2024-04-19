use crate::{BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;
use rhai::{Array, Dynamic, Engine, OptimizationLevel, Scope, AST};

pub struct Meta<const N: usize> {
    sr: usize,
    // phase: usize,
    pub code: String,
    pub backup: String,
    ast: AST,
    scope: Scope<'static>,
    engine: Engine,
    input_order: Vec<usize>,
}

impl<const N: usize> Default for Meta<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Meta<N> {
    pub fn new() -> Self {
        let phase: usize = 0;
        let mut scope = Scope::new();
        let output = Vec::<Dynamic>::with_capacity(N);

        scope
            .push("phase", phase as f32)
            .push("x0", 0.0_f32)
            .push("x1", 0.0_f32)
            .push("x2", 0.0_f32)
            .push("x3", 0.0_f32)
            .push("y0", 0.0_f32)
            .push("y1", 0.0_f32)
            .push("y2", 0.0_f32)
            .push("y3", 0.0_f32)
            .push("z0", 0.0_f32)
            .push("z1", 0.0_f32)
            .push("z2", 0.0_f32)
            .push("z3", 0.0_f32)
            .push("a", 0.0_f32)
            .push("b", 0.0_f32)
            .push("c", 0.0_f32)
            .push("d", 0.0_f32)
            .push("e", 0.0_f32)
            .push("f", 0.0_f32)
            .push("g", 0.0_f32)
            .push("h", 0.0_f32)
            .push("i", 0.0_f32)
            .push("j", 0.0_f32)
            .push("k", 0.0_f32)
            .push("l", 0.0_f32)
            .push("m", 0.0_f32)
            .push("n", 0.0_f32)
            .push("o", 0.0_f32)
            .push("p", 0.0_f32)
            .push("q", 0.0_f32)
            .push("r", 0.0_f32)
            .push("s", 0.0_f32)
            .push("t", 0.0_f32)
            .push("u", 0.0_f32)
            .push("v", 0.0_f32)
            .push("w", 0.0_f32)
            .push("x", 0.0_f32)
            .push("y", 0.0_f32)
            .push("z", 0.0_f32)
            .push("freq", 0.0_f32)
            .push("freq2", 0.0_f32)
            .push("output", output);

        let mut engine = Engine::new();
        engine.set_optimization_level(OptimizationLevel::Full);
        let ast = engine.compile("").unwrap();

        Self {
            sr: 44100,
            engine,
            scope,
            code: "".to_owned(),
            backup: "".to_owned(),
            ast,
            // phase,
            input_order: Vec::new(),
        }
    }

    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }

    pub fn code(mut self, code: String) -> Self {
        if let Ok(a) = self.engine.compile(&code) {
            self.ast = a;
        };
        Self { code, ..self }
    }

    pub fn to_boxed_nodedata(mut self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        self.scope.push("sr", self.sr as f32);
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new(self))
    }
}

impl<const N: usize> Node<N> for Meta<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        if !inputs.is_empty() {
            let arr: Array = inputs[&self.input_order[0]].buffers()[0]
                .iter()
                .map(|f| Dynamic::from_float(*f))
                .collect();

            self.scope.set_or_push("input", arr);
            // self.engine.optimize_ast();
        }

        match self
            .engine
            .eval_ast_with_scope::<Array>(&mut self.scope, &self.ast)
        {
            Ok(result) => for (out, res) in output[0].iter_mut().zip(result.iter()) {
                if let Ok(v) = res.as_float() {
                    *out = v;
                };
            }
            Err(e) => {
                // TODO What do we do with this Box<EvalAltResult>?
                _ = e;
            }
        }
        // self.phase += N;
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToSymbol(0, s) => if let Ok(a) = self.engine.compile(s) {
                self.ast = a;
            },
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
