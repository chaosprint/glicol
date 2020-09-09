use dasp_signal::{self as signal, Signal};
use dasp_graph::{Buffer, Input, Node};

pub struct SinOsc {
    // pub freq: f64,
    // pub sig: Sine<ConstHz>
    pub sig: Box<dyn Signal<Frame=f64> + Send>,
}

impl SinOsc {
    pub fn new(freq: f64) -> SinOsc {
        let sig = signal::rate(44100.0).const_hz(freq).sine();
        SinOsc {
            sig: Box::new(sig)
        }
    }
}

impl Node for SinOsc {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {

        for o in output {
            o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
    }
}

pub struct Impulse {
    sig: Box<dyn Signal<Frame=f32> + Send>,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

impl Impulse {
    pub fn new(freq: f64) -> Self {
        let p = (44100.0 / freq) as usize;
        let mut i: usize = 0;
        let s = signal::gen_mut(move || {
            let imp = (i % p == 0) as u8;
            i += 1;
            imp as f32
        });
        Self {
            sig: Box::new(s)
        }
    }
}

impl Node for Impulse {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for o in output {
            o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
        // output[0].iter_mut().for_each(|s| *s = self.sig.next());
    }
}