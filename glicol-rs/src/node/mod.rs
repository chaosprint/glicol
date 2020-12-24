pub mod adc; pub mod operator; pub mod sequencer; pub mod envelope;
pub mod filter; pub mod oscillator; pub mod sampler; pub mod noise;
pub mod pass; pub mod map; pub mod rand; pub mod phasor;
pub mod buf; pub mod state; pub mod pan; pub mod delay;
pub mod system; pub mod reverb; pub mod source;
use std::{collections::HashMap};
use super::{Pairs, Rule, NodeResult};

// use dasp_graph::{Buffer, Input, Node};
// use super::{Engine, NodeData, BoxedNodeSend, EngineError, ndef};

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

pub fn make_node(
    name: &str,
    mut paras: &mut Pairs<Rule>,
    samples_dict: &HashMap<String, &'static[f32]>,
    sr: f32,
    bpm: f32,
) -> NodeResult {

    match name {
        "sin" => SinOsc::new(&mut paras),
        "mul" => Mul::new(&mut paras),
        "add" => Add::new(&mut paras),
        "imp" => Impulse::new(&mut paras),
        "sampler" => Sampler::new(&mut paras, 
            samples_dict),
        "seq" => Sequencer::new(&mut paras, sr, bpm),
        "linrange" => LinRange::new(&mut paras),
        "saw" => Saw::new(&mut paras),
        "squ" => Square::new(&mut paras),
        "lpf" => LPF::new(&mut paras),
        "hpf" => HPF::new(&mut paras),
        "speed" => Speed::new(&mut paras),
        "noiz" => Noise::new(&mut paras),
        "choose" => Choose::new(&mut paras),
        "envperc" => EnvPerc::new(&mut paras),
        "pha" => Phasor::new(&mut paras),
        "buf" => Buf::new(&mut paras, 
            samples_dict),
        "state" => State::new(&mut paras),
        "pan" => Pan::new(&mut paras),
        "delay" => Delay::new(&mut paras),
        "apf" => Allpass::new(&mut paras),
        "comb" => Comb::new(&mut paras),
        "mix" => Mix2::new(&mut paras),
        "plate" => Plate::new(&mut paras),
        "onepole" => OnePole::new(&mut paras),
        "allpass" => AllpassGain::new(&mut paras),
        "delayn" => DelayN::new(&mut paras),
        "monosum" => MonoSum::new(&mut paras),
        "const" => ConstSig::new(&mut paras),
        _ => Pass::new(name)
    }
}