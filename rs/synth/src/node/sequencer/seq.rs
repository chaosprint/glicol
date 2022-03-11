use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata, HashMap, GlicolPara};

#[derive(Debug, Clone)]
pub struct Sequencer {
    events: Vec<(f32, GlicolPara<'static>)>,
    ref_order: HashMap<&'static str, usize>,
    speed: f32,
    bpm: f32,
    sr: usize,
    pub step: usize,
    input_order: Vec<usize>,
    // sidechain_lib: HashMap<String, usize>,
}

impl Sequencer {
    pub fn new(events: Vec<(f32, GlicolPara<'static>)>, ) -> Self {
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
    pub fn ref_order(self, ref_order: HashMap<&'static str, usize>) -> Self {
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
        let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
        match inputs.len() {
            0 => {  
                for i in 0..N {
                    output[0][i] = 0.0;
                    for event in &self.events {
                        if (self.step % (bar_length as usize)) == ((event.0 as f64 * bar_length) as usize) {
                            let midi = match event.1 {
                                GlicolPara::Number(value) => value,

                                GlicolPara::Reference(s) => {
                                    // let index = inputs.len() - 1 - has_clock as usize - has_speed_input as usize
                                    // - self.sidechain_lib[&event.1];
                                    // inputs[index].buffers()[0][i]
                                    let source = &inputs[&self.input_order[self.ref_order[s]]];
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
            },
            1 => {

            },
            _ => return ()
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToSeq(pos, events) => {
                match pos {
                    0 => {
                        self.events = events
                    },
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            _ => {}
        }
    }
}