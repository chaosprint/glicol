use std::fmt::Debug;

use smallvec::SmallVec;

pub trait Process<const N: usize, const CH: usize>: Send {
    fn process(&mut self, inputs: &Vec<&mut Buffer<N, CH>>, buffer: &mut Buffer<N, CH>); // , context: &mut Context
    fn inspect(&self) -> String;
    fn min_channel(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Buffer<const N: usize, const CH: usize> {
    pub data: [[f32; N]; CH],
    pub channels: usize,
}

impl<const N: usize, const CH: usize> Buffer<N, CH> {
    pub fn new(channels: usize) -> Self {
        Self {
            data: [[0.0; N]; CH],
            channels,
        }
    }
}

// pub type Buffer<const N: usize, const CH: usize> = [[f32; N]; CH]; // limit to 8 channels
//                                                                    // pub type Buffer = SmallVec<[SmallVec<[f32; 1024]>; 2]>; // limit to 8 channels

// TODO: the buffer pool will create lots of syntax noise
// we need to test if it's worth it
// pub struct BufferPool {
//     pub buffers: Vec<Buffer>,
//     pub count: usize,
//     pub frames: usize,
//     pub channels: usize,
// }

// impl BufferPool {
//     pub fn new(n: usize, frames: usize, channels: usize) -> Self {
//         let buffer = SmallVec::from_elem(SmallVec::from_elem(0.0, frames), channels);
//         Self {
//             buffers: vec![buffer; n],
//             count: 0,
//             frames,
//             channels,
//         }
//     }
//     pub fn request(&mut self) -> &mut Buffer {
//         if self.count < self.buffers.len() {
//             let buffer = &mut self.buffers[self.count];
//             self.count += 1;
//             buffer
//         } else if self.count == self.buffers.len() {
//             self.buffers.push(SmallVec::from_elem(
//                 SmallVec::from_elem(0.0, self.frames),
//                 self.channels,
//             ));
//             self.count += 1;
//             let buffer = &mut self.buffers[self.count];
//             buffer
//         } else {
//             panic!("BufferPool is full");
//         }
//     }
// }

#[derive(Debug)]
pub struct Node<const N: usize, const CH: usize> {
    processor: Box<dyn Process<N, CH>>,
    pub buffer: Buffer<N, CH>,
}

impl<const N: usize, const CH: usize> Debug for dyn Process<N, CH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&self.inspect()).finish()
    }
}

impl<const N: usize, const CH: usize> Node<N, CH> {
    pub fn new(processor: Box<dyn Process<N, CH>>, channels: usize) -> Self {
        // let buffer = SmallVec::from_elem(SmallVec::from_elem(0.0, frames), processor.min_channel());
        let buffer = Buffer::<N, CH>::new(channels);
        Self { processor, buffer }
    }

    pub fn process(&mut self, inputs: &Vec<&mut Buffer<N, CH>>) {
        // println!("processing {} with {:?}", self.inspect(), inputs);
        self.processor.process(inputs, &mut self.buffer);
    }

    pub fn inspect(&self) -> String {
        self.processor.inspect()
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

impl<const N: usize, const CH: usize> Process<N, CH> for SinOsc {
    fn process(&mut self, _inputs: &Vec<&mut Buffer<N, CH>>, buffer: &mut Buffer<N, CH>) {
        let mut phase = self.phase;
        let phase_increment = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate as f32;

        for i in 0..N {
            let sample = phase.sin();
            phase += phase_increment;
            for c in 0..buffer.channels {
                buffer.data[c][i] = sample;
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

impl<const N: usize, const CH: usize> Process<N, CH> for Mul {
    fn process(&mut self, inputs: &Vec<&mut Buffer<N, CH>>, buffer: &mut Buffer<N, CH>) {
        match inputs.len() {
            1 => {
                for (channel, input) in buffer.data.iter_mut().zip(inputs[0].data.iter()) {
                    for (sample, input_sample) in channel.iter_mut().zip(input.iter()) {
                        *sample = input_sample * self.factor;
                    }
                }
            }
            2 => {
                for ((channel, input1), input2) in buffer
                    .data
                    .iter_mut()
                    .zip(inputs[0].data.iter())
                    .zip(inputs[1].data.iter())
                {
                    for ((sample, input_sample1), input_sample2) in
                        channel.iter_mut().zip(input1.iter()).zip(input2.iter())
                    {
                        *sample = input_sample1 * input_sample2;
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

impl<const N: usize, const CH: usize> Process<N, CH> for Add {
    fn process(&mut self, inputs: &Vec<&mut Buffer<N, CH>>, buffer: &mut Buffer<N, CH>) {
        for (channel, input) in buffer.data.iter_mut().zip(inputs[0].data.iter()) {
            for (sample, input_sample) in channel.iter_mut().zip(input.iter()) {
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

impl<const N: usize, const CH: usize> Process<N, CH> for Constant {
    fn process(&mut self, _inputs: &Vec<&mut Buffer<N, CH>>, buffer: &mut Buffer<N, CH>) {
        for channel in buffer.data.iter_mut() {
            for sample in channel.iter_mut() {
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

impl<const N: usize, const CH: usize> Process<N, CH> for Mix {
    fn process(&mut self, inputs: &Vec<&mut Buffer<N, CH>>, buffer: &mut Buffer<N, CH>) {
        // set buffer to zeros
        for channel in buffer.data.iter_mut() {
            for sample in channel.iter_mut() {
                *sample = 0.0;
            }
        }

        // sum inputs
        for input in inputs.iter() {
            for (channel, input_channel) in buffer.data.iter_mut().zip(input.data.iter()) {
                for (sample, input_sample) in channel.iter_mut().zip(input_channel.iter()) {
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
