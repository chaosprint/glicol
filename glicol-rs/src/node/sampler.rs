use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use super::super::{Rule, HashMap, NodeData, BoxedNodeSend, NodeResult, EngineError};

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
    ) -> NodeResult {
        let p = paras.next().unwrap();
        let key = p.as_str();
        let pos = (p.as_span().start(), p.as_span().end());
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

impl Node<128> for Sampler {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        output[0].silence();
        if inputs.len() < 2 { () };
        let mut clock = inputs[1].buffers()[0][0] as usize;
        // the input of sampler should be a pitch, and series of 0
        let input_buf = &mut inputs[0].buffers();

        for i in 0..128 {
            if input_buf[0][i] > 0.0 {
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
                            let left = (x * (self.endindex as f64)).floor();
                            let right = (x * (self.endindex as f64)).ceil();
                            (self.sample[left as usize] as f64
                            * ((x * (self.endindex as f64)) - left)
                            + self.sample[right as usize] as f64
                            * (right - (x * (self.endindex as f64)))) as f32
                        },
                        _ => 0.0
                    };
                }
            }

            // TODO: wrap bar end in sampler
            // but this may cause wierd behavior too
            // let one_bar = (240.0/self.bpm * 44100.0) as usize;
            // let near_end = (clock+1024+128) % one_bar < 128;
            // let fadeout = match near_end {
            //     true=> clock
            //     false=>1.0
            // };
            // output[0][i] *= fadeout;
            clock += 1;
        }
        self.playback.retain(|v| ((clock - v.0) as f64) <= v.1);
    }
}