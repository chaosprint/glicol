use std::{collections::HashMap, f32::consts::PI};

impl Clone for Box<dyn Effect + 'static + Send> {
    fn clone(&self) -> Box<dyn Effect + 'static + Send> {
        self.box_clone()
    }
}

pub trait Effect {
    fn process(&mut self, input: f32, phase: usize, bpm: f32, tracks: HashMap<String, Track>) -> f32;
    fn box_clone(&self) -> Box<dyn Effect + 'static + Send>;
}

#[derive(Clone)]
pub struct LPF {
    pub cutoff: String,
    pub q: f32,
    pub input_past: [f32;2],
    pub output_past: [f32;2]
}

impl LPF {
    pub fn new(cutoff: String, q: f32) -> LPF {
        LPF {
            cutoff,
            q,
            input_past: [0.0, 0.0],
            output_past: [0.0, 0.0]
        }
    }
}

impl Effect for LPF {
    fn box_clone(&self) -> Box<dyn Effect + 'static + Send> {
        Box::new(self.clone())
    }
    fn process(&mut self, input: f32, phase: usize, bpm: f32, tracks: HashMap<String, Track>) -> f32 {
        // assert_eq!(input_past[0], 0.0);
        // assert_eq!(output_past[0], 0.0);

        //not the best solution
        let freq: f32;
        if self.cutoff.parse::<f32>().is_ok() {
            freq = self.cutoff.parse::<f32>().unwrap();
        } else {
            if tracks.contains_key(self.cutoff.as_str()) {
                // freq = 3000.0
                freq = tracks[self.cutoff.as_str()].clone().yield_current_control(phase, bpm, tracks)
            } else {
                freq = 100.0
            }
        };
        // let time: f32 = phase as f32 / 44100.0;
        // let freq = (2.0 * PI * time * 2.0).sin() * 900.0 + 1000.0;
        
        let r: f32 = 1.0;
        let c: f32 = 1.0 / (PI * freq / 44100.0).tan();     
        let a1 = 1.0 / ( 1.0 + r * c + c * c);
        let a2 = 2.0 * a1;
        let a3 = a1;
        let b1 = 2.0 * ( 1.0 - c*c) * a1;
        let b2 = ( 1.0 - r * c + c * c) * a1;
        let output = a1 * input + a2 * self.input_past[1] + a3 * self.input_past[0]
            - b1 * self.output_past[1] - b2 * self.output_past[0];
        self.input_past[0] = self.input_past[1];
        self.input_past[1] = input;
        self.output_past[0] = self.output_past[1];
        self.output_past[1] = output;
        output
    }
}