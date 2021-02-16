use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData,
    NodeResult, BoxedNodeSend, EngineError, midi_or_float};

pub struct LinRange {
    out_lo: f32,
    _out_hi: f32,
    // in_lo: f32,
    // in_hi: f32,
    range: f32,
}

impl LinRange {
    pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {

        let p = paras.as_str().split(" ").collect::<Vec<&str>>();
        let para_a = p[0].to_string();
        let para_b = p[1].to_string();

        let low = midi_or_float(para_a);
        let high = midi_or_float(para_b);

        Ok((NodeData::new1( BoxedNodeSend::new( Self {
            out_lo: low,
            _out_hi: high,
            range: (high - low)
        })), vec![]))
    }
}

impl Node<128> for LinRange {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        assert!(inputs.len() > 0, "inputs len error");
        let in_buf = &mut inputs[0].buffers();
        for i in 0..128 {
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