pub mod adc; pub mod operator; pub mod sequencer; pub mod envelope;
pub mod filter; pub mod oscillator; pub mod sampler; pub mod noise;
pub mod pass; pub mod map; pub mod rand; pub mod phasor;
pub mod buf; pub mod state; pub mod pan; pub mod delay;
pub mod system; pub mod reverb; pub mod source;
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