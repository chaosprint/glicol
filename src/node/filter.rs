use dasp_graph::{Buffer, Input, Node};
// use dasp_signal::{self as signal, Signal};

pub struct LPF {
    cutoff: f32,
    q: f32,
    has_mod: bool,
    x0: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}
impl LPF {
    pub fn new(cutoff: String, q: f32) -> Self {
        // let sig = signal::noise(0);
        let para = cutoff.parse::<f32>();
        if para.is_ok() {
            Self {
                cutoff: para.unwrap(),
                q, has_mod: false,
                x0: 0.,
                x1: 0.,
                x2: 0.,
                y1: 0.,
                y2: 0.,
            }
        } else {
            Self {
                cutoff: 1000.0,
                q, has_mod: true,
                x0: 0.,
                x1: 0.,
                x2: 0.,
                y1: 0.,
                y2: 0.,
            }
        }
    }
}

impl Node for LPF {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if !self.has_mod {
            let theta_c = 2.0 * std::f32::consts::PI * self.cutoff / 44100.0;
            let d = 1.0 / self.q;
            let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
            let gama = (0.5 + beta) * theta_c.cos();
            let a0 = (0.5 + beta - gama) / 2.0;
            let a1 = 0.5 + beta - gama;
            let a2 = (0.5 + beta - gama) / 2.0;
            let b1 = -2.0 * gama;
            let b2 = 2.0 * beta;
            // let c0 = 1.0;
            // let d0 = 0.0;
            // let y = 
            if inputs.len() > 0 {
                for i in 0..64 {
                    // output[0][i] = inputs[0].buffers()[0][i];
                    let x0 = inputs[0].buffers()[0][i];
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            }
        } else {
            if inputs.len() > 1 {
                // let input_buf = &mut inputs[0].buffers();
                // let mod_buf = &mut inputs[1].buffers();

                // // mod_buf[0][i]   
                let theta_c = 2.0 * std::f32::consts::PI * inputs[0].buffers()[0][0] / 44100.0;
                let d = 1.0 / self.q;
                let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
                let gama = (0.5 + beta) * theta_c.cos();
                let a0 = (0.5 + beta - gama) / 2.0;
                let a1 = 0.5 + beta - gama;
                let a2 = (0.5 + beta - gama) / 2.0;
                let b1 = -2.0 * gama;
                let b2 = 2.0 * beta;

                for i in 0..64 {
                    // output[0][i] = inputs[0].buffers()[0][i];  
                    let x0 = inputs[1].buffers()[0][i]; // strange order hard code
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            }
        }
    }
}

pub struct HPF {
    cutoff: f32,
    q: f32,
    has_mod: bool,
    x0: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl HPF {
    pub fn new(cutoff: String, q: f32) -> Self {
        // let sig = signal::noise(0);
        let para = cutoff.parse::<f32>();
        if para.is_ok() {
            Self {
                cutoff: para.unwrap(),
                q, has_mod: false,
                x0: 0.,
                x1: 0.,
                x2: 0.,
                y1: 0.,
                y2: 0.,
            }
        } else {
            Self {
                cutoff: 1000.0,
                q, has_mod: true,
                x0: 0.,
                x1: 0.,
                x2: 0.,
                y1: 0.,
                y2: 0.,
            }
        }
    }
}

impl Node for HPF {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if !self.has_mod {
            let theta_c = 2.0 * std::f32::consts::PI * self.cutoff / 44100.0;
            let d = 1.0 / self.q;
            let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
            let gama = (0.5 + beta) * theta_c.cos();
            let a0 = (0.5 + beta + gama) / 2.0;
            let a1 = -0.5 - beta - gama;
            let a2 = (0.5 + beta + gama) / 2.0;
            let b1 = -2.0 * gama;
            let b2 = 2.0 * beta;
            // let c0 = 1.0;
            // let d0 = 0.0;
            // let y = 
            if inputs.len() > 0 {
                for i in 0..64 {
                    // output[0][i] = inputs[0].buffers()[0][i];
                    let x0 = inputs[0].buffers()[0][i];
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            }
        } else {
            if inputs.len() > 1 {
                // let input_buf = &mut inputs[0].buffers();
                // let mod_buf = &mut inputs[1].buffers();

                // // mod_buf[0][i]
                // inputs[0] is the modulator... strange
                let theta_c = 2.0 * std::f32::consts::PI * inputs[0].buffers()[0][0] / 44100.0;
                let d = 1.0 / self.q;
                let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
                let gama = (0.5 + beta) * theta_c.cos();
                let a0 = (0.5 + beta + gama) / 2.0;
                let a1 = -0.5 - beta - gama;
                let a2 = (0.5 + beta + gama) / 2.0;
                let b1 = -2.0 * gama;
                let b2 = 2.0 * beta;

                for i in 0..64 {
                    // output[0][i] = inputs[0].buffers()[0][i];  
                    let x0 = inputs[1].buffers()[0][i]; // strange order hard code
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            }
        }
    }
}