// it is possible to denote a node id
// we use the counter example again

use petgraph::stable_graph::{StableDiGraph};
use glicol_synth::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

#[derive(Debug, Copy, Clone)]
struct Counter<const N:usize> { n: usize }

impl<const N:usize> Counter<N> {
    pub fn new() -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( Self { n:0 } ) )
    }
}

impl<const N:usize> Node<N> for Counter<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.n as f32;
            self.n += 1;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 {
            self.n = info.1.parse::<f32>().unwrap() as usize;
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Powf<const N:usize> { 
    val: f32,
    main_input_node_id: usize,
    sidechain_node_id: usize, // in this example, we don't need more sidechain
}

impl<const N:usize> Powf<N> {
    pub fn new(val: f32) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( Self { val, main_input_node_id: 0, sidechain_node_id: 1 } ) )
    }
}

impl<const N:usize> Node<N> for Powf<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                for i in 0..N {
                    output[0][i] = inputs[0].buffers()[0][i].powf(self.val);
                }
            },
            2 => {
                let mut main = &inputs[0]; 
                let mut sidechain = &inputs[1];

                // match inputs[0].node_id {
                //     self.main_input_node_id => { },
                //     self.sidechain_node_id => {},
                //     _ => {}
                // };

                // match inputs[0].node_id {
                //     self.main_input_node_id => { },
                //     self.sidechain_node_id => {},
                //     _ => {}
                // };

                if self.main_input_node_id == inputs[0].node_id {
                    main = &inputs[0]
                } else if self.sidechain_node_id == inputs[0].node_id {
                    sidechain = &inputs[0]
                }

                if self.main_input_node_id == inputs[1].node_id {
                    main = &inputs[1]
                } else if self.sidechain_node_id == inputs[1].node_id  {
                    sidechain = &inputs[1]
                }

                println!("inputs[0] {}", inputs[0].node_id);
                println!("inputs[1] {}", inputs[1].node_id);
                // let main = match inputs[0] {

                // }
                for i in 0..N {
                    output[0][i] = main.buffers()[0][i].powf(sidechain.buffers()[0][i]);
                }
            },
            _ => {}
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        match info.1 {
            "main" => {
                self.main_input_node_id = info.0 as usize;
            } // is the main input,
            "sidechain" => {
                self.sidechain_node_id = info.0 as usize;
            },
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ConstSig<const N:usize> { val: f32 }

impl<const N:usize> ConstSig<N> {
    pub fn new(val: f32) -> NodeData<BoxedNodeSend<N>, N> {
        // NodeData::new1( Self {val} )
        NodeData::new1( BoxedNodeSend::<N>::new( Self {val} ) )
    }
}

impl<const N:usize> Node<N> for ConstSig<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.val;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 {
            self.val = info.1.parse::<f32>().unwrap();
        }
    }
}

pub enum Msg {
    InputOrder((u8, usize))
}

fn main() {
    let mut graph = GlicolGraph::<128>::with_capacity(1024, 1024);

    let node_index_a = graph.add_node( Counter::<128>::new());
    let node_index_b = graph.add_node( Powf::<128>::new(0.5));
    let node_index_c = graph.add_node( ConstSig::<128>::new(2.)); // this 42 will overide 0.5


    // we don't need the edge index here 
    let _e1 = graph.add_edge(node_index_a, node_index_b, ());
    let _e2 = graph.add_edge(node_index_c, node_index_b, ());

    // we can tell node b which is the main input, and which one is the sidechain
    graph[node_index_b].node.send_msg((node_index_c.index() as u8, "sidechain"));

    // comment out the line above and try this instead
    // graph[node_index_b].node.send_msg((node_index_c.index() as u8, "main"));

    let mut processor = GlicolProcessor::with_capacity(1024);
    processor.process(&mut graph, node_index_b);
    println!("node_index_a result {:?}", graph[node_index_b].buffers);

}