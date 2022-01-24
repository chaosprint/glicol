use super::super::*;

// out: shape 0.0, 1.0 | 0.01, 1.0 | 0.2, 0.0
// what if some people write: out: shape 0.1, 1.0 | 0.1 0.0
pub struct Shape<const N:usize> {
    x: Vec<usize>,
    y: Vec<f32>,
    ydiff: Vec<f32>,
    points: Vec::<(f32, f32)>,
    trigger: f32,
    sr: usize,
    clock: usize,
    target_index: usize,
}

impl<const N:usize> Shape<N> {
    // pub fn new(points: Vec::<(f32, f32)>) -> GlicolNodeData<N> {

    //     mono_node!( N, Self { info } )
    // }

    pub fn new() -> Self {
        Self {
            x: vec![],
            y: vec![],
            ydiff: vec![],
            points: vec![],
            trigger: 0.0,
            sr: 44100,
            clock: 0,
            target_index: 0,
        }
    }

    pub fn points(self, points:  Vec::<(f32, f32)>) -> Self {
        println!("\n\n points {:?}", points);
        let x: Vec<usize> = points.iter().map(|x|(x.0 * self.sr as f32) as usize).collect();
        let mut y: Vec<f32> = points.iter().map(|y|y.1).collect();
        y.insert(0, 0.0);
        let ydiff: Vec<f32> = y.iter().zip(y.iter().skip(1)).map(|(cur, next)|next - cur).collect();

        println!("\n\n x, y ydiff {:?} {:?} {:?} \n\n", x, y, ydiff);

        Self {x, y, ydiff, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node! ( N, self )
    }
}

impl<const N:usize> Node<N> for Shape<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let l = inputs.len();
        let has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0 && inputs[l-1].buffers()[0][1] == 0.;
        // let input = inputs[0].buffers()[0].clone();
        if (l - has_clock as usize == 0) {
            return ()
        }
        let input = inputs[0].buffers()[0].clone();

        // self.current = 0  let target = self.x[target_index] self.clock
        if self.target_index == self.x.len() {
            output[0].iter_mut().for_each(|s| *s = self.y[self.y.len()-1]);
            return ()
        }

        for i in 0..N {
            if input[i] != 0.0 {
                self.trigger = input[i];
                self.target_index = 0;
                self.clock = 0;
            }
            if self.trigger != 0.0 {
                if self.target_index == self.x.len() {
                    output[0][i] = self.y[self.y.len()-1];
                } else {
                    output[0][i] = self.y[self.target_index] + (self.clock as f32 / self.x[self.target_index] as f32 * self.ydiff[self.target_index]);
                    if self.clock == self.x[self.target_index] {
                        self.clock = 0;
                        self.target_index += 1;
                    }
                }
                // println!("target_index {:?} {:?}", self.target_index, self.x.len());
                self.clock += 1;
            }
        }
        // println!("{:?}", output);
        // output[0].silence();
    }
}