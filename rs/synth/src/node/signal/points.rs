use crate::GlicolPara;
use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Points {
    pub point_list: Vec<(usize, f32)>, // time and value
    pub span: f32,
    pub bpm: f32,
    pub sr: usize,
    step: usize,
    _index: usize,
    is_looping: bool,
    input_order: Vec<usize>,
}

impl Default for Points {
    fn default() -> Self {
        Self::new()
    }
}

impl Points {
    pub fn new() -> Self {
        // points: GlicolPara, bpm: f32, sr: usize, span: f32
        // let mut point_list = process_points(points, bpm, sr, span);
        // println!("point_list {:?}", point_list);
        Self {
            point_list: vec![],
            span: 1.0,
            bpm: 120.,
            sr: 44100,
            step: 0,
            _index: 0,
            is_looping: false,
            input_order: vec![],
        }
    }

    pub fn span(self, span: f32) -> Self {
        Self { span, ..self }
    }

    pub fn is_looping(self, is_looping: bool) -> Self {
        Self { is_looping, ..self }
    }

    pub fn bpm(self, bpm: f32) -> Self {
        Self { bpm, ..self }
    }

    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }

    pub fn points(self, points: GlicolPara) -> Self {
        let bpm = self.bpm;
        let sr = self.sr;
        let span = self.span;
        let point_list = self.make_point_list(points, bpm, sr, span);
        Self { point_list, ..self }
    }

    fn make_point_list(
        &self,
        points: GlicolPara,
        bpm: f32,
        sr: usize,
        span: f32,
    ) -> Vec<(usize, f32)> {
        let mut point_list = match points {
            GlicolPara::Points(p) => p.into_iter().map(|point| {
                let time = point.0;
                let mut pos = 0; // which sample
                if let GlicolPara::Time(t) = time {
                    let cycle_dur = 60. / bpm * 4.;
                    let bar_dur = cycle_dur * span * sr as f32;

                    for time_kind in t {
                        match time_kind {
                            GlicolPara::Bar(x) => {
                                pos += (x * bar_dur) as usize;
                            }
                            GlicolPara::Second(x) => {
                                pos += (x * (sr as f32)) as usize;
                            }
                            GlicolPara::Millisecond(x) => {
                                pos += (x / 1000.0 * (sr as f32)) as usize;
                            }
                            _ => {}
                        }
                    }
                }
                let value = match point.1 {
                    GlicolPara::Number(v) => v,
                    _ => 0.0,
                };
                (pos, value)
            }).collect(),
            _ => vec![]
        };

        if point_list[0].0 != 0 {
            point_list.insert(0, (0, 0.0));
        }

        point_list
    }

    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Points {
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("span {}", self.span);
        let list_len = self.point_list.len();
        if list_len == 0 {
            return ;
        }
        let cycle_dur = 60. / self.bpm * 4.;
        let bar_dur = (cycle_dur * self.span * self.sr as f32) as usize;

        if !self.is_looping {
            for out in &mut *output[0] {
                let pos = self.step;
                let samples = &self.point_list;

                let index = samples.iter().position(|s| pos <= s.0).map_or(0, |p| p - 1);

                // println!("index {} pos {} samples {:?}", index, pos, samples);
                if index < samples.len() - 1 {
                    // not yet reach the last point
                    let (prev_pos, prev_val) = samples[index];
                    let (next_pos, next_val) = samples[index + 1];
                    let t = (pos as f32 - prev_pos as f32) / (next_pos as f32 - prev_pos as f32);
                    *out = prev_val + t * (next_val - prev_val);
                } else {
                    // reach the last point
                    // pass the last point stay at the last point
                    // panic!("self.step {}", self.step);
                    *out = samples.last().unwrap().1;
                }
                self.step += 1;
            }
        } else {
            for out in &mut *output[0] {
                let pos = self.step % bar_dur;
                let period = bar_dur;
                let samples = &self.point_list;

                let prev = samples.iter()
                    .enumerate()
                    .find(|(_, (s, _))| pos <= *s);

                let is_last = prev.is_none();
                let (index, (prev_pos, prev_val)) = prev
                    .unwrap_or_else(|| (samples.len() - 1, samples.last().unwrap()));

                let (next_pos, next_val) = samples[(index + 1) % samples.len()];

                let t = if is_last {
                    (pos as f32 - *prev_pos as f32)
                        / (period as f32 - *prev_pos as f32 + next_pos as f32)
                } else {
                    (pos as f32 - *prev_pos as f32) / (next_pos as f32 - *prev_pos as f32)
                };
                *out = prev_val + t * (next_val - prev_val);

                self.step += 1;
            }
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetParam(0, params) => {
                self.point_list = self.make_point_list(params, self.bpm, self.sr, self.span);
                self.step = 0;
            },
            Message::SetToNumber(1, num) => {
                self.span = num;
                self.step = 0;
            },
            Message::SetToBool(2, b) => {
                self.is_looping = b;
                self.step = 0;
            },
            Message::SetBPM(bpm) => self.bpm = bpm,
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            _ => {}
        }
    }
}
