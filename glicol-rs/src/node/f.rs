

type Fixed = ring_buffer::Fixed<Vec<f32>>;

#[allow(dead_code)]
pub struct Allpass {
    delay: f32,
    decay: f32,
    // gain: f32,
    bufx: Fixed,
    bufy: Fixed,
    sidechain_ids: Vec::<u8>
}

impl Allpass {

    handle_params!(
        {
            delay: 0.5,
            decay: 2.0
        }, [
            (
                delay, bufx, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            ), (
                delay, bufy, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            )
        ]
    );
    // pub fn new(paras: &mut Pairs<Rule>) -> 
    // Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

    //     let para_a: String = paras.next().unwrap().as_str().to_string();
    //     let para_b: String = paras.next().unwrap().as_str().to_string();
    //     // let para_c: String = paras.next().unwrap().as_str().to_string();

    //     let delay = para_a.parse::<f32>()?;
    //     let decay = para_b.parse::<f32>()?;
    //     // let gain = para_c.parse::<f32>()?;

    //     let mut sidechains = Vec::<String>::new();
    //     let mut which_param = Vec::<u8>::new();
    //     if !para_a.parse::<f32>().is_ok() {sidechains.push(para_a); which_param.push(0)};
    //     if !para_b.parse::<f32>().is_ok() {sidechains.push(para_b); which_param.push(1)};
    //     // if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c); which_param.push(2)};
    //     let size = (delay / 1000.0 * 44100.0) as usize;

    //     Ok((NodeData::new1( BoxedNodeSend::new( Self {
    //         delay: delay,
    //         decay: decay,
    //         bufx: ring_buffer::Fixed::from(vec![0.0; size]),
    //         bufy: ring_buffer::Fixed::from(vec![0.0; size]),
    //         // gain: gain,
    //         // control: which_param
    //         sidechain_ids: vec![]
    //     })), sidechains))
    // }
}

impl Node<128> for Allpass {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // output[0] = inputs[0].buffers()[0].clone();

        // y(n) = -a * x(n) + x(n-D) + a * y(n-D)
        // a = exp(log(0.001) * D/t60).
        // let decay = (self.decay * 44100.0) as usize;
        let a = (0.001_f32.log10() * (self.delay / self.decay)).exp();

        for i in 0..128 {
            // println!("{:?}", self.buf);
            let xn = inputs[0].buffers()[0][i];
            let yn = -a as f32 * xn + self.bufx[0] + a as f32 * self.bufy[0];
            // save new input to ring buffer
            self.bufx.push(xn);
            self.bufy.push(yn);
            output[0][i] = yn;
        }
    }
}

#[allow(dead_code)]
pub struct Comb {
    delay_time: f32,
    gain: f32,
    forward: f32,
    back: f32,
    bufx: Fixed,
    bufy: Fixed,
    sidechain_ids: Vec::<u8>
}

impl Comb {
    handle_params!(
        {
            delay_time: 600.0,
            gain: 0.5,
            forward: 0.5,
            back: 0.5
        }, [
            (
                delay_time, bufx, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            ), (
                delay_time, bufy, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            )
        ]
    );
    // pub fn new(paras: &mut Pairs<Rule>) -> 
    // Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

    //     let para_a: String = paras.next().unwrap().as_str().to_string();
    //     let para_b: String = paras.next().unwrap().as_str().to_string();
    //     let para_c: String = paras.next().unwrap().as_str().to_string();
    //     let para_d: String = paras.next().unwrap().as_str().to_string();
        
    //     let delay_time = para_a.parse::<f64>()?;
    //     let gain = para_b.parse::<f64>()?;
    //     let forward = para_c.parse::<f64>()?;
    //     let back = para_d.parse::<f64>()?;

    //     let mut sidechains = Vec::<String>::new();
    //     if !para_a.parse::<f64>().is_ok() {sidechains.push(para_a);};
    //     if !para_b.parse::<f64>().is_ok() {sidechains.push(para_b);};
    //     if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c);};
    //     if !para_d.parse::<f64>().is_ok() {sidechains.push(para_d);};
        
