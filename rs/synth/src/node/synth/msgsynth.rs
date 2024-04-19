use std::cmp::Ordering;

use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct MsgSynth {
    synth_list: Vec<(usize, f32)>,
    phase_list: Vec<f32>,
    att: f32,
    dec: f32,
    events: Vec<(usize, f32)>, // event.0 is step to play the note, event.1 is midi
    ref_order: HashMap<String, usize>,
    // period_in_cycle: f32, // in cycles, can be 1.2121 for example
    // cycle_dur: f32, // time
    sr: usize,
    step: usize,
    input_order: Vec<usize>,
}

impl Default for MsgSynth {
    fn default() -> Self {
        Self::new()
    }
}

impl MsgSynth {
    pub fn new() -> Self {
        Self {
            synth_list: vec![],
            phase_list: vec![],
            events: vec![], // test with (88200, 60.)
            att: 0.001,
            dec: 0.1,
            ref_order: HashMap::new(),
            input_order: Vec::new(),
            // period_in_cycle: 1.0,
            // cycle_dur: 2.0,
            sr: 44100,
            step: 0,
        }
    }
    pub fn ref_order(self, ref_order: HashMap<String, usize>) -> Self {
        Self { ref_order, ..self }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }

    pub fn attack(self, att: f32) -> Self {
        Self { att, ..self }
    }

    pub fn decay(self, dec: f32) -> Self {
        Self { dec, ..self }
    }

    // pub fn period_in_cycle(self, period_in_cycle: f32) -> Self {
    //     Self {
    //         period_in_cycle, ..self
    //     }
    // }

    // pub fn cycle_dur(self, cycle_dur: f32) -> Self {
    //     Self {
    //         cycle_dur, ..self
    //     }
    // }

    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for MsgSynth {
    // the behaviour of this synth is that it only plays a note on msg
    // not from any input
    // different from psynth, the event in this node is linear

    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // panic!();

        let attack_n = (self.att * self.sr as f32) as usize;
        let decay_n = (self.dec * self.sr as f32) as usize;
        // match inputs.len() {
        // 0 => {
        // let bar_length = self.cycle_dur * self.period_in_cycle * self.sr as f32;
        for out in &mut *output[0] {
            *out = 0.0;

            for event in &self.events {
                if self.step == event.0 {
                    let midi = event.1;
                    let freq = 2f32.powf((midi - 69.) / 12.) * 440.;
                    self.synth_list.push((self.step, freq));
                    self.phase_list.push(0.0);
                }
            }

            let mut to_remove = Vec::with_capacity(self.synth_list.len());
            for (synth_index, (synth_info, phase)) in self.synth_list.iter()
                .zip(self.phase_list.iter_mut())
                .enumerate()
            {
                let dur = (self.att + self.dec) * self.sr as f32;

                if self.step - synth_info.0 <= dur as usize {
                    let pos = self.step - synth_info.0;

                    let amp = match pos.cmp(&attack_n) {
                        Ordering::Greater | Ordering::Equal => if attack_n == 0 {
                            0.0
                        } else {
                            pos as f32 / (self.att * self.sr as f32)
                        },
                        Ordering::Less => if decay_n == 0 {
                            0.0
                        } else {
                            (dur as usize - pos) as f32 / (self.dec * self.sr as f32)
                        }
                    };

                    let phase_out = *phase * 2. - 1.;
                    *phase += synth_info.1 / self.sr as f32;
                    if *phase > 1. {
                        *phase -= 1.
                    }
                    // println!("amp {} out {} step {}", amp, out, self.step);
                    *out += amp * phase_out * 0.1;
                    // println!("output[{}] {}",i, output[0][i]);
                } else {
                    // remove this from start_step_list and output_list
                    to_remove.push(synth_index)
                }
            }
            for c in to_remove.iter().rev() {
                self.synth_list.remove(*c);
                self.phase_list.remove(*c);
            }
            self.step += 1;
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, v) => match pos {
                1 => self.att = v,
                2 => self.dec = v,
                _ => {}
            },
            Message::SetToSymbol(pos, s) => {
                // panic!();
                match pos {
                    0 => {
                        // self.type = 0
                    }
                    3 => {
                        let event_s: String =
                            s.chars().filter(|c| !c.is_whitespace()).collect::<_>();
                        // estimate event_s "2.74343=>60"
                        // for event_s.split("=>");
                        if event_s.contains("=>") {

                            let mut events = event_s.split("=>").map(|v| v.parse::<f32>().unwrap());

                            if let (Some(start), Some(end)) = (events.next(), events.next()) {
                                let event_n = (start * self.sr as f32) as usize;
                                self.events.push((event_n, end));
                            }
                        }
                    }
                    _ => {}
                }
            }
            Message::SetRefOrder(ref_order) => {
                self.ref_order = ref_order;
            }
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
