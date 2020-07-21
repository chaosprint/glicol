use std::{collections::HashMap, iter::Cloned, slice::Iter};

use dasp_signal::{self as signal, Signal, Sine, ConstHz};
use dasp_slice::add_in_place;
// use dasp::interpolate::linear::Linear;
// use dasp_signal::{self as signal, Signal, FromIterator, interpolate::linear::Linear, interpolate::Converter};
// use dasp_interpolate::linear::Linear;
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNode, BoxedNodeSend};

use petgraph;
use petgraph::graph::{NodeIndex};

pub struct SinOsc {
    // pub freq: f64,
    pub sig: Sine<ConstHz>
}

impl SinOsc {
    pub fn new(freq: f64) -> SinOsc {
        let sig = signal::rate(48000.0).const_hz(freq).sine();
        SinOsc {
            sig: sig
        }
    }
}

impl Node for SinOsc {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {

        for o in output {
            o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
    }
}

pub struct Engine {
    // pub chains: HashMap<String, Vec<Box<dyn Node + 'static + Send >>>,
    pub phase: usize,
    pub graph: petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>,
    pub processor: dasp_graph::Processor<petgraph::graph::DiGraph<NodeData<BoxedNodeSend>, (), u32>>,
    // pub synth: Synth,
    pub node: Vec<NodeIndex>,
    pub sr: f32,
    pub bpm: f32,
}

impl Engine {
    pub fn new() -> Engine {
        // Chose a type of graph for audio processing.
        type Graph = petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>;
        // Create a short-hand for our processor type.
        type Processor = dasp_graph::Processor<Graph>;
        // Create a graph and a processor with some suitable capacity to avoid dynamic allocation.
        let max_nodes = 512; // if 1024, error, 512 is fine
        let max_edges = 512;
        let g = Graph::with_capacity(max_nodes, max_edges);
        let p = Processor::with_capacity(max_nodes);

        Engine {
            // chains: HashMap::<String, Vec<Box<dyn Node + 'static + Send >>>::new(), // a hashmap of Box<AsFunc>
            graph: g,
            processor: p,
            node: Vec::new(),
            phase: 0,
            sr: 44100.0,
            bpm: 120.0,
        }
    }

    fn generate_wave_buf(&mut self, size:usize) -> [f32; 128] {
        let mut output: [f32; 128] = [0.0; 128];

        for n in self.node.clone() {
            self.processor.process(&mut self.graph, n);
            let b = &self.graph[n].buffers[0];
            for i in 0..64 {
                output[i] += b[i]
            }
        }

        for n in self.node.clone() {
            self.processor.process(&mut self.graph, n);
            let b = &self.graph[n].buffers[0];
            for i in 64..128 {
                output[i] += b[i-64]
            }
        }

        // for i in 0..size {
            // let original_chains = self.chains.clone();
            // for (_, chain) in &mut self.chains {
                // output[i] += chain.yield_currentx_sample(self.phase) as f32;
                // output[i] += chain.fold("", |a, b| b.run(a)).next() as f32 * 0.0625;
                // output[i] += sig.yield_current_sample(self.phase, self.bpm) * 0.0625;
                // output[i] += chain[0].next() as f32 * 0.1;
            // }
            // output[i] = (2.0 * PI * 440.0 * self.phase as f32 / 44100.0).sin();
            // self.phase += 1;
        // }
        output
    }

