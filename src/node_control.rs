use dasp_graph::{Buffer, Input, Node};

pub struct Sequencer {
    events: Vec<(f64, f64)>,
    speed: f32,
    pub step: usize,
}

impl Sequencer {
    pub fn new(events: Vec<(f64, f64)>, speed: f32) -> Self {
        Self {
            events,
            speed,
            step: 0
        }
    }
}

impl Node for Sequencer {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        
        if inputs.len() > 0 {
            self.speed = inputs[0].buffers()[0][0];
        }
        // self.onebarlength = ?
        // let relative_time = event.0;
        // let relative_pitch = event.1; a ratio for midi 60 freq
        let bar_length = 88200.0 / self.speed as f64;
        for i in 0..64 {
            output[0][i] = 0.0;     
            for event in &self.events {
                // default bpm 120 -> 1 bar lasts 2 second, hence 88200.0
                if (self.step % (bar_length as usize)) == ((event.0 * bar_length) as usize) {
                    output[0][i] += event.1 as f32;
                }
            }
            self.step += 1;
        }
        // for o in output {
        //     o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        // }
    }
}

pub struct Speed {
    pub speed: f32
}

impl Node for Speed {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for o in output {
            o.iter_mut().for_each(|s| *s = self.speed);
        }
    }
}