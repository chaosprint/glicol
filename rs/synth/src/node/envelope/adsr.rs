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
    step: usize, // the clock since the node created
    is_releasing: bool,
    sr: usize,
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
            is_releasing: false,
            gate: 0.0,
            sr: 44100,
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
                
                let n_before_sustain = attack_len + decay_len;
                let buf = &mut inputs[&self.input_order[0]].buffers();

                for i in 0..N {

                    if buf[0][i] > 0.0 {
                        // entering the attack decay phase
                        if self.gate == 0. {
                            self.pos = 0;
                            self.gate = buf[0][i];
                        }
                        
                        if self.pos <= attack_len {
                            if attack_len == 0 {
                                output[0][i] = 0.0;
                            } else {
                                output[0][i] = self.pos as f32 / attack_len as f32;
                            }
                        } else if self.pos > attack_len && self.pos <= n_before_sustain {
                            if decay_len == 0 {
                                output[0][i] = self.sustain;
                            } else {
                                output[0][i] = (n_before_sustain - self.pos) as f32 / decay_len as f32 * (1. - self.sustain) + self.sustain;
                            }
                        } else {
                            output[0][i] = self.sustain
                        }
                    } else {
                        if self.gate > 0. {
                            self.pos = 0;
                        }
                        if self.is_releasing || self.gate > 0. {
                            self.gate = 0.;
                            self.is_releasing = true;
                            if self.pos == release_len {
                                output[0][i] = 0.0;
                                self.is_releasing = false;
                            } else {
                                output[0][i] = (release_len - self.pos) as f32 / release_len as f32 * self.sustain;
                            }
                        }
                    };
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