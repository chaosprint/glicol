use glicol_synth::{
    oscillator::{SinOsc, SquOsc, TriOsc, SawOsc},
    filter::{ResonantLowPassFilter, ResonantHighPassFilter, OnePole, AllPassFilterGain},
    signal::{ConstSig, Impulse, Noise},
    operator::{Mul, Add},
    delay::{DelayN, DelayMs},
    sequencer::{Sequencer, Choose, Speed},
    envelope::EnvPerc,
    effect::{Plate, Balance},
    compound::{Bd, Hh, Sn, SawSynth, SquSynth, TriSynth},
    Pass,
    Sum2,
};

#[cfg(feature = "use-meta")]
use glicol_synth::dynamic::Meta;

#[cfg(feature = "use-samples")]
use glicol_synth::sampling::Sampler;

use hashbrown::HashMap;
use glicol_synth::{NodeData, BoxedNodeSend, GlicolPara}; //, Processor, Buffer, Input, Node
use glicol_macros::{
    get_one_para_from_number_or_ref,
    get_one_para_from_number_or_ref2
};
use crate::EngineError;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
// pub type NodeResult<const N: usize> = Result<(GlicolNodeData<N>, Vec<String>), GlicolError>;

#[allow(unused_variables, unused_mut)]
pub fn makenode<const N: usize>(
    name: &str,
    paras: &mut Vec<GlicolPara>,
    // pos: (usize, usize),
    samples_dict: &HashMap<String, (&'static[f32], usize, usize)>,
    sr: usize,
    bpm: f32,
    seed: usize
) -> Result<(GlicolNodeData<N>, Vec<String>), EngineError> {
    let (nodedata, reflist) = match name {

        #[cfg(feature="bela")]
        "adc" => {
            let port = match &paras[0] {
                GlicolPara::Number(v) => *v as usize,
                _ => unimplemented!()
            };
            (NodeData::new1( BoxedNodeSend::new(Pass{}) ), vec![format!("~adc{}", port+1)])
        },

        #[cfg(feature="use-samples")]
        "sp" => {
            match &paras[0] {
                GlicolPara::SampleSymbol(s) => {
                    if !samples_dict.contains_key(s) {
                        return Err(EngineError::NonExsitSample(s.to_owned()))
                    }
                    (Sampler::new(samples_dict[s], sr).to_boxed_nodedata(2), vec![])
                }
                _ => unimplemented!()
            }
        },

        #[cfg(feature="use-meta")]
        "meta" => {
            match &paras[0] {
                GlicolPara::Symbol(s) => {
                    (Meta::new().sr(sr).code(s.to_owned()).to_boxed_nodedata(1), vec![])
                },
                _ => unimplemented!()
            }
        },
        "lpf" => {
            let data = ResonantLowPassFilter::new().cutoff(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    GlicolPara::Reference(_) => 100.0,
                    _ => unimplemented!()
                }
            ).q(
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(1);

            let mut reflist = vec![];
            match &paras[0] {
                GlicolPara::Reference(s) => reflist.push(s.to_owned()),
                _ => {}
            };
            (data, reflist)
        },
        "balance" => {
            let data = Balance::new().to_boxed_nodedata(2);
            let reflist = vec![match &paras[0] {
                GlicolPara::Reference(s) => s.to_owned(),
                _ => "".to_owned()
            }, match &paras[1] {
                GlicolPara::Reference(s) => s.to_owned(),
                _ => "".to_owned()
            }];
            (data, reflist)
        },
        "rhpf" => {
            let data = ResonantHighPassFilter::new().cutoff(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    GlicolPara::Reference(_) => 100.0,
                    _ => unimplemented!()
                }
            ).q(
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(1);

            let mut reflist = vec![];
            match &paras[0] {
                GlicolPara::Reference(s) => reflist.push(s.to_owned()),
                _ => {}
            };
            (data, reflist)
        },
        "apfmsgain" => {
            let data = AllPassFilterGain::new().sr(sr).delay(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    GlicolPara::Reference(_) => 0.0,
                    _ => unimplemented!()
                }
            ).gain(
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(2);

            let mut reflist = vec![];
            match &paras[0] {
                GlicolPara::Reference(s) => reflist.push(s.to_owned()),
                _ => {}
            };
            (data, reflist)
        },
        "envperc" => {
            let data = EnvPerc::new().sr(sr).attack(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).decay(
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(2);
            let reflist = vec![];
            (data, reflist)
        },
        "tri" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (TriOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (TriOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s.to_owned()])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "squ" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (SquOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SquOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s.to_owned()])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "saw" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (SawOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SawOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s.to_owned()])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "sin" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (SinOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SinOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s.to_owned()])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "plate" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (Plate::new(*v).to_boxed_nodedata(2), vec![])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "imp" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (Impulse::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Impulse::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s.to_owned()])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "mul" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (Mul::new(*v).to_boxed_nodedata(2), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Mul::new(0.0).to_boxed_nodedata(2), vec![s.to_string()])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "delayn" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (DelayN::new(*v as usize).to_boxed_nodedata(2), vec![])
                },
                GlicolPara::Reference(s) => {
                    (DelayN::new(0).to_boxed_nodedata(2), vec![s.to_string()])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "delayms" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (DelayMs::new().sr(sr).delay(*v).to_boxed_nodedata(2), vec![])
                },
                GlicolPara::Reference(s) => {
                    (DelayMs::new().sr(sr).delay(2000.).to_boxed_nodedata(2), vec![s.to_string()])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "noise" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (Noise::new(*v as usize).to_boxed_nodedata(1), vec![])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "speed" => get_one_para_from_number_or_ref!(Speed),
        "onepole" => get_one_para_from_number_or_ref!(OnePole),
        "add" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (Add::new(*v).to_boxed_nodedata(2), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Add::new(0.0).to_boxed_nodedata(2), vec![s.to_string()])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "constsig" => {
            match &paras[0] {
                GlicolPara::Number(v) => {
                    (ConstSig::new(*v).to_boxed_nodedata(1), vec![])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        // todo: give sr to them
        "bd" => get_one_para_from_number_or_ref2!(Bd),
        "hh" => get_one_para_from_number_or_ref2!(Hh),
        "sn" => get_one_para_from_number_or_ref2!(Sn),
        "sawsynth" => {
            let data = SawSynth::new(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                },
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(2);
            (data, vec![])
        },
        "squsynth" => {
            let data = SquSynth::new(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                },
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(2);
            (data, vec![])
        },
        "trisynth" => {
            let data = TriSynth::new(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                },
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(2);
            (data, vec![])
        },
        "get" => {
            let mut reflist = Vec::<String>::new();
            match &paras[0] {
                GlicolPara::Reference(s) => {
                    reflist.push(s.to_string())
                },
                _ => unimplemented!()
            }
            ( NodeData::new2( BoxedNodeSend::new(Pass{}) ), reflist)
        },
        "seq" => {
            let mut reflist = Vec::<String>::new();
            let events = match &paras[0] {
                GlicolPara::Sequence(s) => s,
                _ => unimplemented!(),
            };
            let mut order = HashMap::new();
            let mut count = 0;
            for event in events {
                match &event.1 {
                    GlicolPara::Reference(s) => { // reflist: ["~a", "~b", "~a"]
                        if !reflist.contains(&s) {
                            reflist.push(s.to_string());
                            order.insert(s.to_string(), count);
                            count += 1;
                        }
                    },
                    _ => {}
                }
            }
            (Sequencer::new(events.clone()).sr(sr).bpm(bpm).ref_order(order).to_boxed_nodedata(2), reflist)
        },
        "choose" => {
            let list = match &paras[0] {
                GlicolPara::NumberList(v) => {
                    v
                },
                _ => unimplemented!()
            };
            (Choose::new(list.clone(), seed as u64).to_boxed_nodedata(2), vec![])
        },
        "mix" => {
            let list: Vec<_> = paras.iter().map(|x|match x  {
                GlicolPara::Reference(s) => {
                    (*s).clone()
                },
                _ => unimplemented!()
            }).collect();
            ( NodeData::new2( BoxedNodeSend::new(Sum2{})), list)
        },
        // "sendpass" => {
        //     let reflist = match &paras[0] {
        //         GlicolPara::RefList(v) => {
        //             v
        //         },
        //         _ => unimplemented!()
        //     };
        //     ( Pass{}.to_boxed_nodedata(2), reflist)
        // },
        _ => unimplemented!()
    };
    return Ok((nodedata, reflist))
}