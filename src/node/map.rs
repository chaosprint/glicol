use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend};

pub struct LinRange {
    out_lo: f32,
    _out_hi: f32,
    // in_lo: f32,
    // in_hi: f32,
    range: f32,
}

impl LinRange {
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {
        let para_a: String = paras.next().unwrap().as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let para_b: String = paras.next().unwrap().as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let low = para_a.parse::<f32>().unwrap();
        let high = para_b.parse::<f32>().unwrap();

        assert!(high > low);

        (NodeData::new1( BoxedNodeSend::new( Self {
            out_lo: low,
            _out_hi: high,
            range: (high - low)
        })), vec![])
        
    }
}

impl Node for LinRange {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        assert!(inputs.len() > 0, "inputs len error");
        let in_buf = &mut inputs[0].buffers();
        for i in 0..64 {
            output[0][i] = (in_buf[0][i] + 1.0) / 2.0 * self.range + self.out_lo;
        }
    }
}

// pub struct ExpRange {
//     out_lo: f32,
//     out_hi: f32,
//     in_lo: f32,
//     in_hi: f32
// }