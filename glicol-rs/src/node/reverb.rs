use dasp_graph::{Buffer, Input, Node};
use super::super::{Engine, Rule, NodeData, BoxedNodeSend, EngineError, midi_or_float};
use pest::iterators::Pairs;

pub struct Plate {
    engine: Engine
}

impl Plate {
    pub fn new(_paras: &mut Pairs<Rule>) -> Result<
    (NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        // let param_a = paras.as_str().parse::<f32>().unwrap();
        let mut engine = Engine::new();
        engine.set_code("out: ~input >> apf 600.0 2000.0");
        engine.make_graph()?;
        engine.update();
        Ok((NodeData::new2(BoxedNodeSend::new( Self {
            engine
        })), vec![]))
    }
}

impl Node for Plate {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        self.engine.input(inputs); // mono or stereo?
        let buf = self.engine.gen_next_buf_64().unwrap();
        for i in 0..64 {
            output[0][i] = buf[i];
            output[1][i] = buf[i+64];
        }
    }
}