    pub fn process(&mut self, out_ptr: *mut f32, size: usize) {
        let wave_buf = self.generate_wave_buf(size);
        let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
        for i in 0..size {
            out_buf[i] = wave_buf[i] as f32
        }
    }
}

// pub trait QuaverFunc<T: Signal<Frame=f64>, K: Signal<Frame=f64>> {
//     fn connect_left(self, b: T) -> K;
// }

// pub struct QuaverSinOsc {
//     pub sig: Sine<ConstHz>
// }

// impl QuaverSinOsc {
//     pub fn new(freq: f64) -> QuaverSinOsc {
//         let sig = signal::rate(48000.0).const_hz(freq).sine();
//         QuaverSinOsc {
//             sig: sig
//         }
//     }
// }

// impl QuaverFunc for QuaverSinOsc {
//     fn connect_left(self, b: Box<dyn Signal<Frame=f64>>) -> Box<dyn Signal<Frame=f64>> {
//         Box::new(self.sig)
//     }
// }

// pub struct QuaverSigMul {
//     pub mul: f64
// }

// impl QuaverSigMul {
//     pub fn new(mul: f64) -> QuaverSigMul {
//         QuaverSigMul {
//             mul
//         }
//     }
// }

// impl<T, K> QuaverFunc<T, K> for QuaverSigMul where
//     T: Signal<Frame=f64>,
//     K: Singal<Frame=f64> {
//     fn connect_left(self, ) -> {
//     }
// }

// #[derive(Iterator)]
// pub struct QuaverSignal {
//     pub functions: Vec<Box<dyn QuaverFunction + 'static + Send>>,
//     pub phase: usize,
//     pub bpm: f64,
//     // sig: dasp::signal::GenMut<f64, f64>
//     // sig: dasp::signal::Signal::GenMut<(dyn dasp::signal::Signal + 'static)>
//     // sig: dasp::signal::Signal
//     // sig: dasp::signal::Sine<
//     //     dasp::signal::Hz<dasp::signal::Sine<
//     //     dasp::signal::ConstHz>>>
//     sig: dasp::signal::Sine<
//         dasp::signal::ConstHz>,
//     // pub sig: dasp::signal::MulAmp<dasp::signal::Sine<dasp::signal::ConstHz>, dasp::signal::Sine<dasp::signal::ConstHz>>
//     // pub buffer: [f32; 128] in the future

//     // pub event: Vec<Event>,
//     // pub ins: Vec<Box<dyn Instrument + 'static + Send>>,
// }

// impl QuaverSignal {
//     pub fn new() -> QuaverSignal {

//         // let mut f = -1.0;
//         // let mut sig = signal::gen_mut(|| {
//         //     let r = f;
//         //     // f[0] = (f[0] * f[0] - 1.85) * 0.5;
//         //     f = (f * f - 1.85) * 0.5;
//         //     r
//         // });
//         // let sig = signal::rate(48000.0).hz(signal::rate(48000.0).const_hz(20.0).sine()).sine();
//         // let sig_a = signal::rate(48000.0).const_hz(20.0).sine();
//         let sig = signal::rate(48000.0).const_hz(200.0).sine();
//         // let a = source.next();
//         // let b = source.next();
//         // let interp = Linear::new(a, b);
//         // let sig = sig_a.mul_amp(sig_b);

//         QuaverSignal {
//             functions: Vec::new(),
//             phase: 0,
//             bpm: 120.0,
//             sig: sig,
//             // buffer: [0.0; 128]
//         }
//     }

//     pub fn update_bpm(mut self, bpm: f64) {
//         self.bpm = bpm
//     }
//     pub fn output_current_sample(&mut self, phase: usize) -> f64 { 
//         // 0.0
//         self.sig.next()
//     }
// }

// impl Iterator for QuaverSignal {
//     type Item = f64;
//     fn next(&mut self) -> Option::<f64> {
//         self.phase += 1;
//         Some(self.output_current_sample(self.phase))
//     }
// }

// pub struct Event {
//     pub relative_time: f64,
//     pub pitch: f64,
//     // event need to know the signal for calling next()
//     // pub sig: Vec<Converter<FromIterator<Cloned<Iter<'static, [f32; 1]>>>, Linear<[f32; 1]>>>
// }

// impl Event {
//     pub fn new() -> Event {
//         Event {
//             relative_time: 0.0,
//             pitch: -1.0,
//             // sig: Vec::new()
//         }
//     }
// }

// pub struct QuaverLoop {
//     pub events: Vec<Event>
// }

// impl QuaverLoop {
//     pub fn new() -> QuaverLoop {
//         QuaverLoop {
//             events: Vec::new()
//         }
//     }
// }

        // let (sender, receiver) = unbounded();

        // let mut rack = Rack::new(vec![]);

        // let sine = SineOsc::new().hz(1).rack(&mut rack);
        // let modulator = Modulator::new(sine.tag())
        //     .base_hz(440)
        //     .mod_hz(220)
        //     .mod_idx(1)
        //     .rack(&mut rack);

        // let square = SquareOsc::new().hz(modulator.tag()).rack(&mut rack);
        
        // Lpf::new(square.tag()).cutoff_freq(880).rack(&mut rack);

        // let synth = Synth { rack };


// impl QuaverFunction for QuaverLoop {
//     fn run(self) {
//         // sig.event = self;
//         // sig
//     }
// }

// pub trait QuaverFunction<T, K> {
//     fn modify(self, lhs: T) -> K; // take, process, and return a signal
// }


// impl QuaverFunction<T, K> for Event 
// where T: String, K: QuaverSignal {
//     fn modify(self, lhs: T) -> K {
//         qs.event = self;
//         qs
//     }
// }



// ******** loop >> speed >> sampler ***********

// pub struct Seq {
//     event: Vec<Event>
// }

// pub struct SeqModifier {
//     speed: f64,
//     shift: f64
// }

// pub struct SignalChain;

// ****** Trait ******

// pub trait SeqRHS<C> {
//     fn wrap_lhs_seq(&self, seq: Seq) -> C;
// }

// impl SeqRHS<Seq> for SeqModifier {
//     fn wrap_lhs_seq(&self, seq: Seq) -> Seq {
//        seq
//     }
// }

// impl SeqRHS<SignalChain> for SignalChain {
//     fn wrap_lhs_seq(&self, seq: Seq) -> SignalChain {
//        Self
//     }
// }

// ******** end of Seq ***********

// pub trait Function {
//     // fn run<T>(&self) -> T; // T can be f32, f64...
//     fn connect_left<C: Function>(&self, signal: Signal) -> C; // C can be SeqMod or 
// }

// .fold(0, |sum, x| sum + x);

// .fold(signal_struct, |func_chain, next_function| next_function.connect_left(func_chain))