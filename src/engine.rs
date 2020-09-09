use std::{collections::HashMap};
// , iter::Cloned, slice::Iter

use pest::Parser;
#[derive(Parser)]
#[grammar = "quaver.pest"]
pub struct QParser;

use dasp_signal::{self as signal, Signal};
// , Sine, ConstHz, GenMut
use dasp_slice::{ToFrameSlice};
// add_in_place, 
// use dasp::slice::;
use dasp_interpolate::linear::Linear;
// use dasp_signal::{self as signal, Signal, FromIterator, interpolate::linear::Linear, interpolate::Converter};
// use dasp_interpolate::linear::Linear;
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
// BoxedNode
// use std::f64::consts::PI;

use petgraph;
use petgraph::graph::{NodeIndex};

pub struct SinOsc {
    // pub freq: f64,
    // pub sig: Sine<ConstHz>
    pub sig: Box<dyn Signal<Frame=f64> + Send>,
}

impl SinOsc {
    pub fn new(freq: f64) -> SinOsc {
        let sig = signal::rate(44100.0).const_hz(freq).sine();
        SinOsc {
            sig: Box::new(sig)
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

pub struct Mul {
    pub mul: String
}
impl Mul {
    pub fn new(mul: String) -> Mul {
        Mul { mul }
    }
}
impl Node for Mul {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        let num = self.mul.parse::<f64>();
        if num.is_ok() {
            if inputs.len() > 0 {
                let buf = &mut inputs[0].buffers();
                output[0] = buf[0].clone(); // write
                output[0].iter_mut().for_each(|s| *s = *s * num.clone().unwrap() as f32);
            }
        } else {
            if inputs.len() > 1 {
                let buf = &mut inputs[0].buffers();
                let mod_buf = &mut inputs[1].buffers();
                // output[0] = buf[0].clone();
                output[0].clone_from_slice(&buf[0]);
                // for i in 0..output[0].len() {
                for i in 0..64 {
                    output[0][i] *= mod_buf[0][i];
                    // output[0].iter_mut().for_each(|s| *s = *s * 0.9 as f32);
                }
                
            }
        }
    }
}

pub struct Add {
    pub add: f64
}
impl Add {
    pub fn new(add: f64) -> Add {
        Add { add }
    }
}
impl Node for Add {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        if inputs.len() > 0 {
            let buf = &mut inputs[0].buffers();
            output[0] = buf[0].clone();
            output[0].iter_mut().for_each(|s| *s = *s + self.add as f32);
        }
    }
}


pub struct Sampler {
    pub sig: Vec< Box<dyn Signal<Frame=[f32;1]> + 'static + Send>>,
    // pub sig: Box<dyn Signal<Frame=[f32;1]> + Send>,
    pub samples: &'static[f32],
    // pub length: u32,
}

impl Sampler {
    pub fn new(samples: &'static[f32]) -> Self {
        // let f: &[[f32;1]] = samples.to_frame_slice().unwrap();
        // let mut source = signal::from_iter(f.iter().cloned());
        // let a = source.next();
        // let b = source.next();
        // let interp = Linear::new(a, b);
        // let s = source.scale_hz(interp, 1.5 );
        Self {
            sig: Vec::new(),
            // sig: Box::new(s)
            samples
        }
    }
}

impl Node for Sampler {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        output[0].silence();
        if inputs.len() > 0 {
            // the input of sampler should be a pitch, and series of 0
            let input_buf = &mut inputs[0].buffers();

            for i in 0..64 {
                if input_buf[0][i] > 0.0 {
                    // do it every sample, will it be too expensive?
                    let f: &[[f32;1]] = self.samples.to_frame_slice().unwrap();
                    // let s = signal::from_iter(f.iter().cloned());
                    let mut source = signal::from_iter(f.iter().cloned());
                    let a = source.next();
                    let b = source.next();
                    let interp = Linear::new(a, b);
                    let s = source.scale_hz(interp, input_buf[0][i] as f64);
                    // as f64 /2.0_f64.powf((60.0-69.0)/12.0)/440.0;
                    self.sig.push(Box::new(s));
                }
                // for i in 0..output[0].len() {
                for v in &mut self.sig {
                    if !v.is_exhausted() {
                        output[0][i] += v.next()[0];
                    }                   
                }
            }
        }
    }
}

