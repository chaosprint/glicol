pub struct Impulse {
    clock: usize,
    period: usize,
    // sig: Box<dyn Signal<Frame=f32> + Send>,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

// impl Impulse {
//     pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {

//         let para_a: String = paras.as_str().to_string()
//         .chars().filter(|c| !c.is_whitespace()).collect();
//         let p = paras.next().unwrap();
//         let pos = (p.as_span().start(), p.as_span().end());

//         let freq = match para_a.parse::<f32>() {
//             Ok(v) => v,
//             Err(_) => return Err(EngineError::ParameterError(pos))
//         };
//         let period = (44100.0 / freq) as usize;

//         // let mut i: usize = 0;
//         // let s = signal::gen_mut(move || {
//         //     let imp = (i % p == 0) as u8;
//         //     i += 1;
//         //     imp as f32
//         // });
//         Ok((NodeData::new1(BoxedNodeSend::new(Self {
//             // sig: Box::new(s)
//             clock: 0,
//             period: period,
//         })), vec![]))
//     }
// }

impl Node<128> for Impulse {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {

        self.clock = inputs[0].buffers()[0][0] as usize;
        // println!("processed");
        // for o in output {
        //     o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        // }
        for i in 0..128 {
            let out = (self.clock % self.period == 0) as u8;
            output[0][i] = out as f32;
            self.clock += 1;
        }
    }
}