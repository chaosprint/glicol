// use dasp_signal::{self as signal, Signal};
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use pest::iterators::Pairs;
use super::super::{Rule, EngineError, midi_or_float};

pub struct SinOsc {
    // pub freq: f64,
    // pub sig: Sine<ConstHz>
    pub freq: f32,
    phase: f32,
    diff: f32,
    has_mod: bool
    // pub sig: Box<dyn Signal<Frame=f64> + Send>,
}

impl SinOsc {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<
    (NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        // let mut paras = paras.next().unwrap().into_inner();

        // let freq: String = paras.next().unwrap().as_str().to_string()
        // .chars().filter(|c| !c.is_whitespace()).collect();

        let freq: String = paras.as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        if freq.parse::<f32>().is_ok() {
            let f = freq.parse::<f32>().unwrap();
            // println!("{}", f);
            return Ok((NodeData::new1(BoxedNodeSend::new(Self {
                freq: f,
                phase: 0.0, diff: 0.0, has_mod: false
            })), vec![]))
        } else {
            return Ok((NodeData::new1(BoxedNodeSend::new(Self { 
                freq: 0.0,
                phase:  0.0, diff: 0.0, has_mod: true 
            })), vec![freq]))
        }
    }
}

impl Node for SinOsc {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        
        // let freq = self.freq.parse::<f32>();
        if self.has_mod {
            // assert_eq!(inputs.len(), 1);
            // assert!(inputs.len() > 0);
            let mod_buf = &mut inputs[0].buffers();
            for i in 0..64 {
                output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                if mod_buf[0][i] != 0.0 { // doesn't make sense to have 0 freq
                    self.diff = mod_buf[0][i] / 44100.0;    
                }
                self.phase += self.diff;
                // self.phase += 440.0 / 44100.0;
                if self.phase > 1.0 {
                    self.phase -= 1.0
                }
            }
            // output[1] = output[0].clone();
            // }
        } else {

            // no mod, input[0] is the clock
            let mut clock = inputs[0].buffers()[0][0] as usize;

            for i in 0..64 {
                
                output[0][i] = (clock as f64 / (44100.0 / self.freq as f64 ) * 2.0 
                * std::f64::consts::PI).sin() as f32;
                
                clock += 1;

                // if we lost the phase, i.e. :(clock/(44100.0 / self.freq) * 2.0 
                // * std::f32::consts::PI)
                // we will get a click

                // output[0][i] = self.phase.sin();
                // self.phase += self.freq / 44100.0 * 2.0 * std::f32::consts::PI;
                // // self.phase += 220.0 / 44100.0;
                // if self.phase > 2.0 * std::f32::consts::PI {
                //     self.phase -= 2.0 * std::f32::consts::PI
                // }
            }
            // output[1] = output[0].clone();
        }
    }
}

pub struct Impulse {
    // sig: Box<dyn Signal<Frame=f32> + Send>,
    clock: usize,
    period: usize,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

impl Impulse {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        let para_a: String = paras.as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let freq = para_a.parse::<f32>()?;
        let period = (44100.0 / freq) as usize;

        // let mut i: usize = 0;
        // let s = signal::gen_mut(move || {
        //     let imp = (i % p == 0) as u8;
        //     i += 1;
        //     imp as f32
        // });
        Ok((NodeData::new1(BoxedNodeSend::new(Self {
            // sig: Box::new(s)
            clock: 0,
            period: period,
        })), vec![]))
    }
}

impl Node for Impulse {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        self.clock = inputs[0].buffers()[0][0] as usize;

        // println!("processed");
        // for o in output {
        //     o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        // }

        for i in 0..64 {
            let out = (self.clock % self.period == 0) as u8;
            output[0][i] = out as f32;
            self.clock += 1;
        }
        // assert_eq!(output[1][0], output[0][0]);
    }
}

pub struct Saw {
    phase_n: usize,
    freq: f32,
    has_sidechain: bool
}

impl Saw {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        let para_a: String = paras.as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let is_float = para_a.parse::<f32>();
        let has_sidechain = !is_float.is_ok();
        let (freq, sidechain) = match has_sidechain {
            true => (440.0, vec![para_a]),
            false => (midi_or_float(para_a), vec![])
        };

        Ok((NodeData::new1(BoxedNodeSend::new(Self {
            phase_n: 0,
            freq: freq,
            has_sidechain: has_sidechain
        })), sidechain))
    }
}

impl Node for Saw {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        for i in 0..64 {
            if self.has_sidechain {
                let mod_buf = &mut inputs[0].buffers();
                if mod_buf[0][i] != 0.0 {
                    self.freq = mod_buf[0][i];
                }
            }
            assert_ne!(self.freq, 0.0);
            let circle_len = (44100.0 / self.freq) as usize;
            output[0][i] = ((self.phase_n % circle_len) as f32 / circle_len as f32)*2.0-1.0;
            self.phase_n += 1;
        }
    }
}

pub struct Square {
    phase_n: usize,
    freq: f32,
    has_sidechain: bool
}

impl Square {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        let para_a: String = paras.next().unwrap().as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let is_float = para_a.parse::<f32>();
        let has_sidechain = !is_float.is_ok();
        let (freq, sidechain) = match has_sidechain {
            true => (440.0, vec![para_a]),
            false => (midi_or_float(para_a), vec![])
        };

        Ok((NodeData::new1(BoxedNodeSend::new(Self {
            phase_n: 0,
            freq: freq,
            has_sidechain: has_sidechain
        })), sidechain))
    }
}

impl Node for Square {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        for i in 0..64 {
            if self.has_sidechain {
                let mod_buf = &mut inputs[0].buffers();
                if mod_buf[0][i] != 0.0 {
                    self.freq = mod_buf[0][i];
                }   
            }
            let circle_len = (44100.0 / self.freq) as usize;
            output[0][i] = ((self.phase_n % circle_len) > (circle_len / 2)) as u8 as f32 * 2.0 - 1.0;
            self.phase_n += 1;
        }
    }
}