use crate::{
    impl_to_boxed_nodedata, BoxedNodeSend, Buffer, GlicolPara, Input, Message, Node, NodeData,
};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Arrange {
    _current_bar: usize,
    events: Vec<GlicolPara>,
    speed: f32,
    pub bpm: f32,
    sr: usize,
    pub step: usize,
    input_order: Vec<usize>,
    // sidechain_lib: HashMap<String, usize>,
}

impl Arrange {
    pub fn new(events: Vec<GlicolPara>) -> Self {
        // let mut total_circles = 0;
        // for j in 0..(self.events.len()/2) {
        //     match self.event[j*2] {
        //         GlicolPara::Number(value) => total_circles += *value,
        //         _ => {}
        //     };
        // };
        Self {
            _current_bar: 0,
            events,
            input_order: Vec::new(),
            speed: 1.0,
            bpm: 120.,
            sr: 44100,
            step: 0,
        }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }
    pub fn bpm(self, bpm: f32) -> Self {
        Self { bpm, ..self }
    }

    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Arrange {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        let bar_length = 240.0 / self.bpm * self.sr as f32 / self.speed;
        // calculate which bar we are and which input to pass to theoutput
        for (i, out) in output[0].iter_mut().enumerate() {
            let pos = self.step as f32 / bar_length;
            let mut bar_count = 0.0;
            for j in 0..(self.events.len() / 2) {
                let GlicolPara::Number(bar) = self.events[j * 2 + 1] else {
                    return;
                };

                bar_count += bar;
                if pos < bar_count {
                    let source = &inputs[&self.input_order[j]];
                    *out = source.buffers()[0][i];
                    // match &self.events[j*2] {
                    //     GlicolPara::Reference(s) => {
                    //         let source = &inputs[&self.input_order[j]];
                    //         output[0][i] = source.buffers()[0][i];
                    //     },
                    //     _ => return ()
                    // };
                    break;
                } else if j == self.events.len() / 2 - 1 {
                    self.step = 0;
                    let source = &inputs[&self.input_order[0]];
                    *out = source.buffers()[0][i];
                    // match &self.events[0] {
                    //     GlicolPara::Reference(s) => {
                    //         let source = &inputs[&self.input_order[0]];
                    //         output[0][i] = source.buffers()[0][i];
                    //     },
                    //     _ => return ()
                    // };
                }
            }
            self.step += 1;
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(i, value) => {
                self.step = 0;
                let to_push = i as usize - self.events.len();

                self.events.reserve(to_push + 1);
                self.events.extend(std::iter::repeat(GlicolPara::Number(0.0)).take(to_push));
                self.events.push(GlicolPara::Number(value));
            }
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
