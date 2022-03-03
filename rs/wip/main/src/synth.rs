use glicol_synth::{
    oscillator::{SinOsc},
    signal::{ConstSig},
    operator::{Mul, Add},
};

use glicol_synth::{NodeData, BoxedNodeSend}; //, Processor, Buffer, Input, Node
use glicol_parser::{GlicolPara};
// use pest::iterators::Pair;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
// pub type NodeResult<const N: usize> = Result<(GlicolNodeData<N>, Vec<String>), GlicolError>;

pub fn makenode<'a, const N: usize>(
    name: &str,
    paras: &mut Vec<GlicolPara<'a>>,
    // pos: (usize, usize),
    // samples_dict: &HashMap<String, &'static[f32]>,
    // sr: usize,
    // bpm: f32,
) -> (GlicolNodeData<N>, Vec<&'a str>) {
    let (nodedata, reflist) = match name {
        "sin" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (SinOsc::new().freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SinOsc::new().freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "mul" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (Mul::new(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Mul::new(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "add" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (Add::new(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Add::new(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "constsig" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (ConstSig::new(v).to_boxed_nodedata(1), vec![])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        _ => unimplemented!(),
    };
    return (nodedata, reflist)
}