    //     // if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c); which_param.push(2)};
    //     let size = (delay_time / 1000.0 * 44100.0) as usize;

    //     Ok((NodeData::new1( BoxedNodeSend::new( Self {
    //         delay_time,
    //         gain,
    //         forward,
    //         back,
    //         bufx: ring_buffer::Fixed::from(vec![0.0; size]),
    //         bufy: ring_buffer::Fixed::from(vec![0.0; size]),
    //     })), sidechains))
    // }
}

impl Node<128> for Comb {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // output[0] = inputs[0].buffers()[0].clone();

        let a = self.gain;
        let b = self.forward;
        let c = self.back;
        // println!("{:?}",self.bufx);

        for i in 0..128 {
            let xn = inputs[0].buffers()[0][0];
            let xn_d = self.bufx[0];
            let yn_d = self.bufy[0];
            let yn = a * xn + b * xn_d + c * yn_d;
            self.bufx.push(xn);
            self.bufy.push(yn);
            output[0][i] = yn;
        }
       

    }
}

pub struct OnePole {
    sidechain_ids: Vec<u8>,
    a: f32,
    y1: f32
}

impl OnePole {
    handle_params!({
        a: 0.9
    }, {
        y1: 0.0
    });
}

impl Node<128> for OnePole {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        match self.sidechain_ids.len() {
            0 => {
                let input_sig = inputs[0].buffers()[0].clone();
                for i in 0..128 {
                    let y = input_sig[i] + self.a * self.y1;
                    output[0][i] = y;
                    self.y1 = y;
                }
            },
            1 => {
                let modulator = inputs[0].buffers()[0].clone();
                let input_sig = inputs[1].buffers()[0].clone();
                for i in 0..128 {
                    let y = input_sig[i] + modulator[i] * self.y1;
                    output[0][i] = y;
                    self.y1 = y;
                }
            },
            _ => unimplemented!()
        };
    }
}

#[allow(dead_code)]
pub struct AllpassGain {
    delay: f32,
    a: f32,
    bufx: Fixed,
    bufy: Fixed,
    sidechain_ids: Vec::<u8>
}

impl AllpassGain {
    handle_params!({
        delay: 5000.0,
        a: 0.5
    }, [
        (
            delay, bufx, |d: f32| -> Fixed {
                let size = (d / 1000.0 * 44100.0) as usize;
                ring_buffer::Fixed::from(vec![0.0; size])
            }
        ), (
            delay, bufy, |d: f32| -> Fixed {
                let size = (d / 1000.0 * 44100.0) as usize;
                ring_buffer::Fixed::from(vec![0.0; size])
            }
        )
    ]);
}

impl Node<128> for AllpassGain {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        match self.sidechain_ids.len() {
            0 => {
                for i in 0..128 {
                    // println!("{:?}", self.buf);
                    let xn = inputs[0].buffers()[0][i];
                    let yn = -self.a * xn
                    + self.bufx[0] + self.a * self.bufy[0];
                    
                    // save new input to ring buffer
                    self.bufx.push(xn);
                    self.bufy.push(yn);
                    output[0][i] = yn;
                }
            },
            1 => {
                let insig = inputs[1].buffers()[0].clone();
                let modulator = inputs[0].buffers()[0].clone();
                let new_delay_samples = (modulator[0] / 44100.0) as usize;
                let length = self.bufx.len();
                
                for i in 0..128 {
                    // println!("{:?}", self.buf);
                    let xn = insig[i];
                    let yn = -self.a * xn
                    + self.bufx[0] + self.a * self.bufy[0];
                    
                    // save new input to ring buffer
                    self.bufx.push(xn);
                    self.bufy.push(yn);
                    output[0][i] = yn;
                    self.bufx.set_first(length - new_delay_samples);
                    self.bufy.set_first(length - new_delay_samples);
                }
            },
            _ => {}
        }
    }
}