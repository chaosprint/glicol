use glicol_synth::{
    compound::{Bd, Hh, SawSynth, Sn, SquSynth, TriSynth},
    delay::{DelayMs, DelayN},
    effect::{Balance, Pan, Plate},
    envelope::{Adsr, EnvPerc},
    filter::{AllPassFilterGain, OnePole, ResonantHighPassFilter, ResonantLowPassFilter},
    operator::{Add, Mul},
    oscillator::{SawOsc, SinOsc, SquOsc, TriOsc},
    sequencer::{Arrange, Choose, Sequencer, Speed},
    signal::{ConstSig, Impulse, Noise, Points},
    synth::{MsgSynth, PatternSynth},
    Node, Pass, Sum2,
};

#[cfg(feature = "use-meta")]
use glicol_synth::dynamic::Meta;

use glicol_synth::dynamic::Eval;

#[cfg(feature = "use-samples")]
use glicol_synth::sampling::{PSampler, Sampler};

use crate::EngineError;
use glicol_synth::{BoxedNodeSend, GlicolPara, NodeData}; //, Processor, Buffer, Input, Node
use hashbrown::HashMap;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
// pub type NodeResult<const N: usize> = Result<(GlicolNodeData<N>, Vec<String>), GlicolError>;

#[allow(unused_variables, unused_mut)]
pub fn makenode<const N: usize>(
    name: &str,
    paras: &mut [GlicolPara],
    // pos: (usize, usize),
    samples_dict: &HashMap<String, (&'static [f32], usize, usize)>,
    sr: usize,
    bpm: f32,
    seed: usize,
) -> Result<(GlicolNodeData<N>, Vec<String>), EngineError> {
    let (nodedata, reflist) = match name {
        #[cfg(feature = "use-samples")]
        "psampler" => {
            let pattern_info = match &paras[0] {
                GlicolPara::Pattern(pattern, span) => (pattern, span),
                _ => unimplemented!(),
            };
            let mut samples_dict_selected = HashMap::new();

            let pattern = (*pattern_info.0)
                .iter()
                .map(|v| {
                    let value = match &v.0 {
                        GlicolPara::Number(_) => "".to_owned(),
                        GlicolPara::Symbol(s) => s.to_string(),
                        _ => unimplemented!(),
                    };
                    let time = v.1;
                    if !samples_dict.contains_key(&value) {
                        return Err(EngineError::NonExsitSample(value.clone()));
                    } else {
                        samples_dict_selected.insert(value.clone(), samples_dict[&value]);
                    }

                    Ok((value, time))
                })
                .collect::<Result<Vec<_>, EngineError>>()?;

            let span = *pattern_info.1;

            (
                PSampler::new(samples_dict_selected, sr, bpm, vec![], pattern, span)
                    .to_boxed_nodedata(2),
                vec![],
            )
        }

        "points" => {
            let points = paras[0].clone();
            let span = match &paras[1] {
                GlicolPara::Number(v) => *v,
                _ => unimplemented!(),
            };
            let is_looping = match &paras[2] {
                GlicolPara::Bool(v) => *v,
                _ => unimplemented!(),
            };
            (
                Points::new()
                    .bpm(bpm)
                    .sr(sr)
                    .span(span)
                    .points(points)
                    .is_looping(is_looping)
                    .to_boxed_nodedata(1),
                vec![],
            )
        }

        "msgsynth" => (
            MsgSynth::new()
                .sr(sr)
                .attack(match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .decay(match &paras[2] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .to_boxed_nodedata(1),
            vec![],
        ),
        "pattern_synth" => {
            match &paras[0] {
                GlicolPara::Symbol(s) => {
                    let pattern = s.replace('`', "");
                    let events = pattern
                        .split(',')
                        .map(|event| {
                            // println!("event {:?}", event);
                            let mut result = event
                                .split(' ')
                                .filter(|x| !x.is_empty())
                                .map(|x| x.replace(' ', "").parse::<f32>().unwrap());

                            // println!("result {:?}", result);
                            (result.next().unwrap(), result.next().unwrap())
                        })
                        .collect();

                    (
                        PatternSynth::new(events).sr(sr).to_boxed_nodedata(1),
                        vec![],
                    )
                }
                _ => unimplemented!(),
            }
        }

        #[cfg(feature = "bela")]
        "adc" => {
            let port = match &paras[0] {
                GlicolPara::Number(v) => *v as usize,
                _ => unimplemented!(),
            };
            (
                NodeData::new1(BoxedNodeSend::new(Pass {})),
                vec![format!("~adc{}", port + 1)],
            )
        }

        #[cfg(feature = "use-samples")]
        "sp" => match &paras[0] {
            GlicolPara::SampleSymbol(s) => {
                if !samples_dict.contains_key(s) {
                    return Err(EngineError::NonExsitSample(s.to_owned()));
                }
                (
                    Sampler::new(samples_dict[s], sr).to_boxed_nodedata(2),
                    vec![],
                )
            }
            _ => unimplemented!(),
        },

        #[cfg(feature = "use-meta")]
        "meta" => match &paras[0] {
            GlicolPara::Symbol(s) => (
                Meta::new().sr(sr).code(s.to_owned()).to_boxed_nodedata(1),
                vec![],
            ),
            _ => unimplemented!(),
        },
        // "expr" => {
        //     match &paras[0] {
        //         GlicolPara::Symbol(s) => {
        //             (Expr::new().sr(sr).code(s.to_owned()).to_boxed_nodedata(1), vec![])
        //         },
        //         _ => unimplemented!()
        //     }
        // },
        "eval" => match &paras[0] {
            GlicolPara::Symbol(s) => (
                Eval::new().sr(sr).code(s.to_owned()).to_boxed_nodedata(1),
                vec![],
            ),
            _ => unimplemented!(),
        },
        "lpf" => {
            let qvalue = match &paras[1] {
                GlicolPara::Number(v) => *v,
                _ => unimplemented!(),
            };
            let mut reflist = vec![];
            let data = match &paras[0] {
                GlicolPara::Number(v) => ResonantLowPassFilter::new()
                    .cutoff(*v)
                    .q(qvalue)
                    .sr(sr)
                    .to_boxed_nodedata(1),
                GlicolPara::Reference(s) => {
                    reflist.push(s.to_owned());
                    ResonantLowPassFilter::new()
                        .q(qvalue)
                        .sr(sr)
                        .to_boxed_nodedata(1)
                }
                GlicolPara::Pattern(events, span) => {
                    let pattern = events
                        .iter()
                        .map(|v| {
                            let value = match v.0 {
                                GlicolPara::Number(num) => num,
                                _ => 100.0,
                            };
                            (value, v.1)
                        })
                        .collect();

                    // println!("pattern {:?}", pattern);
                    ResonantLowPassFilter::new()
                        .q(qvalue)
                        .pattern(pattern)
                        .span(*span)
                        .bpm(bpm)
                        .sr(sr)
                        .to_boxed_nodedata(1)
                }
                _ => unimplemented!(),
            };
            (data, reflist)
        }
        "balance" => {
            let data = Balance::new().to_boxed_nodedata(2);
            let reflist = vec![
                match &paras[0] {
                    GlicolPara::Reference(s) => s.to_owned(),
                    _ => "".to_owned(),
                },
                match &paras[1] {
                    GlicolPara::Reference(s) => s.to_owned(),
                    _ => "".to_owned(),
                },
            ];
            (data, reflist)
        }
        "rhpf" => {
            let data = ResonantHighPassFilter::new()
                .cutoff(match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    GlicolPara::Reference(_) => 100.0,
                    _ => unimplemented!(),
                })
                .q(match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .to_boxed_nodedata(1);

            let mut reflist = vec![];
            if let GlicolPara::Reference(s) = &paras[0] {
                reflist.push(s.to_owned());
            };
            (data, reflist)
        }
        "apfmsgain" => {
            let data = AllPassFilterGain::new()
                .sr(sr)
                .delay(match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    GlicolPara::Reference(_) => 0.0,
                    _ => unimplemented!(),
                })
                .gain(match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .to_boxed_nodedata(2);

            let mut reflist = vec![];
            if let GlicolPara::Reference(s) = &paras[0] {
                reflist.push(s.to_owned());
            };
            (data, reflist)
        }
        // "reverb" => {
        //     let data = Reverb::new().sr(sr).to_boxed_nodedata(2);
        //     let reflist = vec![];
        //     (data, reflist)
        // },
        "envperc" => {
            let data = EnvPerc::new()
                .sr(sr)
                .attack(match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .decay(match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .to_boxed_nodedata(2);
            let reflist = vec![];
            (data, reflist)
        }
        "adsr" => {
            let data = Adsr::new()
                .sr(sr)
                .attack(match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .decay(match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .sustain(match &paras[2] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .release(match &paras[3] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                })
                .to_boxed_nodedata(2);
            let reflist = vec![];
            (data, reflist)
        }
        "tri" => match &paras[0] {
            GlicolPara::Number(v) => (TriOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            GlicolPara::Reference(s) => (
                TriOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_owned()],
            ),
            _ => {
                unimplemented!();
            }
        },
        "squ" => match &paras[0] {
            GlicolPara::Number(v) => (SquOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            GlicolPara::Reference(s) => (
                SquOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_owned()],
            ),
            _ => {
                unimplemented!();
            }
        },
        "saw" => match &paras[0] {
            GlicolPara::Number(v) => (SawOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            GlicolPara::Reference(s) => (
                SawOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_owned()],
            ),
            _ => {
                unimplemented!();
            }
        },
        "sin" => match &paras[0] {
            GlicolPara::Number(v) => (SinOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            GlicolPara::Reference(s) => (
                SinOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_owned()],
            ),
            _ => {
                unimplemented!();
            }
        },
        "plate" => match &paras[0] {
            GlicolPara::Number(v) => (Plate::new(*v).to_boxed_nodedata(2), vec![]),
            _ => {
                unimplemented!();
            }
        },
        "imp" => match &paras[0] {
            GlicolPara::Number(v) => (Impulse::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            GlicolPara::Reference(s) => (
                Impulse::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_owned()],
            ),
            _ => {
                unimplemented!();
            }
        },
        "mul" => match &paras[0] {
            GlicolPara::Number(v) => (Mul::new(*v).to_boxed_nodedata(2), vec![]),
            GlicolPara::Reference(s) => (Mul::new(0.0).to_boxed_nodedata(2), vec![s.to_string()]),
            _ => {
                unimplemented!();
            }
        },
        "pan" => match &paras[0] {
            GlicolPara::Number(v) => (Pan::new(*v).to_boxed_nodedata(2), vec![]),
            GlicolPara::Reference(s) => (Pan::new(0.0).to_boxed_nodedata(2), vec![s.to_string()]),
            _ => {
                unimplemented!();
            }
        },
        "delayn" => match &paras[0] {
            GlicolPara::Number(v) => (DelayN::new(*v as usize).to_boxed_nodedata(2), vec![]),
            GlicolPara::Reference(s) => (DelayN::new(0).to_boxed_nodedata(2), vec![s.to_string()]),
            _ => {
                unimplemented!();
            }
        },
        "delayms" => match &paras[0] {
            GlicolPara::Number(v) => (
                DelayMs::new().sr(sr).delay(*v, 2).to_boxed_nodedata(2),
                vec![],
            ),
            GlicolPara::Reference(s) => (
                DelayMs::new().sr(sr).delay(2000., 2).to_boxed_nodedata(2),
                vec![s.to_string()],
            ),
            _ => {
                unimplemented!();
            }
        },
        "noise" => match &paras[0] {
            GlicolPara::Number(v) => (Noise::new(*v as usize).to_boxed_nodedata(1), vec![]),
            _ => {
                unimplemented!();
            }
        },
        "speed" => get_one_para_from_number_or_ref::<N, Speed>(paras, 1),
        "onepole" => get_one_para_from_number_or_ref::<N, OnePole>(paras, 1),
        "add" => match &paras[0] {
            GlicolPara::Number(v) => (Add::new(*v).to_boxed_nodedata(2), vec![]),
            GlicolPara::Reference(s) => (Add::new(0.0).to_boxed_nodedata(2), vec![s.to_string()]),
            _ => {
                unimplemented!();
            }
        },
        "constsig" => {
            let mut reflist = vec![];
            let data = match &paras[0] {
                GlicolPara::Number(v) => ConstSig::new(*v).sr(sr).to_boxed_nodedata(1),
                GlicolPara::Pattern(events, span) => {
                    let pattern = events
                        .iter()
                        .map(|v| {
                            let value = match v.0 {
                                GlicolPara::Number(num) => num,
                                _ => 100.0,
                            };
                            (value, v.1)
                        })
                        .collect();

                    // println!("pattern {:?}", pattern);
                    ConstSig::new(0.0)
                        .pattern(pattern)
                        .span(*span)
                        .bpm(bpm)
                        .sr(sr)
                        .to_boxed_nodedata(1)
                }
                _ => unimplemented!(),
            };
            (data, reflist)
        }
        // todo: give sr to them
        "bd" => get_one_para_from_number_or_ref::<N, Bd<N>>(paras, 2),
        "hh" => get_one_para_from_number_or_ref::<N, Hh<N>>(paras, 2),
        "sn" => get_one_para_from_number_or_ref::<N, Sn<N>>(paras, 2),
        "sawsynth" => {
            let data = SawSynth::new(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                },
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                },
            )
            .to_boxed_nodedata(2);
            (data, vec![])
        }
        "squsynth" => {
            let data = SquSynth::new(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                },
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                },
            )
            .to_boxed_nodedata(2);
            (data, vec![])
        }
        "trisynth" => {
            let data = TriSynth::new(
                match &paras[0] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                },
                match &paras[1] {
                    GlicolPara::Number(v) => *v,
                    _ => unimplemented!(),
                },
            )
            .to_boxed_nodedata(2);
            (data, vec![])
        }
        "get" => {
            let mut reflist = Vec::<String>::new();
            match &paras[0] {
                GlicolPara::Reference(s) => reflist.push(s.to_string()),
                _ => unimplemented!(),
            }
            (NodeData::new2(BoxedNodeSend::new(Pass {})), reflist)
        }
        "seq" => {
            let mut reflist = Vec::<String>::new();
            let events = match &paras[0] {
                GlicolPara::Sequence(s) => s,
                _ => unimplemented!(),
            };
            let mut order = HashMap::new();
            let mut count = 0;
            for event in events {
                if let GlicolPara::Reference(s) = &event.1 {
                    // reflist: ["~a", "~b", "~a"]
                    if !reflist.contains(s) {
                        reflist.push(s.to_string());
                        order.insert(s.to_string(), count);
                        count += 1;
                    }
                }
            }
            (
                Sequencer::new(events.clone())
                    .sr(sr)
                    .bpm(bpm)
                    .ref_order(order)
                    .to_boxed_nodedata(2),
                reflist,
            )
        }
        "choose" => {
            let list = match &paras[0] {
                GlicolPara::NumberList(v) => v,
                _ => unimplemented!(),
            };
            (
                Choose::new(list.clone(), seed as u64).to_boxed_nodedata(2),
                vec![],
            )
        }
        "mix" => {
            let list: Vec<_> = paras
                .iter()
                .map(|x| match x {
                    GlicolPara::Reference(s) => (*s).clone(),
                    _ => unimplemented!(),
                })
                .collect();
            (NodeData::new2(BoxedNodeSend::new(Sum2 {})), list)
        }
        "arrange" => {
            let mut reflist = vec![];
            for p in paras.iter() {
                if let GlicolPara::Reference(s) = p {
                    reflist.push((*s).clone());
                }
            }
            (
                Arrange::new(paras.to_vec())
                    .sr(sr)
                    .bpm(bpm)
                    .to_boxed_nodedata(2),
                reflist,
            )
        }

        // "sendpass" => {
        //     let reflist = match &paras[0] {
        //         GlicolPara::RefList(v) => {
        //             v
        //         },
        //         _ => unimplemented!()
        //     };
        //     ( Pass{}.to_boxed_nodedata(2), reflist)
        // },
        _ => unimplemented!(),
    };
    Ok((nodedata, reflist))
}

fn get_one_para_from_number_or_ref<const N: usize, T>(
    paras: &[GlicolPara],
    channels: usize,
) -> (NodeData<BoxedNodeSend<N>, N>, Vec<String>)
where
    T: From<f32> + Node<N> + Send + 'static,
{
    match &paras[0] {
        GlicolPara::Number(v) => (T::from(*v).to_boxed_nodedata(channels), vec![]),
        GlicolPara::Reference(s) => (T::from(0.0).to_boxed_nodedata(channels), vec![s.to_owned()]),
        _ => {
            unimplemented!();
        }
    }
}
