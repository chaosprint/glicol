// use dasp_signal::{self as signal, Signal};
// use dasp_slice::{ToFrameSlice};
// use dasp_interpolate::linear::Linear;
use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use super::super::{Rule, HashMap, NodeData, BoxedNodeSend, EngineError};

pub struct Sampler {
    // pub sig: Vec< Box<dyn Signal<Frame=[f32;1]> + 'static + Send>>,
    playback: Vec<(usize, f64)>,
    sample: &'static[f32],
    len: usize,
    endindex: usize,
}
impl Sampler {
    pub fn new(
        paras: &mut Pairs<Rule>,
        samples_dict: &HashMap<String, &'static[f32]>
    ) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        let p = paras.next().unwrap();
        let pos = (p.as_span().start(), p.as_span().end());
        let key = p.as_str();
        if !samples_dict.contains_key(key) {
            return Err(EngineError::SampleNotExistError(pos))
        }
        let sample = samples_dict[key];
        let len = sample.len();
        let endindex = len - 1;
        Ok((NodeData::new1(BoxedNodeSend::new(Self{
            playback: Vec::new(),
            sample,
            len,
            endindex,
        })), vec![]))
    }
}

impl Node for Sampler {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        output[0].silence();
        
        if inputs.len() > 1 {
            let mut clock = inputs[1].buffers()[0][0] as usize;
            // the input of sampler should be a pitch, and series of 0
            let input_buf = &mut inputs[0].buffers();

            for i in 0..64 {
                // if input_buf[0][i] > 0.0 {
                    // do it every sample, will it be too expensive?
                if input_buf[0][i] > 0.0 {
                    // let f: &[[f32;1]] = self.samples.to_frame_slice().unwrap();
                    // let s = signal::from_iter(f.iter().cloned());
                    // let mut source = signal::from_iter(f.iter().cloned());
                    // let a = source.next();
                    // let b = source.next();
                    // let interp = Linear::new(a, b);
                    // let s = source.scale_hz(interp, input_buf[0][i] as f64);
                    // as f64 /2.0_f64.powf((60.0-69.0)/12.0)/440.0;
                    // self.sig.push(Box::new(s));
                    let dur = self.len as f64 / input_buf[0][i] as f64;
                    self.playback.push((clock, dur));
                }
                for (begin, dur) in &self.playback {
                    let pos = (clock - begin) as f64 / dur;
                    if pos <= 1.0 {
                        output[0][i] += match pos {
                            x if x == 0.0 => self.sample[0],
                            x if x == 1.0 => self.sample[self.endindex],
                            x if x > 0.0 && x < 1.0 => {
                                let left = (x*(self.endindex as f64)).floor();
                                let right = (x*(self.endindex as f64)).ceil();
                                (self.sample[left as usize] as f64
                                * ((x*(self.endindex as f64)) - left)
                                + self.sample[right as usize] as f64
                                * (right - (x*(self.endindex as f64)))) as f32
                            },
                            _ => 0.0
                        };
                    }
                }

                clock += 1;
                // }
                // for i in 0..output[0].len() {
                // self.sig.retain(|s|!s.is_exhausted());
                // for s in &mut self.sig {
                //     // if !s.is_exhausted() {
                //     output[0][i] += s.next()[0];
                //     // }            
                // }
            }
            self.playback.retain(|v| ((clock - v.0) as f64) <= v.1);
        }
    }
}

// discarded
// struct Looper {
//     sig: Box<dyn Signal<Frame=f32> + Send>,
//     // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
// }

// #[allow(dead_code)]
// impl Looper {
//     fn new(events: Vec<(f64, f64)>) -> Self {
//         // let p = (44100.0 / 10.0) as usize;
//         let mut i: usize = 0;
//         let s = signal::gen_mut(move || {
//             let mut output: f32 = 0.0;

//             for event in &events {
//                 let relative_time = event.0;
//                 let relative_pitch = event.1;

//                 // bpm should be somewhere here
//                 // 88200 -> bpm 120 -> one_bar_length = 2 second
//                 if i % 88200 == (relative_time * 88200.0) as usize {
//                     // this it the sampler to trigger
//                     output = relative_pitch as f32;
//                 }
//             }
//             // let imp = (i % p == 0) as u8;
//             i += 1;
//             output
//         });
//         Self {
//             sig: Box::new(s)
//         }
//     }
// }

// impl Node for Looper {
//     fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
//         // self.onebarlength = ?
//         for o in output {
//             o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
//         }
//         // output[0].iter_mut().for_each(|s| *s = self.sig.next());
//     }
// }