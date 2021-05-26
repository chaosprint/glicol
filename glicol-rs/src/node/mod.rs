pub mod sin_osc; use sin_osc::SinOsc; pub mod saw_osc; use saw_osc::SawOsc;
pub mod squ_osc; use squ_osc::SquOsc; pub mod tri_osc; use tri_osc::TriOsc;
pub mod const_sig; use const_sig::ConstSig; pub mod noise; use noise::Noise;
pub mod mul; use mul::Mul; pub mod add; use add::Add;
pub mod filter; use filter::lpf::*; use filter::hpf::*;
pub mod imp; use imp::*;
pub mod system; use super::*;
pub mod seq; use seq::*;
pub mod sampler; use sampler::*;
pub mod speed; use speed::*;
pub mod pass; use pass::*;

// pub mod adc;
// pub mod operator;
// pub mod envelope;
// pub mod filter; 
// pub mod map; 
// pub mod rand; 
// pub mod phasor;
// pub mod buf; 
// pub mod state; 
// pub mod pan; 
// pub mod delay;
// pub mod reverb;

// use operator::*;
// use phasor::{Phasor};
// use envelope::EnvPerc;
// use filter::{LPF, HPF, Allpass, Comb, OnePole, AllpassGain};
// use map::{LinRange};
// use rand::{Choose};
// use buf::{Buf};
// use state::{State};
// use pan::{Pan, Mix2};
// use delay::{Delay, DelayN};
// use reverb::{Plate};

/// This function handles the parameters and pass back a result (nodedata, refs)
/// There are several steps to check if the parameters are valid
/// 1. total numbers
/// 2. if moduable?
pub fn make_node(
    name: &str,
    paras: &mut Pairs<Rule>,
    samples_dict: &HashMap<String, &'static[f32]>,
    sr: usize,
    bpm: f32,
) -> NodeResult {
    
    let alias = match name {
        "sp" => "sampler",
        "*" => "mul",
        _ => name
    };

    let modulable = match alias {
        "sin" => vec![Para::Modulable],
        "saw" => vec![Para::Modulable],
        "squ" => vec![Para::Modulable],
        "tri" => vec![Para::Modulable],
        "const" => vec![Para::Number(0.0)],
        "mul" => vec![Para::Modulable],
        "add" => vec![Para::Modulable],
        "lpf" => vec![Para::Modulable, Para::Number(1.0)],
        "hpf" => vec![Para::Modulable, Para::Number(1.0)],
        "sampler" => {
            if !samples_dict.contains_key(&paras.as_str().replace("\\", "")) {
                let p = paras.next().unwrap();
                let pos = (p.as_span().start(), p.as_span().end());
                return Err(EngineError::SampleNotExistError(pos))
            }
            vec![]
        }, // bypass the process_parameters
        "seq" => vec![], // do no comsume paras
        _ => vec![Para::Modulable], // pass
    };

    // this func checks if the parameters are correct
    let (p, refs) = process_parameters(paras, modulable)?;
    
    let nodedata = match alias {
        "sin" => sin_osc!({freq: get_num(&p[0]), sr: sr}),
        "saw" => saw_osc!({freq: get_num(&p[0]), sr: sr}),
        "squ" => squ_osc!({freq: get_num(&p[0]), sr: sr}),
        "tri" => tri_osc!({freq: get_num(&p[0]), sr: sr}),
        "const" => const_sig!(get_num(&p[0])),
        "mul" => mul!(get_num(&p[0])),
        "add" => add!(get_num(&p[0])),
        "lpf" => rlpf!({cutoff: get_num(&p[0]), q: get_num(&p[1])}),
        "hpf" => rhpf!({cutoff: get_num(&p[0]), q: get_num(&p[1])}),
        "noiz" => noise!(get_num(&p[0]) as u64),
        "noise" => noise!(get_num(&p[0]) as u64),
        "imp" => imp!({freq: get_num(&p[0]), sr: sr}),
        "sampler" => {
            println!("samplers{:?}", samples_dict[&paras.as_str().replace("\\", "")]);
            sampler!(samples_dict[&paras.as_str().replace("\\", "")])},
        "seq" => {
            seq!({pattern: paras.as_str(), sr: sr, bpm: bpm})
        },
        "speed" => speed!(get_num(&p[0])),
        _ => Pass::new()

        // "choose" => Choose::new(&mut paras)?,
        // "envperc" => EnvPerc::new(30.0, 50.0)?,
        // "pan" => Pan::new(&mut paras)?,
        // "buf" => Buf::new(&mut paras, 
        //     samples_dict)?,
        // "linrange" => LinRange::new(&mut paras)?,
        // "pha" => Phasor::new(&mut paras)?,
        // "state" => State::new(&mut paras)?,
        // "delay" => Delay::new(&mut paras)?,
        // "apf" => Allpass::new(&mut paras)?,
        // "comb" => Comb::new(&mut paras)?,
        // "mix" => Mix2::new(&mut paras)?,
        // "plate" => Plate::new(&mut paras)?,
        // "onepole" => OnePole::new(&mut paras)?,
        // "allpass" => AllpassGain::new(&mut paras)?,
        // "delayn" => DelayN::new(&mut paras)?,
        // "monosum" => MonoSum::new(&mut paras)?,
        // _ => Pass::new(name)?
    };
    Ok((nodedata, refs))
}


#[derive(Debug, Clone, PartialEq)]
/// Parameter of a node can be f32, String, or NodeIndex for sidechain
pub enum Para {
    Number(f32),
    Symbol(String),
    Index(NodeIndex),
    Modulable
}

fn get_num(p: &Para) -> f32 {
    match p {
        Para::Number(v) => *v,
        Para::Modulable => 0.0,
        _ => 0.0
    }
}

fn get_string(p: Para) -> String {
    match p {
        Para::Symbol(v) => v,
        _ => unimplemented!()
    }
}

#[macro_export]
macro_rules! mono_node {
    ($body:expr) => {
        NodeData::new1( BoxedNodeSend::new(($body)))
    };
}

#[macro_export]
macro_rules! ndef {
    ($struct_name: ident, $channel_num: ident, {$code_str: expr}) => {
        pub struct $struct_name {
            engine: Engine
        }
        
        impl $struct_name {
            pub fn new(paras: &mut Pairs<Rule>) -> Result<
            (NodeData<BoxedNodeSend<128>, 128>, Vec<String>), EngineError> {
                let mut engine = Engine::new();
                engine.set_code(&format!($code_str, a=paras.as_str()));
                engine.make_graph()?;
                Ok((NodeData::$channel_num(BoxedNodeSend::new( Self {
                    engine
                })), vec![]))
            }
        }
        
        impl Node<128> for $struct_name {
            fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
                // self.engine.input(inputs); // mono or stereo?
                let mut input = inputs[0].buffers()[0].clone();
                let buf = self.engine.gen_next_buf_128(&mut input).unwrap();
                match output.len() {
                    1 => {
                        for i in 0..128 {
                            output[0][i] = buf.0[i];
                        }
                    },
                    2 => {
                        for i in 0..128 {
                            output[0][i] = buf.0[i];
                            output[1][i] = buf.0[i+128];
                        }
                    },
                    _ => {}
                }
            }
        }
    };
}

