use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct PatternSynth {
    synth_list: Vec<(usize, f32)>,
    phase_list: Vec<f32>,
    att: f32,
    dec: f32,
    events: Vec<(f32, f32)>, // event.0 is frac, event.1 is midi
    ref_order: HashMap<String, usize>,
    period_in_cycle: f32, // in cycles, can be 1.2121 for example
    cycle_dur: f32,       // time
    sr: usize,
    step: usize,
    input_order: Vec<usize>,
}

impl PatternSynth {
    pub fn new(events: Vec<(f32, f32)>) -> Self {
        Self {
            synth_list: vec![],
            phase_list: vec![],
            events,
            att: 0.001,
            dec: 0.1,
            ref_order: HashMap::new(),
            input_order: Vec::new(),
            period_in_cycle: 1.0,
            cycle_dur: 2.0,
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
    pub fn period_in_cycle(self, period_in_cycle: f32) -> Self {
        Self {
            period_in_cycle,
            ..self
        }
    }

    pub fn cycle_dur(self, cycle_dur: f32) -> Self {
        Self { cycle_dur, ..self }
    }

    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for PatternSynth {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("seq inputs info {:?} ; self.input_order {:?}", inputs, self.input_order);
        // println!("events{:?}", self.events);
        let attack_n = (self.att * self.sr as f32) as usize;
        let decay_n = (self.dec * self.sr as f32) as usize;
        if inputs.is_empty() {
            let bar_length = self.cycle_dur * self.period_in_cycle * self.sr as f32;
            for out in &mut *output[0] {
                *out = 0.0;

                for event in &self.events {
                    if (self.step % (bar_length as usize))
                        == ((event.0 * self.cycle_dur * self.sr as f32) as usize)
                    {
                        let midi = event.1;
                        let freq = 2f32.powf((midi - 69.) / 12.) * 440.;

                        // need to push current step to the playback list
                        // println!("{}{}", event.0 * self.cycle_dur);
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
                            Ordering::Less | Ordering::Equal => if attack_n == 0 {
                                0.0
                            } else {
                                pos as f32 / (self.att * self.sr as f32)
                            },
                            Ordering::Greater => if decay_n == 0 {
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
                // println!("output, {}", output[0][i]);
            }
            // println!("self.synth_list {:?} step, {:?}", self.synth_list, self.step);
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            // Message::SetBPM(bpm) => {
            //     self.bpm = bpm
            // },
            Message::SetToSymbol(0, s) => {
                // panic!();
                self.events.clear();
                let pattern = s.replace('`', "");
                for event in pattern.split(',') {
                    // println!("event {:?}", event);
                    let mut result = event
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.replace(' ', "").parse::<f32>().unwrap());

                    // println!("result {:?}", result);
                    self.events.push(
                        (result.next().unwrap(), result.next().unwrap())
                    );
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
