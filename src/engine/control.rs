mod instrument;

#[derive(Clone)]
pub struct Track {
    pub amp: f32,
    pub state: String,
    pub event: Vec<Event>,
    pub ins: Vec<Box<dyn Instrument + 'static + Send>>, // waveshape?
    pub fx: Vec<Box<dyn Effect + 'static + Send>>
}

impl Track {
    pub fn new() -> Track {
        Track {
            amp: 0.0,
            state: "".to_string(),
            event: Vec::new(),
            ins: Vec::new(),
            fx: Vec::new(),
        }
    }
}

impl AsTrack for Track {
    fn yield_current_sample(&mut self, phase: usize, bpm: f32, tracks: HashMap<String, Track>) -> f32 { 
        let one_bar_length = (60.0 / bpm * 4.0 * 44100.0) as usize;
        let mut phase_in_bar = phase % one_bar_length as usize;
        let mut output = 0.0;

        for point in &self.event {
            // if phase is within the event dur
            // if event sig is empty or exausted
            // create sig and save it in where? 
            // 
            // eles yield next

            let point_start_phase = point.relative_time * one_bar_length as f32; // e.g. 0.5 * 44100
            // let relative_phase = (phase_in_bar as i32 - point_start_phase as i32 ) as usize;
            let relative_phase = (phase_in_bar as i32 - point_start_phase as i32 ) as usize;
            output += self.ins[0].yield_current_sample(relative_phase);
            phase_in_bar += one_bar_length;
            if self.fx.len() > 0 {
                for i in 0..self.fx.len() {
                    output = self.fx[i].process(output, relative_phase, bpm, tracks.clone());
                }
            }

            // this still cannot handle samples longer than 1 bar
            // while (phase_in_bar as i32 - point_start_phase as i32) < 0 {
            //     phase_in_bar += one_bar_length;
            // }; 

            // while (phase_in_bar as i32 - point_start_phase as i32) < self.ins[0].get_dur() as i32 {
            //     let relative_phase = (phase_in_bar as i32 - point_start_phase as i32 ) as usize;
            //     output += self.ins[0].yield_current_sample(relative_phase);
            //     phase_in_bar += one_bar_length;
            //     if self.fx.len() > 0 {
            //         for i in 0..self.fx.len() {
            //             output = self.fx[i].process(output, relative_phase, bpm, tracks.clone());
            //         }
            //     }
            // };
        };
        output
    }
}

pub trait AsTrack {
    fn yield_current_sample(&mut self, pha: usize,
        bpm: f32, tracks: HashMap<String, Track>) -> f32;
}

pub trait AsControlTrack {
    fn yield_current_control(&mut self, pha: usize,
        bpm: f32, tracks: HashMap<String, Track>) -> f32;
}

impl AsControlTrack for Track {
    fn yield_current_control(&mut self, phase: usize, _bpm: f32, _tracks: HashMap<String, Track>) -> f32 {
        // let mut output = 0.0;
        // let time = phase as f32 / 44100.0;
        self.ins[0].yield_current_sample(phase)
        // (2.0 * PI * time * 3.0).sin() * 900.0 + 1000.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    relative_time: f32,
    pitch: f32
}

impl Event {
    pub fn new(relative_time: f32, pitch: f32) -> Event {
        Event {
            relative_time,
            pitch
        }
    }
}
