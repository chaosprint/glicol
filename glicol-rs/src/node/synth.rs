use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use super::super::{Rule, HashMap, NodeData, BoxedNodeSend, EngineError};

pub struct Synth {
    // pub sig: Vec< Box<dyn Signal<Frame=[f32;1]> + 'static + Send>>,
    playback: Vec<(usize, f64)>,
    // sample: &'static[f32],
    // len: usize,
    // endindex: usize,
}

impl Synth {
    pub fn new(
        paras: &mut Pairs<Rule>,
        samples_dict: &HashMap<String, &'static[f32]>
    ) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        // paras include names, attack, decay
        // repeatable \saw _ _ \squ _ _

        Ok((NodeData::new1(BoxedNodeSend::new(Self{
            playback: Vec::new(),
        })), vec![]))
    }
}

impl Node for Synth {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
    }
}