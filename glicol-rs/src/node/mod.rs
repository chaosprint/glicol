pub mod sin_osc; use sin_osc::SinOsc;
pub mod const_sig; use const_sig::ConstSig;
pub mod mul; use mul::Mul;
pub mod add; use add::Add;
pub mod system;
use super::*;
// pub mod adc;
// pub mod operator;
// pub mod sequencer;
// pub mod envelope;
// pub mod filter; 
// pub mod sampler; 
// pub mod noise;
// pub mod pass;
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
// use sampler::{Sampler};
// use sequencer::{Sequencer, Speed};
// use envelope::EnvPerc;
// use noise::Noise;
// use pass::Pass;
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

    let modulable = match name {
        "sin" => vec![Para::Modulable],
        "const" => vec![Para::Number(0.0)],
        "mul" => vec![Para::Modulable],
        "add" => vec![Para::Modulable],
        "sampler" => vec![], // bypass the process_parameters
        _ => unimplemented!()
    };

    // this func checks if the parameters are correct
    let (p, refs) = process_parameters(paras, modulable)?;

    let nodedata = match name {
        "sin" => sin_osc!({freq: get_num(&p[0]), sr: sr}),
        "const" => const_sig!(get_num(&p[0])),
        "mul" => mul!(get_num(&p[0])),
        "add" => add!(get_num(&p[0])),
        // "lpf" => lpf!{cutoff: &p[0], q: &p[1]},
        // "mul" => Mul::new(&p[0]),
        // "add" => Add::new(&p[0]),
        // "imp" => Impulse::new(&p[0])?,
        // "sp" => Sampler::new(paras.as_str(), 
        //     samples_dict)?,
        // "sampler" => Sampler::new(paras.as_str(), 
        //     samples_dict)?,
        // "seq" => Sequencer::new(&mut paras, sr, bpm)?,
        _ => unimplemented!()
        // "imp" => Impulse::new(&mut paras)?,
        // "sp" => Sampler::new(&mut paras, 
        //     samples_dict)?,
        // "sampler" => Sampler::new(&mut paras, 
        //     samples_dict)?,
        // "buf" => Buf::new(&mut paras, 
        //     samples_dict)?,
        // "seq" => Sequencer::new(&mut paras, sr, bpm)?,
        // "linrange" => LinRange::new(&mut paras)?,
        // "saw" => Saw::new(&mut paras)?,
        // "squ" => Square::new(&mut paras)?,
        // "lpf" => LPF::new(&mut paras)?,
        // "hpf" => HPF::new(&mut paras)?,
        // "spd" => Speed::new(&mut paras)?,
        // "speed" => Speed::new(&mut paras)?,
        // "noiz" => Noise::new(&mut paras)?,
        // "choose" => Choose::new(&mut paras)?,
        // "envperc" => EnvPerc::new(30.0, 50.0)?,
        // "pha" => Phasor::new(&mut paras)?,
        // "state" => State::new(&mut paras)?,
        // "pan" => Pan::new(&mut paras)?,
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

