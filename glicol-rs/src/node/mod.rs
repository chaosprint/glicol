// pub mod adc;
pub mod operator;
pub mod sequencer;
pub mod envelope;
pub mod filter; 
pub mod oscillator; 
pub mod sampler; 
pub mod noise;
pub mod pass;
pub mod map; 
pub mod rand; 
pub mod phasor;
pub mod buf; 
pub mod state; 
pub mod pan; 
pub mod delay;
pub mod system;
pub mod reverb;
pub mod source;
use std::{collections::HashMap};
use super::{Pairs, Rule, NodeResult};

use phasor::{Phasor};
use oscillator::{SinOsc, Impulse, Saw, Square};
use operator::{Add, Mul, MonoSum};
use sampler::{Sampler};
use sequencer::{Sequencer, Speed};
use envelope::EnvPerc;
use noise::Noise;
use pass::Pass;
use filter::{LPF, HPF, Allpass, Comb, OnePole, AllpassGain};
use map::{LinRange};
use rand::{Choose};
use buf::{Buf};
use state::{State};
use pan::{Pan, Mix2};
use delay::{Delay, DelayN};
use reverb::{Plate};
use source::{ConstSig};
// use synth::{Synth};

pub fn make_node(
    name: &str,
    mut paras: &mut Pairs<Rule>,
    samples_dict: &HashMap<String, &'static[f32]>,
    sr: f32,
    bpm: f32,
) -> NodeResult {

    let result = match name {
        "sin" => SinOsc::new(&mut paras)?,
        "*" => Mul::new(&mut paras)?,
        "mul" => Mul::new(&mut paras)?,
        "add" => Add::new(&mut paras)?,
        "imp" => Impulse::new(&mut paras)?,
        "sp" => Sampler::new(&mut paras, 
            samples_dict)?,
        "sampler" => Sampler::new(&mut paras, 
            samples_dict)?,
        "buf" => Buf::new(&mut paras, 
            samples_dict)?,
        "seq" => Sequencer::new(&mut paras, sr, bpm)?,
        "linrange" => LinRange::new(&mut paras)?,
        "saw" => Saw::new(&mut paras)?,
        "squ" => Square::new(&mut paras)?,
        "lpf" => LPF::new(&mut paras)?,
        "hpf" => HPF::new(&mut paras)?,
        "spd" => Speed::new(&mut paras)?,
        "speed" => Speed::new(&mut paras)?,
        "noiz" => Noise::new(&mut paras)?,
        "choose" => Choose::new(&mut paras)?,
        "envperc" => EnvPerc::new(&mut paras)?,
        "pha" => Phasor::new(&mut paras)?,
        "state" => State::new(&mut paras)?,
        "pan" => Pan::new(&mut paras)?,
        "delay" => Delay::new(&mut paras)?,
        "apf" => Allpass::new(&mut paras)?,
        "comb" => Comb::new(&mut paras)?,
        "mix" => Mix2::new(&mut paras)?,
        "plate" => Plate::new(&mut paras)?,
        "onepole" => OnePole::new(&mut paras)?,
        "allpass" => AllpassGain::new(&mut paras)?,
        "delayn" => DelayN::new(&mut paras)?,
        "monosum" => MonoSum::new(&mut paras)?,
        "const" => ConstSig::new(&mut paras)?,
        // "synth" => Synth::new(&mut paras)?,
        _ => Pass::new(name)?
    };
    Ok(result)
    // match result {
    //     Ok(good) => Ok(good),
    //     Err(e) => {
    //         match e {
    //             // EngineError::NonExistControlNodeError(na) => Err(e),
    //             EngineError::HandleNodeError => Err(e),
    //             EngineError::ParameterError  => Err(e),
    //             EngineError::SampleNotExistError(_pos) => Err(e),
    //             _ => unimplemented!()
    //         }
    //     }
    // }
}

#[macro_export]
/// this works well for nodes whose inner states are only floats
/// e.g. oscillator, filter, operator
macro_rules! handle_params {
    (
        { $($id: ident: $default: expr),* }
        $(,{$( $extra_params: ident : $val: expr),* })?
        $(,[$( ( $related: ident, $extra_id: ident, $handler: expr) ),* ])?
    ) => {
        pub fn new(paras: &mut Pairs<Rule>) ->
        NodeResult {

            let mut sidechains = Vec::<String>::new();
            let mut params_val = std::collections::HashMap::<&str, f32>::new();
            let mut sidechain_ids = Vec::<u8>::new();
            let mut _sidechain_id: u8 = 0;

            // TODO: need to handle unwarp
            $(
                let current_param: String = paras.next().unwrap().as_str().to_string();
                let parse_result = current_param.parse::<f32>();
                match parse_result {
                    Ok(val) => {
                        params_val.insert(stringify!($id), val);
                    },
                    Err(_) => {
                        sidechains.push(current_param);
                        params_val.insert(stringify!($id), $default);
                        sidechain_ids.push(_sidechain_id);
                    }
                };
                _sidechain_id += 1;
            )*

            $(
                $(
                    let $extra_id = $handler(params_val[stringify!($related)]);
                )*
            )?

            Ok((NodeData::new1( BoxedNodeSend::new( Self {
                $(
                    $id: params_val[stringify!($id)],
                )*
                $(
                    $(
                        $extra_params: $val,
                    )*
                )?
                $(
                    $(
                        $extra_id,
                    )*
                )?
                sidechain_ids
            })), sidechains))
        }
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