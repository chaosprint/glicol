use glicol_synth::{
    oscillator::{SinOsc, SquOsc, TriOsc, SawOsc},
    filter::{ResonantLowPassFilter, OnePole},
    signal::{ConstSig, Impulse},
    operator::{Mul, Add},
    sampling::Sampler,
};

use glicol_synth::{NodeData, BoxedNodeSend}; //, Processor, Buffer, Input, Node
use glicol_parser::{GlicolPara};
use glicol_macros::get_one_para_from_number_or_ref;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
// pub type NodeResult<const N: usize> = Result<(GlicolNodeData<N>, Vec<String>), GlicolError>;

pub fn makenode<'a, const N: usize>(
    name: &str,
    paras: &mut Vec<GlicolPara<'a>>,
    // pos: (usize, usize),
    samples_dict: &std::collections::HashMap<&'a str, (&'static[f32], usize)>,
    // sr: usize,
    // bpm: f32,
) -> (GlicolNodeData<N>, Vec<&'a str>) {
    let (nodedata, reflist) = match name {
        "sp" => {
            match paras[0] {
                GlicolPara::Symbol(s) => {
                    (Sampler::new(samples_dict[s]).to_boxed_nodedata(2), vec![])
                }
                _ => {
                    unimplemented!();
                }
            }
        },
        "lpf" => {
            let data = ResonantLowPassFilter::new().cutoff(
                match paras[0] {
                    GlicolPara::Number(v) => v,
                    GlicolPara::Reference(_) => 100.0,
                    _ => unimplemented!()
                }
            ).q(
                match paras[1] {
                    GlicolPara::Number(v) => v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(1);

            let mut reflist = vec![];
            match paras[0] {
                GlicolPara::Reference(s) => reflist.push(s),
                _ => {}
            };
            (data, reflist)
            // match paras[0] {
            //     GlicolPara::Number(v) => {
            //         (TriOsc::new().freq(v).to_boxed_nodedata(1), vec![])
            //     },
            //     GlicolPara::Reference(s) => {
            //         (TriOsc::new().freq(0.0).to_boxed_nodedata(1), vec![s])
            //     },
            //     _ => {
            //         unimplemented!();
            //     }
            // }
            
        },
        "tri" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (TriOsc::new().freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (TriOsc::new().freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "squ" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (SquOsc::new().freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SquOsc::new().freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "saw" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (SawOsc::new().freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SawOsc::new().freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
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
        "imp" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (Impulse::new().freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Impulse::new().freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "mul" => get_one_para_from_number_or_ref!(Mul),
        "onepole" => get_one_para_from_number_or_ref!(OnePole),
        "add" => get_one_para_from_number_or_ref!(Add),
        "constsig" => get_one_para_from_number_or_ref!(ConstSig),
        _ => unimplemented!()
    };
    return (nodedata, reflist)
}