pub struct Impulse {
    sig: Box<dyn Signal<Frame=f32> + Send>,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

impl Impulse {
    pub fn new(freq: f64) -> Self {
        let p = (44100.0 / freq) as usize;
        let mut i: usize = 0;
        let s = signal::gen_mut(move || {
            let imp = (i % p == 0) as u8;
            i += 1;
            imp as f32
        });
        Self {
            sig: Box::new(s)
        }
    }
}

impl Node for Impulse {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for o in output {
            o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
        // output[0].iter_mut().for_each(|s| *s = self.sig.next());
    }
}

pub struct Looper {
    sig: Box<dyn Signal<Frame=f32> + Send>,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

impl Looper {
    pub fn new(events: Vec<(f64, f64)>) -> Self {
        // let p = (44100.0 / 10.0) as usize;
        let mut i: usize = 0;
        let s = signal::gen_mut(move || {
            let mut output: f32 = 0.0;

            for event in &events {
                let relative_time = event.0;
                let relative_pitch = event.1;

                // bpm should be somewhere here
                if i % 44100 == (relative_time * 44100.0) as usize {
                    // this it the sampler to trigger
                    output = relative_pitch as f32;
                }
            }
            // let imp = (i % p == 0) as u8;
            i += 1;
            output
        });
        Self {
            sig: Box::new(s)
        }
    }
}

impl Node for Looper {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for o in output {
            o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
        // output[0].iter_mut().for_each(|s| *s = self.sig.next());
    }
}

// pub struct Control {
//     pub bpm: f64,
//     pub elapsed_samples: usize,
// }

pub struct Engine {
    // pub chains: HashMap<String, Vec<Box<dyn Node + 'static + Send >>>,
    pub elapsed_samples: usize,
    pub graph: petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>,
    // pub graph_: Box<petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>>,
    pub processor: dasp_graph::Processor<petgraph::graph::DiGraph<NodeData<BoxedNodeSend>, (), u32>>,
    // pub synth: Synth,
    pub nodes: HashMap<String, NodeIndex>,
    pub samples_dict: HashMap<String, &'static[f32]>,
    // pub nodes_: HashMap<String, NodeIndex>,
    pub sr: f64,
    pub bpm: f64,
    pub code: String,
    pub update: bool,
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
        // let g_ = Graph::with_capacity(max_nodes, max_edges);
        let p = Processor::with_capacity(max_nodes);
        // let box_g = Box::new(g);

        Engine {
            // chains: HashMap::<String, Vec<Box<dyn Node + 'static + Send >>>::new(), // a hashmap of Box<AsFunc>
            graph: g,
            processor: p,
            code: String::from(""),
            samples_dict: HashMap::new(),
            nodes: HashMap::new(),
            elapsed_samples: 0,
            sr: 44100.0,
            bpm: 120.0,
            update: false,
        }
    }

