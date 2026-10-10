use crate::{Buffer, Input, Message, Node};
use glicol_parser::nodes::UsizeOrRef;
use hashbrown::HashMap;

#[derive(Debug)]
pub struct Sequencer {
    events: Vec<(f32, UsizeOrRef<String>)>,
    ref_order: HashMap<String, usize>,
    speed: f32,
    pub bpm: f32,
    sr: usize,
    pub step: usize,
    has_chain_input: bool,
    input_order: Vec<usize>,
    // sidechain_lib: HashMap<String, usize>,
}

impl Sequencer {
    pub fn new(events: Vec<(f32, UsizeOrRef<String>)>) -> Self {
        Self {
            events,
            ref_order: HashMap::new(),
            input_order: Vec::new(),
            speed: 1.0,
            bpm: 120.,
            sr: 44100,
            step: 0,
            has_chain_input: false,
        }
    }
    pub fn ref_order(self, ref_order: HashMap<String, usize>) -> Self {
        Self { ref_order, ..self }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }
    pub fn bpm(self, bpm: f32) -> Self {
        Self { bpm, ..self }
    }
}

impl<const N: usize> Node<N> for Sequencer {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("seq inputs info {:?} ; self.input_order {:?}", inputs, self.input_order);
        match inputs.len() {
            0 => {
                let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
                for out in &mut *output[0] {
                    *out = 0.0;
                    for event in &self.events {
                        if (self.step % (bar_length as usize))
                            == ((event.0 as f64 * bar_length) as usize)
                        {
                            let midi = match event.1 {
                                UsizeOrRef::Usize(value) => value,
                                UsizeOrRef::Ref(_) => 0,
                            };

                            if midi == 0 {
                                *out = 0.0
                            } else {
                                *out = 2.0f32.powf((midi as f32 - 60.0) / 12.0)
                            }
                        }
                    }
                    self.step += 1;
                }
            }
            _ => {
                // println!("{:?} {:?}", inputs, self.input_order);
                let possible_speed = &inputs[&self.input_order[0]].buffers()[0];
                let has_speed =
                    self.has_chain_input && possible_speed[0] > 0. && possible_speed[1] == 0.;

                if has_speed {
                    self.speed = possible_speed[0];
                }

                let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
                for (idx, out) in output[0].iter_mut().enumerate() {
                    *out = 0.0;

                    for event in &self.events {
                        if (self.step % (bar_length as usize))
                            == ((event.0 as f64 * bar_length) as usize)
                        {
                            let midi = match &event.1 {
                                UsizeOrRef::Usize(value) => *value as f32,
                                UsizeOrRef::Ref(s) => {
                                    let source = &inputs
                                        [&self.input_order[self.ref_order[s] + has_speed as usize]]; //panic?
                                    source.buffers()[0][idx]
                                }
                            };

                            if midi == 0.0 {
                                *out = 0.0
                            } else {
                                *out = 2.0f32.powf((midi - 60.0) / 12.0)
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
            Message::SetBPM(bpm) => self.bpm = bpm,
            Message::SetToSeq(0, events) => self.events = events,
            Message::SetRefOrder(ref_order) => {
                self.ref_order = ref_order;
            }
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(0, index) => {
                self.has_chain_input = true;
                self.input_order.insert(0, index)
            }
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
                self.has_chain_input = false;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ref_input_from_another_seq_is_not_mistaken_for_speed() {
        const N: usize = 128;
        let mut inputs: HashMap<usize, Input<N>> = HashMap::new();
        let mut output = [Buffer::<N>::SILENT];
        let mut seq = Sequencer::new(vec![(0.0, UsizeOrRef::Ref("~n".to_owned()))]);
        let mut refs = HashMap::new();
        refs.insert("~n".to_owned(), 0);
        <Sequencer as Node<N>>::send_msg(&mut seq, Message::SetRefOrder(refs));
        <Sequencer as Node<N>>::send_msg(&mut seq, Message::Index(7));
        let blip = [Buffer::from({
            let mut b = [0.0f32; N];
            b[0] = 2.0f32.powf((1.0f32 - 60.0) / 12.0);
            b
        })];
        inputs.insert(7, Input::new(&blip, 7));
        seq.process(&mut inputs, &mut output);
    }
}
