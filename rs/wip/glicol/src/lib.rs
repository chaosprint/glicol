pub mod synth;

use synth::oscillator::{SinOsc, SawOsc};
use synth::signal::{ConstSig};

use petgraph::stable_graph::{StableDiGraph};
use dasp_graph::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

pub struct Engine<const N: usize> {
    pub graph: GlicolGraph<N>,
    pub processor: GlicolProcessor<N>,
}

impl<N> Engine<N> {
    pub fn new() -> Self {
        Self {
            graph: GlicolGraph::<N>::with_capacity(1024, 1024),
            processor: GlicolProcessor::<N>::with_capacity(1024),
        }
    }

    pub fn make_graph(ast: HashMap::<&str, Vec<GlicolNode>>) -> Result<(), E>{
        Ok(())
    }
}