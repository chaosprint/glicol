use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct PSampler {
    playback: Vec<(usize, String, f32)>,
    samples_dict: HashMap<String, (&'static [f32], usize, usize)>,
    pub events: Vec<(String, f32)>,
    pattern: Vec<(String, f32)>,
    // len: usize,
    // endindex: usize,
    pub step: usize,
    period_in_cycle: f32,
    cycle_dur: f32,
    sr: usize,
    input_order: Vec<usize>
}

impl PSampler {
    pub fn new(
        samples_dict: HashMap<String, (&'static [f32], usize, usize)>,
        sr: usize,
        bpm: f32,
        events: Vec<(String, f32)>,
        pattern: Vec<(String, f32)>,
        period_in_cycle: f32,
    ) -> Self {
        Self {
            playback: vec![],
            samples_dict,
            events,
            pattern,
            // len: sample.0.len()/sample.1, // total_length/channels
            // endindex: sample.0.len()-1,
            step: 0,
            cycle_dur: 60. / bpm * 4.,
            period_in_cycle,
            sr,
            input_order: vec![]
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for PSampler {
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        output[0].silence();
        output[1].silence();

        // let main_input = inputs.values_mut().next().unwrap();
        // let input_buf = &mut main_input.buffers();

        let bar_dur = self.cycle_dur * self.period_in_cycle * self.sr as f32;

        for i in 0..N {
            // if input_buf[0][i] > 0.0 {
            
            for event in &self.pattern {
                
                if (self.step % (bar_dur as usize)) == ((event.1 * self.cycle_dur * self.sr as f32) as usize) {
                    let pitch = 1.0;
                    let sample_name = &event.0;
                    let sample = self.samples_dict[sample_name];
                    let dur = (sample.0.len()/sample.1) as f32 / pitch / ( sample.2 as f32 / self.sr as f32 );
                    self.playback.push((self.step, sample_name.to_owned(), dur));
                }
            }
            
            let mut count = 0;
            let mut to_remove = vec![];
            for (begin, name, dur) in &self.playback {
                let pos = (self.step - begin) as f32 / dur;
                if pos <= 1.0 {
                    let sample = self.samples_dict[name];
                    match sample.1 {
                        1 => {
                            output[0][i] += match pos {
                                x if x == 0.0 => sample.0[0],
                                x if x == 1.0 => sample.0[sample.0.len()-1],
                                x if x > 0.0 && x < 1.0 => {
                                    let pos_index_float = x * ((sample.0.len()-1) as f32);
                                    let left = pos_index_float.floor();
                                    let right = pos_index_float.ceil();
                                    let left_portion = pos_index_float - left;
                                    let right_portion = 1. - left_portion;

                                    sample.0[left as usize] * left_portion +
                                    sample.0[right as usize] * right_portion
                                },
                                _ => 0.0
                            };
                            output[1][i] = output[0][i];
                        },
                        2 => {
                            match pos {
                                x if x == 0.0 => {
                                    output[0][i] += sample.0[0];
                                    output[1][i] += sample.0[(sample.0.len()/sample.1)];
                                },
                                x if x == 1.0 => {
                                    output[0][i] += sample.0[(sample.0.len()/sample.1)-1];
                                    output[1][i] += sample.0[sample.0.len()-1];
                                },
                                x if x > 0.0 && x < 1.0 => {
                                    let pos_index_float = x * (((sample.0.len()/sample.1)-2) as f32);
                                    let left = pos_index_float.floor();
                                    let right = pos_index_float.ceil();
                                    let left_portion = pos_index_float - left;
                                    let right_portion = 1. - left_portion;

                                    output[0][i] += sample.0[left as usize] * left_portion +
                                    sample.0[right as usize] * right_portion;
                                    
                                    output[1][i] += sample.0[left as usize + (sample.0.len()/sample.1) + 1] * left_portion
                                    + sample.0[right as usize + (sample.0.len()/sample.1) + 1] * right_portion

                                },
                                _ => {}
                            };
                        },
                        _ => {return ()}
                    }
                } else {
                    // panic!();
                    to_remove.push(count)
                }
                count += 1;
            }
            for c in to_remove.iter().rev() {
                self.playback.remove(*c);
            }
            self.step += 1;
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetSamplePattern(pattern, span, samples_dict) => {
                self.playback.clear();
                self.pattern = pattern;
                self.samples_dict = samples_dict;
                self.period_in_cycle = span                
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