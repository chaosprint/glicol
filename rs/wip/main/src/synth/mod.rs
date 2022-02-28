pub mod signal;
pub mod oscillator;
pub mod operator;

use {
    oscillator::{SinOsc},
    signal::{ConstSig},
    operator::{Mul},
};

use dasp_graph::{NodeData, BoxedNodeSend}; //, Processor, Buffer, Input, Node
use glicol_parser::{GlicolPara, get_num};
// use pest::iterators::Pair;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
// pub type NodeResult<const N: usize> = Result<(GlicolNodeData<N>, Vec<String>), GlicolError>;

pub fn makenode<const N: usize>(
    name: &str,
    paras: &mut Vec<GlicolPara>,
    // pos: (usize, usize),
    // samples_dict: &HashMap<String, &'static[f32]>,
    // sr: usize,
    // bpm: f32,
) -> GlicolNodeData<N> {
    let nodedata = match name {
        "sin" => {
            // todo consider multi paras, consider refs
            SinOsc::new().freq(get_num(paras[0])).build()
        },
        "mul" => {
            Mul::new(get_num(paras[0]))
        },
        "constsig" => {
            ConstSig::new(get_num(paras[0]))
        },
        _ => unimplemented!(),
    };
    return nodedata
}