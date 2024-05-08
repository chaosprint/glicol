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
    Node, Pass, Sum2
};

use glicol_parser::{ToInnerOwned as _, nodes::{self, Component, NumberOrRef}};

#[cfg(feature = "use-meta")]
use glicol_synth::dynamic::Meta;

use glicol_synth::dynamic::Eval;

#[cfg(feature = "use-samples")]
use glicol_synth::sampling::{PSampler, Sampler};

use crate::EngineError;
use glicol_synth::{BoxedNodeSend, NodeData}; //, Processor, Buffer, Input, Node
use hashbrown::HashMap;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;

#[allow(unused_variables, unused_mut)]
pub fn makenode<const N: usize>(
    component: &Component<'_>,
    samples_dict: &HashMap<String, (&'static [f32], usize, usize)>,
    sr: usize,
    bpm: f32,
    seed: usize,
) -> Result<(GlicolNodeData<N>, Vec<String>), EngineError> {
    let (nodedata, reflist) = match component {
        #[cfg(feature = "use-samples")]
        Component::PSampler(psampler) => {
            let mut samples_dict_selected = HashMap::new();
            let (pattern, span) = match psampler {
                nodes::PSampler::Event(_) => panic!("An event inside PSampler is not yet supported; please file an issue if you encounter this"),
                nodes::PSampler::Pattern(ref pat) => (&pat.event, pat.span)
            };

            let pattern = pattern.val_times.iter().map(|(val, time)| {
                let value = match &val {
                    nodes::EventValue::Number(_) => String::new(),
                    nodes::EventValue::Symbol(sym) => sym.to_string(),
                };

                if !samples_dict.contains_key(&value) {
                    return Err(EngineError::NonExistSample(value.clone()));
                } else {
                    samples_dict_selected.insert(value.clone(), samples_dict[&value]);
                }

                Ok((value, *time))
            }).collect::<Result<Vec<_>, EngineError>>()?;

            (
                PSampler::new(samples_dict_selected, sr, bpm, vec![], pattern, span).to_boxed_nodedata(2),
                vec![],
            )
        }
        Component::Points(nodes::Points { points, span, is_looping }) => (
            Points::new()
                .bpm(bpm)
                .sr(sr)
                .span(*span)
                .points(points.to_vec())
                .is_looping(*is_looping)
                .to_boxed_nodedata(1),
            vec![],
        ),
        Component::MsgSynth(nodes::MsgSynth { symbol, attack, decay }) => (
            MsgSynth::new()
                .sr(sr)
                .attack(*attack)
                .decay(*decay)
                .to_boxed_nodedata(1),
            vec![],
        ),
        Component::PatternSynth(nodes::PatternSynth { symbol, p2 }) => {
            let pattern = symbol.replace('`', "");
            let events = pattern.split(',')
                .map(|event| {
                    let mut result = event
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.replace(' ', "").parse::<f32>().unwrap());

                    (result.next().unwrap(), result.next().unwrap())
                }).collect();

            (
                PatternSynth::new(events).sr(sr).to_boxed_nodedata(1),
                vec![],
            )
        }

        #[cfg(feature = "bela")]
        Component::Adc(nodes::Adc { port }) => (
            NodeData::new1(BoxedNodeSend::new(Pass {})),
            vec![format!("~adc{}", port + 1)],
        ),

        #[cfg(feature = "use-samples")]
        Component::Sp(nodes::Sp { sample_sym }) => {
            let Some(sample) = samples_dict.get(*sample_sym) else {
                return Err(EngineError::NonExistSample(sample_sym.to_string()));
            };

            (
                Sampler::new(*sample, sr).to_boxed_nodedata(2),
                vec![],
            )
        }

        #[cfg(feature = "use-meta")]
        Component::Meta(nodes::Meta { code }) => (
            Meta::new().sr(sr).code(code.code).to_boxed_nodedata(1),
            vec![],
        ),
        // "expr" => {
        //     match &paras[0] {
        //         GlicolPara::Symbol(s) => {
        //             (Expr::new().sr(sr).code(s.to_owned()).to_boxed_nodedata(1), vec![])
        //         },
        //         _ => unimplemented!()
        //     }
        // },
        Component::Eval(nodes::Eval { code }) => (
            Eval::new().sr(sr).code(code.code).to_boxed_nodedata(1),
            vec![],
        ),
        Component::Lpf(nodes::Lpf { signal, qvalue }) => {
            let mut reflist = vec![];
            let data = match signal {
                nodes::ConstSig::Number(v) => ResonantLowPassFilter::new()
                    .cutoff(*v)
                    .q(*qvalue)
                    .sr(sr)
                    .to_boxed_nodedata(1),
                nodes::ConstSig::Reference(s) => {
                    reflist.push(s.to_string());
                    ResonantLowPassFilter::new()
                        .q(*qvalue)
                        .sr(sr)
                        .to_boxed_nodedata(1)
                }
                nodes::ConstSig::Pattern(nodes::Pattern { event, span }) => {
                    let pattern = event.val_times.iter()
                        .map(|(val, time)| {
                            let value = match val {
                                nodes::EventValue::Number(num) => *num,
                                nodes::EventValue::Symbol(_) => 100.0,
                            };
                            (value, *time)
                        }).collect();

                    // println!("pattern {:?}", pattern);
                    ResonantLowPassFilter::new()
                        .q(*qvalue)
                        .pattern(pattern)
                        .span(*span)
                        .bpm(bpm)
                        .sr(sr)
                        .to_boxed_nodedata(1)
                }
                nodes::ConstSig::Event(_) => panic!("An event as a parameter to lpf is not currently supported")
            };
            (data, reflist)
        }
        Component::Balance(nodes::Balance { left, right }) => {
            let data = Balance::new().to_boxed_nodedata(2);
            let reflist = vec![left.to_string(), right.to_string()];
            (data, reflist)
        }
        Component::Rhpf(nodes::Rhpf { cutoff, qvalue }) => {
            let data = ResonantHighPassFilter::new()
                .cutoff(match cutoff {
                    nodes::NumberOrRef::Number(v) => *v,
                    nodes::NumberOrRef::Ref(_) => 100.0,
                })
                .q(*qvalue)
                .to_boxed_nodedata(1);

            let mut reflist = vec![];
            if let nodes::NumberOrRef::Ref(s) = cutoff {
                reflist.push(s.to_string());
            };
            (data, reflist)
        }
        Component::ApfmsGain(nodes::ApfmsGain { delay, gain }) => {
            let data = AllPassFilterGain::new()
                .sr(sr)
                .delay(match delay {
                    nodes::NumberOrRef::Number(v) => *v,
                    nodes::NumberOrRef::Ref(_) => 0.0,
                })
                .gain(*gain)
                .to_boxed_nodedata(2);

            let mut reflist = vec![];
            if let nodes::NumberOrRef::Ref(s) = delay {
                reflist.push(s.to_string());
            };
            (data, reflist)
        }
        // "reverb" => {
        //     let data = Reverb::new().sr(sr).to_boxed_nodedata(2);
        //     let reflist = vec![];
        //     (data, reflist)
        // },
        Component::EnvPerc(nodes::EnvPerc { attack, decay }) => (
            EnvPerc::new()
                .sr(sr)
                .attack(*attack)
                .decay(*decay)
                .to_boxed_nodedata(2),
            vec![]
        ),
        Component::Adsr(nodes::Adsr { attack, decay, sustain, release }) => (
            Adsr::new()
                .sr(sr)
                .attack(*attack)
                .decay(*decay)
                .sustain(*sustain)
                .release(*release)
                .to_boxed_nodedata(2),
            vec![]
        ),
        Component::Tri(nodes::Tri { param }) => match param {
            nodes::NumberOrRef::Number(v) => (TriOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            nodes::NumberOrRef::Ref(s) => (
                TriOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_string()],
            ),
        },
        Component::Squ(nodes::Squ { param }) => match param {
            nodes::NumberOrRef::Number(v) => (SquOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            nodes::NumberOrRef::Ref(s) => (
                SquOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_string()],
            ),
        },
        Component::Saw(nodes::Saw { param }) => match param {
            nodes::NumberOrRef::Number(v) => (SawOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            nodes::NumberOrRef::Ref(s) => (
                SawOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_string()],
            ),
        },
        Component::Sin(nodes::Sin { param }) => match param {
            nodes::NumberOrRef::Number(v) => (SinOsc::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            nodes::NumberOrRef::Ref(s) => (
                SinOsc::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_string()],
            ),
        },
        Component::Plate(nodes::Plate { mix }) => (
            Plate::new(*mix).to_boxed_nodedata(2), vec![]
        ),
        Component::Imp(nodes::Imp { param }) => match param {
            nodes::NumberOrRef::Number(v) => (Impulse::new().sr(sr).freq(*v).to_boxed_nodedata(1), vec![]),
            nodes::NumberOrRef::Ref(s) => (
                Impulse::new().sr(sr).freq(0.0).to_boxed_nodedata(1),
                vec![s.to_string()],
            ),
        },
        Component::Mul(nodes::Mul { param }) => match param {
            nodes::NumberOrRef::Number(v) => (Mul::new(*v).to_boxed_nodedata(2), vec![]),
            nodes::NumberOrRef::Ref(s) => (Mul::new(0.0).to_boxed_nodedata(2), vec![s.to_string()]),
        },
        Component::Pan(nodes::Pan { param }) => match param {
            nodes::NumberOrRef::Number(v) => (Pan::new(*v).to_boxed_nodedata(2), vec![]),
            nodes::NumberOrRef::Ref(s) => (Pan::new(0.0).to_boxed_nodedata(2), vec![s.to_string()]),
        },
        Component::Delayn(nodes::Delayn { param }) => match param {
            nodes::UsizeOrRef::Usize(v) => (DelayN::new(*v).to_boxed_nodedata(2), vec![]),
            nodes::UsizeOrRef::Ref(s) => (DelayN::new(0).to_boxed_nodedata(2), vec![s.to_string()]),
        },
        Component::Delayms(nodes::Delayms { param }) => match param {
            nodes::NumberOrRef::Number(v) => (
                DelayMs::new().sr(sr).delay(*v, 2).to_boxed_nodedata(2),
                vec![],
            ),
            nodes::NumberOrRef::Ref(s) => (
                DelayMs::new().sr(sr).delay(2000., 2).to_boxed_nodedata(2),
                vec![s.to_string()],
            ),
        },
        Component::Noise(nodes::Noise { seed }) => (
            Noise::new(*seed).to_boxed_nodedata(1), vec![]
        ),
        Component::Speed(nodes::Speed { param }) => get_one_para_from_number_or_ref::<N, Speed>(param, 1),
        Component::Onepole(nodes::Onepole { param }) => get_one_para_from_number_or_ref::<N, OnePole>(param, 1),
        Component::Add(nodes::Add { param }) => get_one_para_from_number_or_ref::<N, Add>(param, 2),
        Component::ConstSig(sig) => (
            match sig {
                nodes::ConstSig::Number(v) => ConstSig::new(*v).sr(sr).to_boxed_nodedata(1),
                nodes::ConstSig::Pattern(nodes::Pattern { event, span }) => {
                    let pattern = event.val_times.iter()
                        .map(|(val, times)| {
                            let value = match val {
                                nodes::EventValue::Number(num) => *num,
                                nodes::EventValue::Symbol(_) => 100.0,
                            };
                            (value, *times)
                        }).collect();

                    // println!("pattern {:?}", pattern);
                    ConstSig::new(0.0)
                        .pattern(pattern)
                        .span(*span)
                        .bpm(bpm)
                        .sr(sr)
                        .to_boxed_nodedata(1)
                }
                nodes::ConstSig::Reference(_) | nodes::ConstSig::Event(_) =>
                    panic!("constsig does not yet support using a reference or an event as the parameter"),
            },
            vec![]
        ),
        // todo: give sr to them
        Component::Bd(nodes::Bd { param }) => get_one_para_from_number_or_ref::<N, Bd<N>>(param, 2),
        Component::Hh(nodes::Hh { param }) => get_one_para_from_number_or_ref::<N, Hh<N>>(param, 2),
        Component::Sn(nodes::Sn { param }) => get_one_para_from_number_or_ref::<N, Sn<N>>(param, 2),
        Component::SawSynth(nodes::SawSynth { attack, decay }) => (
            SawSynth::new(*attack, *decay).to_boxed_nodedata(2),
            vec![]
        ),
        Component::SquSynth(nodes::SquSynth { attack, decay }) => (
            SquSynth::new(*attack, *decay).to_boxed_nodedata(2),
            vec![]
        ),
        Component::TriSynth(nodes::TriSynth { attack, decay }) => (
            TriSynth::new(*attack, *decay).to_boxed_nodedata(2),
            vec![]
        ),
        Component::Get(nodes::Get { reference }) => (
            NodeData::new2(BoxedNodeSend::new(Pass {})),
            vec![reference.to_string()]
        ),
        Component::Seq(nodes::Seq { events }) => {
            let mut reflist = Vec::<String>::new();
            let mut order = HashMap::new();
            let mut count = 0;
            for event in events {
                if let NumberOrRef::Ref(s) = &event.1 {
                    // reflist: ["~a", "~b", "~a"]
                    if !reflist.iter().any(|r| r == s) {
                        reflist.push(s.to_string());
                        order.insert(s.to_string(), count);
                        count += 1;
                    }
                }
            }
            (
                Sequencer::new(events.to_inner_owned())
                    .sr(sr)
                    .bpm(bpm)
                    .ref_order(order)
                    .to_boxed_nodedata(2),
                reflist,
            )
        }
        Component::Choose(nodes::Choose { choices }) => (
            Choose::new(choices.clone(), seed as u64).to_boxed_nodedata(2),
            vec![],
        ),
        Component::Mix(nodes::Mix { nodes }) => (
            NodeData::new2(BoxedNodeSend::new(Sum2 {})),
            nodes.iter().map(ToString::to_string).collect()
        ),
        Component::Arrange(nodes::Arrange { events }) => {
            let reflist = events
                .iter()
                .flat_map(|ev| match ev {
                    nodes::NumberOrRef::Number(_) => None,
                    nodes::NumberOrRef::Ref(s) => Some(s.to_string())
                })
                .collect();

            (
                Arrange::new(events.to_inner_owned())
                    .sr(sr)
                    .bpm(bpm)
                    .to_boxed_nodedata(2),
                reflist,
            )
        },
        Component::Reverb(_)
        | Component::Expr(_) => panic!("{component:?} is currently not supported within the engine"),
        #[cfg(not(feature = "use-samples"))]
        Component::Sp(_) | Component::PSampler(_) => panic!("The `use-samples` feature is required to use the `sp` or `psampler` node"),
        #[cfg(not(feature = "bela"))]
        Component::Adc(_) => panic!("The `bela` feature is required to use the `adc` node"),
        #[cfg(not(feature = "use-meta"))]
        Component::Meta(_) => panic!("The `use-meta` feature is required to use the `meta` node"),

        // "sendpass" => {
        //     let reflist = match &paras[0] {
        //         GlicolPara::RefList(v) => {
        //             v
        //         },
        //         _ => unimplemented!()
        //     };
        //     ( Pass{}.to_boxed_nodedata(2), reflist)
        // },
    };
    Ok((nodedata, reflist))
}

fn get_one_para_from_number_or_ref<const N: usize, T>(
    param: &nodes::NumberOrRef<&str>,
    channels: usize
) -> (NodeData<BoxedNodeSend<N>, N>, Vec<String>)
where
    T: From<f32> + Node<N> + Send + 'static,
{
    match param {
        nodes::NumberOrRef::Number(v) => (T::from(*v).to_boxed_nodedata(channels), vec![]),
        nodes::NumberOrRef::Ref(s) => (T::from(0.0).to_boxed_nodedata(channels), vec![(*s).to_owned()]),
    }
}
