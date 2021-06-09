use glicol_macro::*;
use glicol_audio::{SimpleGraph};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;

pub struct AmpLFO {
    graph: SimpleGraph
}

impl AmpLFO {
    pub fn new(freq: f32, low: f32, high: f32) -> Self {
        let mul = (high - low) / 2.0;
        assert!(mul > 0.0);
        assert!(low >= 0.0);
        let ad = mul + low;
        let graph = make_graph!{
            out: _input >> _am;
            _am: sin #freq >> mul #mul >> add #ad;
        };
        Self { graph }
    }
}