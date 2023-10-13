use super::node::*;
use anyhow::{bail, Context};
use hashbrown::HashMap;
use rhai::Engine;
use slotmap::{DefaultKey, SlotMap};
use smallvec::SmallVec;

#[derive(Debug)]
pub struct Graph {
    pub graph_nodes: SlotMap<DefaultKey, Node>,
    pub graph: HashMap<DefaultKey, SmallVec<[DefaultKey; 256]>>,
    reversed_graph: HashMap<DefaultKey, SmallVec<[DefaultKey; 256]>>,
    pub frames: usize,
    pub channels: usize,
    pub destination: DefaultKey,
    pub process_order: Vec<DefaultKey>,
    pub rhai_engine: Engine,
}

impl Graph {
    pub fn new(frames: usize, channels: usize) -> Self {
        let mut graph_nodes = SlotMap::new();
        let destination = graph_nodes.insert(Node::new(Box::new(Mix::new()), frames, channels));
        Graph {
            graph_nodes,
            frames,
            channels,
            graph: HashMap::new(),
            reversed_graph: HashMap::new(),
            destination,
            process_order: Vec::new(),
            rhai_engine: Engine::new(),
        }
    }
    pub fn add_node(&mut self, processor: Box<dyn Process>) -> DefaultKey {
        let key = self
            .graph_nodes
            .insert(Node::new(processor, self.frames, self.channels));

        key
    }

    pub fn add_edge(&mut self, from: DefaultKey, to: DefaultKey) -> anyhow::Result<()> {
        if self.graph_nodes.get(from).is_none() {
            bail!("Node on the left {:?} does not exist", from);
        } else if self.graph_nodes.get(to).is_none() {
            bail!("Node on the right {:?} does not exist", to);
        }
        self.graph.entry(from).or_default().push(to);
        self.find_dfs_postorder_order()
            .context("Failed to find dfs postorder order")?;
        Ok(())
    }

    pub fn find_dfs_postorder_order(&mut self) -> anyhow::Result<()> {
        self.reversed_graph = HashMap::<DefaultKey, SmallVec<[DefaultKey; 256]>>::new();
        for (node, neighbors) in self.graph.iter() {
            for &neighbor in neighbors.iter() {
                self.reversed_graph
                    .entry(neighbor)
                    .or_insert_with(|| SmallVec::new())
                    .push(*node);
            }
        }

        self.process_order = dfs_postorder_iter(self.destination, &self.reversed_graph);
        Ok(())
    }

    pub fn connect_to_destination(&mut self, from: DefaultKey) -> anyhow::Result<()> {
        if self.graph_nodes.get(from).is_none() {
            bail!("Node on the left {:?} does not exist", from);
        }
        self.graph.entry(from).or_default().push(self.destination);
        self.find_dfs_postorder_order()
            .context("Failed to find dfs postorder order")?;
        Ok(())
    }

    pub fn yield_next_buffer(&mut self) -> anyhow::Result<&Buffer> {
        let result = self.process_order.iter();

        for node in result {
            let mut inputs = Vec::new();
            if let Some(neighbors) = self.reversed_graph.get(node) {
                let graph_nodes_ptr = &mut self.graph_nodes as *mut SlotMap<DefaultKey, Node>;
                for &neighbor in neighbors.iter() {
                    let buf = unsafe { &mut (*graph_nodes_ptr)[neighbor].buffer };
                    inputs.push(buf);
                }
            }
            self.graph_nodes[*node].process(&inputs);
        }
        Ok(&self.graph_nodes[self.destination].buffer)
    }
}

pub fn dfs_postorder_iter(
    start_node: DefaultKey,
    graph: &HashMap<DefaultKey, SmallVec<[DefaultKey; 256]>>,
) -> Vec<DefaultKey> {
    let mut stack = Vec::new();
    let mut visited = HashMap::new();
    let mut result = Vec::new();

    stack.push(start_node);

    while let Some(node) = stack.pop() {
        if !visited.get(&node).unwrap_or(&false) {
            visited.insert(node, true);
            result.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors.iter().rev() {
                    stack.push(neighbor);
                }
            }
        }
    }

    result.reverse();
    result
}