    fn generate_wave_buf(&mut self, _size:usize) -> [f32; 128] {
        let mut output: [f32; 128] = [0.0; 128];

        // (60.0 / self.bpm * 4.0 * 44100.0) as usize
        if self.update && self.elapsed_samples % ((60.0 / self.bpm * 4.0 * 44100.0) as usize) < 128 {
            self.update = false;
                // parse the code
            let lines = QParser::parse(Rule::block, self.code.as_str())
            .expect("unsuccessful parse")
            .next().unwrap();

            // add function to Engine HashMap Function Chain Vec accordingly
            for line in lines.into_inner() {

                let mut ref_name = "~";
                // let mut func_chain = Vec::<Box<dyn Signal<Frame=f64> + 'static + Send>>::new(); // init Chain

                // match line.as_rule() {
                //     Rule::line => {
                let inner_rules = line.into_inner();
                // let mut func_vec = Vec::<Box<dyn QuaverFunction + 'static + Send>>::new();

                for element in inner_rules {
                    match element.as_rule() {
                        Rule::reference => {
                            ref_name = element.as_str();
                        },
                        Rule::chain => {
                            let mut node_vec = Vec::<NodeIndex>::new();
                            for func in element.into_inner() {
                                let mut inner_rules = func.into_inner();
                                let name: &str = inner_rules.next().unwrap().as_str();
                                match name {
                                    "sin" => {
                                        let mut paras = inner_rules.next().unwrap().into_inner();

                                        // parsing 200 will cause error, 200.0 is fine.
                                        let freq = paras.next().unwrap().as_str().parse::<f64>().unwrap();

                                        let sin_osc = SinOsc::new(freq);
                                        // let s_node = engine.graph.add_node(NodeData::new1(BoxedNode::new(Box::new(sin_osc))));
                                        let sin_node = self.graph.add_node(NodeData::new1(BoxedNodeSend::new(sin_osc)));
                                        // engine.graph.add_node(NodeData::new1(BoxedNodeSend::new( Mul::new(0.5))));
                                        
                                        self.nodes.insert(ref_name.to_string(), sin_node);
                                        node_vec.insert(0, sin_node);

                                        // let sig = SinOsc::new(freq);
                                        // Add some nodes and edges...
                                        // engine.graph.add_node(NodeData::new(sig, Vec::<Buffer>::new()));

                                        // here we need to examine freq, if it is number, then make a consthz
                                        // if it is ref, make a hz modulation
                                        // let sig = signal::rate(48000.0).const_hz(freq).sine();
                                        // func_chain.push(Box::new(SinOsc::new(freq)));

                                    },
                                    "mul" => {
                                        let mut paras = inner_rules.next().unwrap().into_inner();
                                        // let mul = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                        // let mul = paras.next().unwrap().as_str().parse::<f64>();
                                        let mul = paras.next().unwrap().as_str().to_string();
                                        // if mul.is_ok() {

                                        let mul_node = self.graph.add_node(NodeData::new1(BoxedNodeSend::new( Mul::new(mul.clone()))));

                                        if node_vec.len() > 0 {
                                            self.graph.add_edge(node_vec[0], mul_node, ());
                                        }
                                        
                                        self.nodes.insert(ref_name.to_string(), mul_node);
                                        node_vec.insert(0, mul_node);

                                        let is_ref = !mul.parse::<f64>().is_ok();

                                        if is_ref {
                                            if !self.nodes.contains_key(mul.as_str()) {
                                                // panic if this item not existed
                                                // TODO: move it to a lazy function
                                                // engine.nodes.insert(mul.as_str().to_string(), mul_node);
                                            }                                    
                                            let mod_node = self.nodes[mul.as_str()]; 
                                            self.graph.add_edge(mod_node, mul_node, ());
                                        }
                                        // } else { // may be a ref

                                            // still need to add this
                                            // let mul_node = engine.graph.add_node(NodeData::new1(BoxedNodeSend::new(
                                            //     Mul::new(
                                            //         mul.unwrap()
                                            //     )
                                            // )));
                                        // };

                                        // match mul {
                                        //     Ok(val) => {

                                        //     },
                                        //     Err(why) => {}
                                        // }

                                        // engine.node.push(mul_node);
                                        // node_vec.push(mul_node);
                                    },
                                    "add" => {
                                        let mut paras = inner_rules.next().unwrap().into_inner();
                                        let add = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                        let add_node = self.graph.add_node(NodeData::new1(BoxedNodeSend::new( Add::new(add))));

                                        if node_vec.len() > 0 {
                                            self.graph.add_edge(node_vec[0], add_node, ());
                                        }
                                        
                                        self.nodes.insert(ref_name.to_string(), add_node);
                                        node_vec.insert(0, add_node);
                                        // engine.node.push(mul_node);
                                        // node_vec.push(mul_node);
                                    },
                                    "loop" => {
                                        // let mut q_loop = QuaverLoop::new();

                                        let mut events = Vec::<(f64, f64)>::new();

                                        let mut paras = inner_rules
                                        .next().unwrap().into_inner();

                                        let seq = paras.next().unwrap();
                                        let mut compound_index = 0;
                                        let seq_by_space: Vec<pest::iterators::Pair<Rule>> = 
                                        seq.clone().into_inner().collect();

                                        for compound in seq.into_inner() {
                                            let mut shift = 0;
                    
                                            // calculate the length of seq
                                            let compound_vec: Vec<pest::iterators::Pair<Rule>> = 
                                            compound.clone().into_inner().collect(); 
                    
                                            for note in compound.into_inner() {
                                                if note.as_str().parse::<i32>().is_ok() {
                                                    let seq_shift = 1.0 / seq_by_space.len() as f64 * 
                                                    compound_index as f64;
                                                    
                                                    let note_shift = 1.0 / compound_vec.len() as f64 *
                                                    shift as f64 / seq_by_space.len() as f64;
                    
                                                    let d = note.as_str().parse::<i32>().unwrap() as f64;
                                                    let relative_pitch = 2.0f64.powf((d - 60.0)/12.0);
                                                    let relative_time = seq_shift + note_shift;
                                                    events.push((relative_time, relative_pitch));
                                                    // let mut event = Event::new();
                                                
                                                    // event.pitch = pitch;

                                                    // better to push a events, right?
                                                    // q_loop.events.push(event);
                                                }
                                                shift += 1;
                                            }
                                            compound_index += 1;
                                        }

                                        let looper_node = self.graph.add_node(
                                            NodeData::new1(BoxedNodeSend::new( Looper::new(events)))
                                        );

                                        if node_vec.len() > 0 {
                                            self.graph.add_edge(node_vec[0], looper_node, ());
                                        }
                                        
                                        self.nodes.insert(ref_name.to_string(), looper_node);
                                        node_vec.insert(0, looper_node);

                                        // func_chain.functions.push(Box::new(q_loop));
                                    },
                                    "sampler" => {
                                        let mut paras = inner_rules.next().unwrap().into_inner();
                                        let symbol = paras.next().unwrap().as_str();

                                        let sampler_node = self.graph.add_node(
                                            NodeData::new1(BoxedNodeSend::new( Sampler::new(self.samples_dict[symbol])))
                                        );

                                        if node_vec.len() > 0 {
                                            self.graph.add_edge(node_vec[0], sampler_node, ());
                                        }
                                        
                                        self.nodes.insert(ref_name.to_string(), sampler_node);
                                        node_vec.insert(0, sampler_node);
                                        // sig.ins.push(
                                        //     Box::new(
                                        //         Sampler::new(samples_dict[symbol].clone())
                                        //     )
                                        // );
                                        // func_chain.functions.push(
                                        //     Box::new(Sampler::new(samples_dict[symbol].clone()))
                                        // );
                                    },
                                    "imp" => {
                                        let mut paras = inner_rules.next().unwrap().into_inner();
                                        let imp = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                        let imp_node = self.graph.add_node(
                                            NodeData::new1(BoxedNodeSend::new( Impulse::new(imp)))
                                        );

                                        if node_vec.len() > 0 {
                                            self.graph.add_edge(node_vec[0], imp_node, ());
                                        }
                                        
                                        self.nodes.insert(ref_name.to_string(), imp_node);
                                        node_vec.insert(0, imp_node);
                                    },
                                    "lpf" => {
                                    },
                                    _ => unreachable!()
                                }
                                // create the edge here
                                // if node_vec.len() == 2 {
                                //     engine.graph.add_edge(node_vec[0], node_vec[1], ());
                                //     node_vec.clear();
                                // }
                            

                            }
                        },
                        _ => unreachable!()
                    }
                }
            }
        }
        // we should see if we can update it
        for (ref_name, node) in &self.nodes {
            self.processor.process(&mut self.graph, *node);
            if ref_name.contains("~") {
                let b = &self.graph[*node].buffers[0];
                for i in 0..64 {
                    output[i] += b[i];
                    self.elapsed_samples += 1;
                }
            }
        }

        for (ref_name, node) in &self.nodes {
            self.processor.process(&mut self.graph, *node);
            if ref_name.contains("~") {
                let b = &self.graph[*node].buffers[0];
                for i in 64..128 {
                    output[i] += b[i-64];
                    self.elapsed_samples += 1;
                }
            }
        }

        
        
        // if self.nodes.len() > 0 {
        //     let n = self.node.len() - 1;
            // for n in self.node.clone() {
            // self.processor.process(&mut self.graph, self.node[n]);
            // let b = &self.graph[self.node[n]].buffers[0];
    
            // for i in 0..64 {
            //     output[i] = b[i]
            // }
            // }
    
            // for n in self.node.clone() {
            // self.processor.process(&mut self.graph, self.node[n]);
            // let b = &self.graph[self.node[n]].buffers[0];
    
            // for i in 64..128 {
            //     output[i] = b[i-64]
            // }
    
            // }
        // }


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