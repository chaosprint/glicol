use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct Adsr {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    pos: usize, // the pos since the attack is triggered
    gate: f32,
    lastx: f32,
    lasty: f32,
    step: usize, // the clock since the node created
    state_change_y: f32,
    sr: usize,
    phase: u8,
    input_order: Vec<usize>,
}

impl Adsr {
    pub fn new() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.3,
            release: 0.1,
            pos: 0,
            step: 0,
            lastx: 0.,
            lasty: 0.,
            state_change_y: 0.0,
            gate: 0.0,
            sr: 44100,
            phase: 0,
            input_order: vec![],
        }
    }

    pub fn attack(self, attack: f32) -> Self {
        Self {attack, ..self}
    }
    pub fn decay(self, decay: f32) -> Self {
        Self {decay, ..self}
    }
    pub fn sustain(self, sustain: f32) -> Self {
        Self {sustain, ..self}
    }
    pub fn release(self, release: f32) -> Self {
        Self {release, ..self}
    }
    pub fn gate(self, gate: f32) -> Self {
        Self {gate, ..self}
    }
    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Adsr {
    
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                let attack_len = (self.attack * self.sr as f32) as usize;
                let decay_len = (self.decay * self.sr as f32) as usize;
                let release_len = (self.release * self.sr as f32) as usize;

                let buf = &mut inputs[&self.input_order[0]].buffers();
    
                for i in 0..N {

                    if buf[0][i] > 0.0 && self.lastx == 0.0 {
                        self.gate = 1.;
                        self.phase = 1; // attack, decay or sustain
                        self.pos = 0;
                        self.state_change_y = self.lasty;
                    } else if buf[0][i] == 0.0 && self.lastx > 0.0 {
                        self.gate = 0.;
                        self.phase = 2; // release
                        self.pos = 0;
                        self.state_change_y = self.lasty;
                    }

                    // based on pos and phase, calculate the output
                    match self.phase {
                        1 => {
                            if self.pos <= attack_len { // attack phase
                                if attack_len == 0 { // special case
                                    output[0][i] = 0.0;
                                } else {
                                    // attack from: lasty -> 1.0
                                    output[0][i] = self.pos as f32 / attack_len as f32 * (1.0 - self.state_change_y);
                                }
                            } else if self.pos > attack_len && self.pos <= attack_len + decay_len {
                                // decay phase
                                if decay_len == 0 { // special case
                                    output[0][i] = self.sustain;
                                } else {
                                    output[0][i] = (attack_len + decay_len - self.pos) as f32 / decay_len as f32 * (1. - self.sustain) + self.sustain;
                                }
                            } else {
                                output[0][i] = self.sustain;
                            }
                        },
                        2 => {
                            if self.pos >= release_len {
                                output[0][i] = 0.0;
                            } else {
                                output[0][i] = (release_len - self.pos) as f32 / release_len as f32 * (self.state_change_y);
                            }
                        },
                        _ => {
                            output[0][i] = 0.0;
                        }
                    };
                    self.lasty = output[0][i];
                    if output.len() == 2 {
                        output[1][i] = output[0][i];
                    }
                    self.lastx = buf[0][i];
                    self.step += 1;
                    self.pos += 1;
                }
            },
            _ => {return ()}
        }
    }    

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {self.attack = value},
                    1 => {self.decay = value},
                    2 => {self.sustain = value},
                    3 => {self.release = value},
                    _ => {}
                }
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

enum Phase {
    Attack,
    Decay,
    Sustain,
    Release
}