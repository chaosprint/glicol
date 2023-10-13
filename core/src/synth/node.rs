use std::fmt::Debug;

use smallvec::SmallVec;

pub trait Process: Send {
    fn process(&mut self, inputs: &Vec<&mut Buffer>, buffer: &mut Buffer); // , context: &mut Context
    fn inspect(&self) -> String;
    fn min_channel(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub data: SmallVec<[SmallVec<[f32; 2]>; 1024]>,
}

impl Buffer {
    pub fn new(frames: usize, channels: usize) -> Self {
        Self {
            data: SmallVec::from_elem(SmallVec::from_elem(0.0, channels), frames),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    processor: Box<dyn Process>,
    pub buffer: Buffer,
}

impl Debug for dyn Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&self.inspect()).finish()
    }
}

impl Node {
    pub fn new(processor: Box<dyn Process>, frames: usize, channels: usize) -> Self {
        // let buffer = SmallVec::from_elem(SmallVec::from_elem(0.0, frames), processor.min_channel());
        let buffer = Buffer::new(frames, channels);
        Self { processor, buffer }
    }

    pub fn process(&mut self, inputs: &Vec<&mut Buffer>) {
        // println!("processing {} with {:?}", self.inspect(), inputs);
        self.processor.process(inputs, &mut self.buffer);
    }

    pub fn inspect(&self) {
        println!("{:?}", self.processor.inspect());
    }
}

#[derive(Debug, Clone)]
pub struct SinOsc {
    pub frequency: f32,
    pub phase: f32,
    pub sample_rate: u32,
}

impl SinOsc {
    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase;
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }
}

impl Process for SinOsc {
    fn process(&mut self, _inputs: &Vec<&mut Buffer>, buffer: &mut Buffer) {
        let mut phase = self.phase;
        let phase_increment = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate as f32;

        for frame in buffer.data.iter_mut() {
            let sample = phase.sin();
            phase += phase_increment;
            for c in 0..frame.len() {
                frame[c] = sample;
            }
        }
        self.phase = phase;
    }
    fn inspect(&self) -> String {
        format!("SinOsc")
    }
    fn min_channel(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct Mul {
    pub factor: f32,
}

impl Mul {
    pub fn new(factor: f32) -> Self {
        Self { factor }
    }

    pub fn set_factor(&mut self, factor: f32) {
        self.factor = factor;
    }
}

impl Process for Mul {
    fn process(&mut self, inputs: &Vec<&mut Buffer>, buffer: &mut Buffer) {
        match inputs.len() {
            1 => {
                // first iter on each frame, then each channel
                for (frame, input_frame) in buffer.data.iter_mut().zip(inputs[0].data.iter()) {
                    for (sample, input_sample) in frame.iter_mut().zip(input_frame.iter()) {
                        *sample = input_sample * self.factor;
                    }
                }
            }
            2 => {
                // first iter on each frame, then each channel
                let main_input = &inputs[0];
                let sidechain_input = &inputs[1];
                for ((frame, main_frame), sidechain_frame) in buffer
                    .data
                    .iter_mut()
                    .zip(main_input.data.iter())
                    .zip(sidechain_input.data.iter())
                {
                    for ((sample, main_sample), sidechain_sample) in frame
                        .iter_mut()
                        .zip(main_frame.iter())
                        .zip(sidechain_frame.iter())
                    {
                        *sample = main_sample * sidechain_sample;
                    }
                }
            }
            _ => {
                return;
            }
        }
    }
    fn inspect(&self) -> String {
        format!("Mul")
    }
    fn min_channel(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct Add {
    pub factor: f32,
}

impl Add {
    pub fn new(factor: f32) -> Self {
        Self { factor }
    }

    pub fn set_factor(&mut self, factor: f32) {
        self.factor = factor;
    }
}

impl Process for Add {
    fn process(&mut self, inputs: &Vec<&mut Buffer>, buffer: &mut Buffer) {
        for (frame, input_frame) in buffer.data.iter_mut().zip(inputs[0].data.iter()) {
            for (sample, input_sample) in frame.iter_mut().zip(input_frame.iter()) {
                *sample = input_sample + self.factor;
            }
        }
    }
    fn inspect(&self) -> String {
        format!("Add")
    }
    fn min_channel(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub value: f32,
}

impl Constant {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl Process for Constant {
    fn process(&mut self, _inputs: &Vec<&mut Buffer>, buffer: &mut Buffer) {
        for frame in buffer.data.iter_mut() {
            for sample in frame.iter_mut() {
                *sample = self.value;
            }
        }
    }
    fn inspect(&self) -> String {
        format!("Constant")
    }
    fn min_channel(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct Mix {}

impl Mix {
    pub fn new() -> Self {
        Self {}
    }
}

impl Process for Mix {
    fn process(&mut self, inputs: &Vec<&mut Buffer>, buffer: &mut Buffer) {
        // set buffer to zeros
        for frame in buffer.data.iter_mut() {
            for sample in frame.iter_mut() {
                *sample = 0.0;
            }
        }

        // sum inputs
        for input in inputs.iter() {
            for (frame, input_frame) in buffer.data.iter_mut().zip(input.data.iter()) {
                for (sample, input_sample) in frame.iter_mut().zip(input_frame.iter()) {
                    *sample += input_sample;
                }
            }
        }
    }
    fn inspect(&self) -> String {
        format!("Mix")
    }
    fn min_channel(&self) -> usize {
        1
    }
}

// #[derive(Debug, Clone)]
// pub struct NodeDef {
//     pub code: String,
// }

// impl NodeDef {
//     pub fn new(code: &str) -> Self {
//         Self {
//             code: code.to_string(),
//         }
//     }
// }

// impl Process for NodeDef {
//     fn process(&mut self, _inputs: &Vec<&mut Buffer>, _buffer: &mut Buffer) {}
//     fn inspect(&self) -> String {
//         self.code.clone()
//     }
//     fn min_channel(&self) -> usize {
//         1
//     }
// }
