use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use pest::iterators::Pairs;
use super::super::{Rule, EngineError, handle_params};

#[allow(dead_code)]
pub struct SinOsc {
    _freq: f32,
    phase: f32,
    clock: usize,
    buffer: Buffer,
    sidechain_ids: Vec<u8>
}

impl SinOsc {
    handle_params!({
        _freq: 440.0
    }, {
        phase: 0.0,
        clock: 0
    }, [(_freq, buffer, |_freq: f32|->Buffer {
        Buffer::default()
    })]);
}

impl Node for SinOsc {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        if inputs.len() < 2 { return () };
        let clock = inputs[1].buffers()[0][0] as usize;
        if self.clock != 0 && self.clock == clock {
            output[0] = self.buffer.clone();
            return ()
        };
        let mod_buf = &mut inputs[0].buffers();
        for i in 0..64 {
            output[0][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
            self.phase += mod_buf[0][i] / 44100.0;
            if self.phase > 1.0 {
                self.phase -= 1.0
            }
        }
        self.buffer = output[0].clone();
        self.clock = clock;
    }
}

pub struct Impulse {
    clock: usize,
    period: usize,
    // sig: Box<dyn Signal<Frame=f32> + Send>,
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

#[allow(dead_code)]
pub struct Saw {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer,
    sidechain_ids: Vec<u8>
}

impl Saw {
    handle_params!({freq: 100.0}, {phase_n: 0, clock: 0},
        [(freq, buffer, |_freq: f32|->Buffer {
            Buffer::default()
        })]);
}

impl Node for Saw {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        if inputs.len() < 2 { return () };
        let mut clock = inputs[1].buffers()[0][0] as usize;
        for i in 0..64 {
            let mod_buf = &mut inputs[0].buffers();
            if mod_buf[0][i] != 0.0 {
                self.freq = mod_buf[0][i];
            };
            let period = 44100.0 / self.freq;
            output[0][i] = (clock % period as usize) as f32
            / period *2.0-1.0;
            clock += 1;
        }
    }
}

#[allow(dead_code)]
pub struct Square {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer,
    sidechain_ids: Vec<u8>
}

impl Square {
    handle_params!({
        freq: 100.0
    }, {
        phase_n: 0,
        clock: 0
    }, [
        (freq, buffer, |_freq: f32|->Buffer {
            Buffer::default()
        })
    ]);
}

impl Node for Square {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        if inputs.len() < 2 { return () };
        let mut clock = inputs[1].buffers()[0][0] as usize;
        for i in 0..64 {
            let mod_buf = &mut inputs[0].buffers();
            if mod_buf[0][i] != 0.0 {
                self.freq = mod_buf[0][i];
            };
            let period = (44100.0 / self.freq) as usize;
            output[0][i] = ((clock%period) > (period/2))
            as u8 as f32 * 2.0 - 1.0;
            clock += 1;
        }
    }
}