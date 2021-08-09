use super::super::{NodeData, GlicolNodeData,
    BoxedNodeSend, mono_node, Buffer, Input, Node};

#[derive(Clone, Debug, Default)]
pub struct Shape {
    xvec: Vec<f32>,
    yvec: Vec<f32>,
    curves: Vec<&'static str>,
    sr: usize,
    pos: usize,
    scale: f32,
}

impl Shape {
    pub fn new() -> Self {
        Self {sr: 44100, ..Self::default()}
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self}
    }

    pub fn xvec(self, xvec: Vec::<f32>) -> Self {
        Self {xvec, ..self}
    }

    pub fn yvec(self, yvec: Vec::<f32>) -> Self {
        Self {yvec, ..self}
    }

    pub fn curves(self, curves: Vec::<&'static str>) -> Self {
        Self {curves, ..self}
    }

    pub fn build(self) -> GlicolNodeData {
        mono_node!(self)
    }

}

#[macro_export]
macro_rules! shape {
    ({$($para: ident: $data:expr),*}) => {
         (
            Shape::new()$(.$para($data))*.build()
        )
    }
}

/// The Sequencer is probably the most complicated node in Glicol
/// The inputs can be clock, speed or many sidechains
impl Node<128> for Shape {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let buf = &mut inputs[0].buffers();
        output[0].silence();
        for i in 0..128 {
            if buf[0][i] > 0.0 {
                self.pos = 0;
                self.scale = buf[0][i];
            }

            for j in 0..self.xvec.len() - 1 {
                let start = (self.xvec[j] * self.sr as f32) as usize ;
                let end = (self.xvec[j+1] * self.sr as f32) as usize;
                // println!("{},{},{}", self.pos, start, end);
                if self.pos >= start && self.pos <= end {
                    let dur = end - start;
                    let pos = self.pos - start;
                    match self.curves[j] {
                        "0" => {
                            output[0][i] = self.yvec[j] + (self.yvec[j+1] - self.yvec[j]) * (pos as f32 / dur as f32);
                        },
                        "exp" => {
                            // given f(x0) = y0, f(x1) = y1, find f(x) = ab^x
                            // b^(x1-x0) = y1 / y0 make sure y0 is non-zero
                            // let b = (y1/y0).log(x1-x0);
                            // output[0][i] = self.yvec[j] + ((self.yvec[j+1] - self.yvec[j]) *  (pos as f32 / dur as f32)).exp() - 1.;
                        },
                        _ => {}
                    }
                }
            }
            output[0][i] *= self.scale;
            self.pos += 1;
        }
        // println!("{:?}", output[0]);
    }
}