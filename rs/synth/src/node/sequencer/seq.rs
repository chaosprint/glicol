use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata, GlicolPara};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Sequencer {
    events: Vec<(f32, GlicolPara)>,
    ref_order: HashMap<String, usize>,
    speed: f32,
    pub bpm: f32,
    sr: usize,
    pub step: usize,
    input_order: Vec<usize>,
    // sidechain_lib: HashMap<String, usize>,
}

impl Sequencer {
    pub fn new(events: Vec<(f32, GlicolPara)>, ) -> Self {
        Self {
            events,
            ref_order: HashMap::new(),
            input_order: Vec::new(),
            speed: 1.0,
            bpm: 120.,
            sr: 44100,
            step: 0
        }
    }
    pub fn ref_order(self, ref_order: HashMap<String, usize>) -> Self {
        Self {
            ref_order, ..self
        }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self {
            sr, ..self
        }
    }
    pub fn bpm(self, bpm: f32) -> Self {
        Self {
            bpm, ..self
        }
    }

    impl_to_boxed_nodedata!();
}

impl< const N: usize> Node<N> for Sequencer {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("seq inputs info {:?} ; self.input_order {:?}", inputs, self.input_order);
        match inputs.len() {
            0 => {
                let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
                for i in 0..N {
                    output[0][i] = 0.0;
                    for event in &self.events {
                        if (self.step % (bar_length as usize)) == ((event.0 as f64 * bar_length) as usize) {
                            let midi = match event.1 {
                                GlicolPara::Number(value) => value,
                                _ => {0.0}
                            };
        
                            if midi == 0.0 {
                                output[0][i] = 0.0
                            } else {
                                output[0][i] = 2.0f32.powf((midi - 60.0)/12.0)
                            }
                        }
                    }
                    self.step += 1;
                }
            },
            _ => {
                // println!("{:?} {:?}", inputs, self.input_order);
                let possible_speed = &self.input_order[0];
                let has_speed = inputs[possible_speed].buffers()[0][0] > 0. && inputs[possible_speed].buffers()[0][1] == 0.;
                if has_speed { self.speed = inputs[possible_speed].buffers()[0][0]}
                let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
                for i in 0..N {
                    output[0][i] = 0.0;
                    
                    for event in &self.events {
                        if (self.step % (bar_length as usize)) == ((event.0 as f64 * bar_length) as usize) {
                            let midi = match &event.1 {
                                GlicolPara::Number(value) => *value,
                                GlicolPara::Reference(s) => {
                                    let source = &inputs[&self.input_order[self.ref_order[s] + has_speed as usize]]; //panic?
                                    source.buffers()[0][i]
                                },
                                _ => {return ()}
                            };
        
                            if midi == 0.0 {
                                output[0][i] = 0.0
                            } else {
                                output[0][i] = 2.0f32.powf((midi - 60.0)/12.0)
                            }
                        }
                    }
                    self.step += 1;
                }
            }
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetBPM(bpm) => {
                self.bpm = bpm
            },
            Message::SetToSeq(pos, events) => {
                match pos {
                    0 => {
                        self.events = events
                    },
                    _ => {}
                }
            },
            Message::SetRefOrder(ref_order) => {
                self.ref_order = ref_order;
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}