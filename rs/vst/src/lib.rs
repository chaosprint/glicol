extern crate vst;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::plugin_main;
use vst::plugin::{Category, HostCallback, Info, Plugin};
use glicol_synth::{SimpleGraph};
use glicol_parser::{Rule, GlicolParser};

struct HelloGlicol {
    graph: SimpleGraph<128>,
    sample_rate: usize
}

impl std::default::Default for HelloGlicol {
    fn default() -> Self {
        HelloGlicol {
            graph: SimpleGraph::<128>::new("o: sin 440.0"),
            sample_rate: 44100
        }
    }
}

impl Plugin for HelloGlicol {
    fn new(_host: HostCallback) -> Self {
        let graph = SimpleGraph::<128>::new("o: sin 440.0");
        HelloGlicol {
            graph,
            sample_rate: 44100
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "HelloGlicol".to_string(),
            vendor: "chaosprint".to_string(),
            unique_id: 8888,
            category: Category::Synth,
            inputs: 1,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }
    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate as usize;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples: usize = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();
        
        let process_times = samples / 128;

        let mut out = vec![0.0; samples];
        let mut index = 0;
        for _ in 0..process_times {
            let o = self.graph.next_block(&mut [0.0; 128]);
            for i in 0..128 {
                out[index] = o[i];
                index += 1;
            }
        }

        for sample_idx in 0..samples {
            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                buff[sample_idx] = out[sample_idx];
            }
        }
    }
}

plugin_main!(HelloGlicol);