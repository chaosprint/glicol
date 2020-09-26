use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};

pub struct Noise {
    sig: Box<dyn Signal<Frame=f64> + Send>
}
impl Noise {
    pub fn new() -> Self {
        let sig = signal::noise(0);
        Self {
            sig: Box::new(sig)
        }
    }
}

impl Node for Noise {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        output[0].iter_mut().for_each(|s| *s = self.sig.next() as f32);
    }
}