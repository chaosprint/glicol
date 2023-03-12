use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use crate::GlicolPara;
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Points {
    pub point_list: Vec<(usize, f32)>, // time and value
    pub span: f32,
    pub bpm: f32,
    pub sr: usize,
    step: usize,
    index: usize,
    is_looping: bool,
    input_order: Vec<usize> 
}

impl Points {
    pub fn new() -> Self { // points: GlicolPara, bpm: f32, sr: usize, span: f32 
        // let mut point_list = process_points(points, bpm, sr, span);
        // println!("point_list {:?}", point_list);
        Self {
            point_list: vec![],
            span: 1.0,
            bpm: 120.,
            sr: 44100,
            step: 0,
            index: 0,
            is_looping: false,
            input_order: vec![]
        }
    }

    pub fn span(self, span: f32) -> Self {
        Self {span, ..self}
    }

    pub fn is_looping(self, is_looping: bool) -> Self {
        Self {is_looping, ..self}
    }

    pub fn bpm(self, bpm: f32) -> Self {
        Self {bpm, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn points(self, points: GlicolPara) -> Self {
        let bpm = self.bpm;
        let sr = self.sr;
        let span = self.span;
        let point_list = self.make_point_list(points, bpm, sr, span);
        Self {point_list, ..self}
    }

    fn make_point_list(&self, points: GlicolPara, bpm: f32, sr: usize, span: f32) -> Vec<(usize, f32)> {
        let mut point_list = vec![];
        match points {
            GlicolPara::Points(p) => {
                for point in p {
                    let time = point.0;
                    let mut pos = 0; // which sample 
                    match time {
                        GlicolPara::Time(t) => {
                            let cycle_dur = 60. / bpm * 4.;
                            let bar_dur = cycle_dur * span * sr as f32;

                            for time_kind in t {
                                match time_kind {
                                    GlicolPara::Bar(x) => {
                                        pos += (x * bar_dur) as usize;
                                    },
                                    GlicolPara::Second(x) => {
                                        pos += (x * (sr as f32) ) as usize;
                                    },
                                    GlicolPara::Millisecond(x) => {
                                        pos += (x / 1000.0 * (sr as f32)) as usize;
                                    },
                                    _ => {},
                                }
                            }
                        },
                        _ => {}
                    }
                    let value = match point.1 {
                        GlicolPara::Number(v) => v,
                        _ => 0.0
                    };
                    point_list.push((pos, value));
                }
            }
            _ => {}
        };
        if point_list[0].0 != 0 {
            point_list.insert(0, (0, 0.0));
        }
        point_list
    }

    impl_to_boxed_nodedata!();
}

impl<const N:usize> Node<N> for Points {
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("span {}", self.span);
        let list_len = self.point_list.len();
        if list_len == 0 {
            return ()
        }
        let cycle_dur = 60. / self.bpm * 4.;
        let bar_dur = (cycle_dur * self.span * self.sr as f32) as usize;
        
        if !self.is_looping {
            for i in 0..N {
                let pos = self.step;
                let samples = &self.point_list;
    
                // WRITTEN BY ChatGPT, instructed and modified by chaosprint
                let len = samples.len();
                let mut index = 0;
                while index < len - 1 && pos > samples[index + 1].0 {
                    index += 1;
                }

                // println!("index {} pos {} samples {:?}", index, pos, samples);
                if index < len - 1 { // not yet reach the last point
                    let (prev_pos, prev_val) = samples[index];
                    let (next_pos, next_val) = samples[index + 1];
                    let t = (pos as f32  - prev_pos as f32 )  / (next_pos  as f32 - prev_pos as f32 );
                    output[0][i] = prev_val + t * (next_val - prev_val);
                    self.step += 1;
                } else { // reach the last point
                    // pass the last point stay at the last point
                    // panic!("self.step {}", self.step);
                    output[0][i] = samples[len-1].1;
                    self.step += 1;
                }
            }
        } else {
            
            for i in 0..N {
                
                let pos = self.step % bar_dur as usize;
                let period = bar_dur as usize;
                let samples = &self.point_list;
    
                // WRITTEN BY ChatGPT, instructed and modified by chaosprint
                let len = samples.len();
                let mut index = 0;
                while index < len - 1 && pos > samples[index + 1].0 {
                    index += 1;
                }
                let (prev_pos, prev_val) = samples[index];
                let (next_pos, next_val) = if index == len - 1 {
                    samples[0]
                } else {
                    samples[index + 1]
                };
                let t = if index == len - 1 {
                    (pos as f32 - prev_pos as f32) / (period  as f32 - prev_pos as f32  + next_pos as f32 )
                } else {
                    (pos as f32  - prev_pos as f32 )  / (next_pos  as f32 - prev_pos as f32 )
                };
                output[0][i] = prev_val + t * (next_val - prev_val);
    
                self.step += 1;
            }
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetParam(pos, params) => {
                match pos {
                    0 => {
                        self.point_list = self.make_point_list(params, self.bpm, self.sr, self.span);
                        self.step = 0;
                    }
                    _ => {}
                }
            },
            Message::SetToNumber(pos, num) => {
                match pos {
                    1 => {
                        self.span = num;
                        self.step = 0;
                    }
                    _ => {}
                }
            },
            Message::SetToBool(pos, b) => {
                match pos {
                    2 => {
                        self.is_looping = b;
                        self.step = 0;
                    }
                    _ => {}
                }
            },
            Message::SetBPM(bpm) => {
                self.bpm = bpm
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