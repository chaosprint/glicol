pub mod system; use super::*;
pub mod sin_osc; use sin_osc::SinOsc;
pub mod saw_osc; use saw_osc::SawOsc;
pub mod squ_osc; use squ_osc::SquOsc;
pub mod tri_osc; use tri_osc::TriOsc;
pub mod imp; use imp::*;
pub mod const_sig; use const_sig::ConstSig;
pub mod noise; use noise::Noise;
pub mod mul; use mul::Mul;
pub mod add; use add::Add;
pub mod filter; use filter::lpf::*; use filter::hpf::*;
pub mod seq; use seq::*;
pub mod sampler; use sampler::*;
pub mod speed; use speed::*;
pub mod pass; use pass::*;
pub mod choose; use choose::*;
pub mod delayn; use delayn::*;
pub mod delay; use delay::*;
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
        "noiz" => "noise",
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
            // check potential errors
            if !samples_dict.contains_key(&paras.as_str().replace("\\", "")) {
                let p = paras.next().unwrap();
                let pos = (p.as_span().start(), p.as_span().end());
                return Err(EngineError::SampleNotExistError(pos))
            }
            vec![]
        }, // bypass the process_parameters
        "seq" => vec![],
        "choose" => { vec![] },
        "delayn" => vec![Para::Number(1.0)],
        "delay" => vec![Para::Modulable],
        _ => vec![Para::Modulable], // pass
    };

    // this func checks if the parameters are correct
    let (p, mut refs) = process_parameters(paras, modulable)?;

    if name == "seq" {refs = process_seq(paras.as_str())?.2}
    
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

        "noise" => noise!(get_num(&p[0]) as u64),
        "imp" => imp!({freq: get_num(&p[0]), sr: sr}),
        "sampler" => {
            sampler!(samples_dict[&paras.as_str().replace("\\", "")])},
        "seq" => {
            let info = process_seq(paras.as_str()).unwrap();
            seq!({events: info.0, sidechain_lib: info.1, sr: sr, bpm: bpm})
        }
        "speed" => speed!(get_num(&p[0])),
        "choose" => choose!(get_notes(paras)?),
        "delayn" => delayn!(get_num(&p[0]) as usize),
        "delay" => delay!({delay: get_num(&p[0]), sr: sr}),
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

type Events = Vec::<(f64, String)>;
type Sidechain = HashMap::<String, usize>;

fn process_seq(pattern: &str) -> Result<(Events, Sidechain, Vec<String>), EngineError> {
    let mut events = Vec::<(f64, String)>::new();
    let mut sidechain_count = 0;
    let mut sidechains = Vec::new();
    let mut sidechain_lib = Sidechain::new();
    let split: Vec<&str> = pattern.split(" ").collect();
    let len_by_space = split.len();
    let compound_unit = 1.0 / len_by_space as f64;

    for (i, compound) in split.iter().enumerate() {
        let c = compound.replace("_", "$_$");
        let notes = c.split("$").filter(|x|x!=&"").collect::<Vec<_>>();

        let notes_len = notes.len();
        for (j, x) in notes.iter().enumerate() {
            let relative_time = i as f64 / len_by_space as f64 
            + (j as f64/ notes_len as f64 ) * compound_unit;

            if x.contains("~") {
                sidechains.push(x.to_string());
                sidechain_lib.insert(x.to_string(), sidechain_count);
                sidechain_count += 1;
            }

            if x != &"_" {
                events.push((relative_time, x.to_string()))
            }
        }
    }
    Ok((events, sidechain_lib, sidechains))
}

fn get_notes(paras: &mut Pairs<Rule>) -> Result<Vec::<f32>, EngineError> {
    let split: Vec<&str> = paras.as_str().split(" ").collect();
    let mut note_list = Vec::<f32>::new();
    println!("split{:?}", split);
    for note in split {
        match note.parse::<f32>() {
            Ok(v) => note_list.push(v),
            Err(_) => {
                let p = paras.next().unwrap();
                let pos = (p.as_span().start(), p.as_span().end());
                return Err(EngineError::ParameterError(pos))
            }
        }
    }
    println!("note_list{:?}", note_list);
    Ok(note_list)
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

