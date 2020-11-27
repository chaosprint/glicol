use dasp_signal::{self as signal, Signal};
use dasp_slice::{ToFrameSlice};
use dasp_interpolate::linear::Linear;
use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use super::super::{Rule, HashMap, NodeData, BoxedNodeSend, EngineError};

pub struct Sampler {
    pub sig: Vec< Box<dyn Signal<Frame=[f32;1]> + 'static + Send>>,
    // pub sig: Box<dyn Signal<Frame=[f32;1]> + Send>,
    pub samples: &'static[f32],
}
// samples: &'static[f32]
impl Sampler {
    pub fn new(
        paras: &mut Pairs<Rule>,
        samples_dict: &HashMap<String, &'static[f32]>
    ) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        // let mut paras = paras.next().unwrap().into_inner();
        // let para_a: String = paras.next().unwrap().as_str().to_string()
        // .chars().filter(|c| !c.is_whitespace()).collect();

        let p = paras.next().unwrap();

        // println!("{:?}", p.as_span());
        let pos = (p.as_span().start(), p.as_span().end());

        let key = p.as_str();
        if !samples_dict.contains_key(key) {
            return Err(EngineError::SampleNotExistError(pos))
        }

        let samples = samples_dict[key];

        Ok((NodeData::new1(BoxedNodeSend::new(Self{
            sig: Vec::new(),
            samples: samples
        })), vec![]))
    }
}

impl Node for Sampler {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        output[0].silence();
        if inputs.len() > 0 {
            // the input of sampler should be a pitch, and series of 0
            let input_buf = &mut inputs[0].buffers();

            for i in 0..64 {
                if input_buf[0][i] > 0.0 {
                    // do it every sample, will it be too expensive?
                    if input_buf[0][i] > 0.0 {
                        let f: &[[f32;1]] = self.samples.to_frame_slice().unwrap();
                        // let s = signal::from_iter(f.iter().cloned());
                        let mut source = signal::from_iter(f.iter().cloned());
                        let a = source.next();
                        let b = source.next();
                        let interp = Linear::new(a, b);
                        let s = source.scale_hz(interp, input_buf[0][i] as f64);
                        // as f64 /2.0_f64.powf((60.0-69.0)/12.0)/440.0;
                        self.sig.push(Box::new(s));
                    }
                }
                // for i in 0..output[0].len() {
                for v in &mut self.sig {
                    if !v.is_exhausted() {
                        output[0][i] += v.next()[0];
                    }                   
                }
            }
        }
    }
}

// discarded
struct Looper {
    sig: Box<dyn Signal<Frame=f32> + Send>,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

#[allow(dead_code)]
impl Looper {
    fn new(events: Vec<(f64, f64)>) -> Self {
        // let p = (44100.0 / 10.0) as usize;
        let mut i: usize = 0;
        let s = signal::gen_mut(move || {
            let mut output: f32 = 0.0;

            for event in &events {
                let relative_time = event.0;
                let relative_pitch = event.1;

                // bpm should be somewhere here
                // 88200 -> bpm 120 -> one_bar_length = 2 second
                if i % 88200 == (relative_time * 88200.0) as usize {
                    // this it the sampler to trigger
                    output = relative_pitch as f32;
                }
            }
            // let imp = (i % p == 0) as u8;
            i += 1;
            output
        });
        Self {
            sig: Box::new(s)
        }
    }
}

impl Node for Looper {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        // self.onebarlength = ?
        for o in output {
            o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
        // output[0].iter_mut().for_each(|s| *s = self.sig.next());
    }
}