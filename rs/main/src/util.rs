use glicol_synth::{
    oscillator::{SinOsc, SquOsc, TriOsc, SawOsc},
    filter::{ResonantLowPassFilter, OnePole, AllPassFilterGain},
    signal::{ConstSig, Impulse, Noise},
    operator::{Mul, Add},
    sampling::Sampler,
    delay::{DelayN, DelayMs},
    sequencer::{Sequencer, Choose, Speed},
    envelope::EnvPerc,
    effect::{Plate},
    compound::{Bd, Hh, Sn, SawSynth, SquSynth, TriSynth},
    dynamic::Meta,
    Pass,
};

use glicol_synth::{NodeData, BoxedNodeSend, GlicolPara, HashMap}; //, Processor, Buffer, Input, Node
use glicol_macros::get_one_para_from_number_or_ref;
use crate::EngineError;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
// pub type NodeResult<const N: usize> = Result<(GlicolNodeData<N>, Vec<String>), GlicolError>;

pub fn makenode<const N: usize>(
    name: &str,
    paras: &mut Vec<GlicolPara<'static>>,
    // pos: (usize, usize),
    samples_dict: &std::collections::HashMap<&'static str, (&'static[f32], usize)>,
    sr: usize,
    bpm: f32,
    seed: usize
) -> Result<(GlicolNodeData<N>, Vec<&'static str>), EngineError> {
    let (nodedata, reflist) = match name {
        "sp" => {
            match paras[0] {
                GlicolPara::SampleSymbol(s) => {
                    if !samples_dict.contains_key(s) {
                        return Err(EngineError::NonExsitSample(s.to_owned()))
                    }
                    (Sampler::new(samples_dict[s]).to_boxed_nodedata(2), vec![])
                }
                _ => unimplemented!()
            }
        },
        "meta" => {
            match paras[0] {
                GlicolPara::Symbol(s) => {
                    (Meta::new().sr(sr).code(s).to_boxed_nodedata(2), vec![])
                },
                _ => unimplemented!()
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
        },
        "apfmsgain" => {
            let data = AllPassFilterGain::new().sr(sr).delay(
                match paras[0] {
                    GlicolPara::Number(v) => v,
                    GlicolPara::Reference(_) => 0.0,
                    _ => unimplemented!()
                }
            ).gain(
                match paras[1] {
                    GlicolPara::Number(v) => v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(2);

            let mut reflist = vec![];
            match paras[0] {
                GlicolPara::Reference(s) => reflist.push(s),
                _ => {}
            };
            (data, reflist)
        },
        "envperc" => {
            let data = EnvPerc::new().sr(sr).attack(
                match paras[0] {
                    GlicolPara::Number(v) => v,
                    _ => unimplemented!()
                }
            ).decay(
                match paras[1] {
                    GlicolPara::Number(v) => v,
                    _ => unimplemented!()
                }
            ).to_boxed_nodedata(2);
            let reflist = vec![];
            (data, reflist)
        },
        "tri" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (TriOsc::new().sr(sr).freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (TriOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "squ" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (SquOsc::new().sr(sr).freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SquOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "saw" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (SawOsc::new().sr(sr).freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SawOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "sin" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (SinOsc::new().sr(sr).freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (SinOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "plate" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (Plate::new(v).to_boxed_nodedata(2), vec![])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "imp" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (Impulse::new().sr(sr).freq(v).to_boxed_nodedata(1), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Impulse::new().sr(sr).freq(0.0).to_boxed_nodedata(1), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
            
        },
        "mul" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (Mul::new(v).to_boxed_nodedata(2), vec![])
                },
                GlicolPara::Reference(s) => {
                    (Mul::new(0.0).to_boxed_nodedata(2), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "delayn" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (DelayN::new(v as usize).to_boxed_nodedata(2), vec![])
                },
                GlicolPara::Reference(s) => {
                    (DelayN::new(0).to_boxed_nodedata(2), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "delayms" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (DelayMs::new().sr(sr).delay(v).to_boxed_nodedata(2), vec![])
                },
                GlicolPara::Reference(s) => {
                    (DelayMs::new().sr(sr).delay(2000.).to_boxed_nodedata(2), vec![s])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "noise" => {
            match paras[0] {
                GlicolPara::Number(v) => {
                    (Noise::new(v as usize).to_boxed_nodedata(1), vec![])
                },
                _ => {
                    unimplemented!();
                }
            }
        },
        "speed" => get_one_para_from_number_or_ref!(Speed),
        "onepole" => get_one_para_from_number_or_ref!(OnePole),
        "add" => get_one_para_from_number_or_ref!(Add),
        "constsig" => get_one_para_from_number_or_ref!(ConstSig),
        // todo: give sr to them
        "bd" => get_one_para_from_number_or_ref!(Bd),
        "hh" => get_one_para_from_number_or_ref!(Hh),
        "sn" => get_one_para_from_number_or_ref!(Sn),
        "sawsynth" => get_one_para_from_number_or_ref!(SawSynth),
        "squsynth" => get_one_para_from_number_or_ref!(SquSynth),
        "trisynth" => get_one_para_from_number_or_ref!(TriSynth),
        "get" => {
            let mut reflist = Vec::<&str>::new();
            match paras[0] {
                GlicolPara::Reference(s) => {
                    reflist.push(s)
                },
                _ => unimplemented!()
            }
            ( NodeData::new2( BoxedNodeSend::new(Pass{}) ), reflist)
        },
        "seq" => {
            let mut reflist = Vec::<&str>::new();
            let events = match &paras[0] {
                GlicolPara::Sequence(s) => s,
                _ => unimplemented!(),
            };
            let mut order = HashMap::new();
            let mut count = 0;
            for event in events {
                match event.1 {
                    GlicolPara::Reference(s) => { // reflist: ["~a", "~b", "~a"]
                        if !reflist.contains(&s) {
                            reflist.push(&s);
                            order.insert(s, count);
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