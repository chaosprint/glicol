mod node;
pub use node::*;
mod graph;
pub use graph::*;

// mod node2;
// pub use node2::*;

// mod graph3;
// pub use graph3::*;

// mod graph2;
// pub use graph2::*;

// fn new_node(node: Box<dyn Process>, buf_pool: &mut BufferPool) -> Node {
//     let need_buffer = node.needs_own_buffer();
//     let buf = if need_buffer {
//         Some(buf_pool.request())
//     } else {
//         None
//     };
//     Node::new(node, buf)
// }

// pub fn test() {
//     // let channels = 2;
//     // let frames = 16;
//     // let mut buf_pool = BufferPool::new(128, channels, frames);
//     let mut graph_nodes: SlotMap<DefaultKey, Node> = SlotMap::with_key();

//     let a = graph_nodes.insert(Node::new(Box::new(Constant::new(44.0)), 16));
//     let b = graph_nodes.insert(Node::new(Box::new(Mul::new(10.)), 16));
//     let c = graph_nodes.insert(Node::new(Box::new(Constant::new(20.)), 16));
//     let d = graph_nodes.insert(Node::new(Box::new(Mul::new(0.1)), 16));

//     let mut graph: HashMap<DefaultKey, SmallVec<[DefaultKey; 256]>> = HashMap::new();
//     graph.insert(a, SmallVec::from_slice(&[b]));
//     graph.insert(c, SmallVec::from_slice(&[d]));
//     graph.insert(d, SmallVec::from_slice(&[b]));

//     let mut reversed_graph: HashMap<DefaultKey, SmallVec<[DefaultKey; 256]>> = HashMap::new();
//     for (node, neighbors) in graph.iter() {
//         for &neighbor in neighbors.iter() {
//             reversed_graph
//                 .entry(neighbor)
//                 .or_insert_with(|| SmallVec::new())
//                 .push(*node);
//         }
//     }

//     let result = dfs_postorder_iter(b, &reversed_graph);

//     // for node in result.iter() {
//     //     println!("visit node: {:?}", graph_nodes[*node].inspect());
//     // }

//     // for node in result.iter() {
//     //     print!("visit node: {:?}", graph_nodes[*node].inspect());
//     //     println!("buffer: {:?}", graph_nodes[*node].buffer);

//     //     let mut inputs = Vec::new();
//     //     if let Some(neighbors) = reversed_graph.get(node) {
//     //         print!(" - neighbours: ");
//     //         let graph_nodes = &mut graph_nodes as *mut SlotMap<DefaultKey, Node>;
//     //         for &neighbor in neighbors.iter() {
//     //             // find the neighbor in the result
//     //             // and pass the output to its input
//     //             // print!("{:?}, ", graph_nodes[neighbor].inspect());
//     //             // let buf = &mut graph_nodes[neighbor].buffer as *mut Buffer;
//     //             let g = &mut unsafe { *graph_nodes };
//     //             let n = &mut g[neighbor];
//     //             let buf = &mut n.buffer;
//     //             inputs.push(buf);
//     //         }
//     //     }
//     //     graph_nodes[*node].process(inputs);
//     //     println!();
//     // }
//     for node in result.iter() {
//         print!("visit node: {:?}", graph_nodes[*node].inspect());
//         println!("buffer: {:?}", graph_nodes[*node].buffer);

//         let mut inputs = Vec::new();
//         if let Some(neighbors) = reversed_graph.get(node) {
//             print!(" - neighbours: ");
//             let graph_nodes_ptr = &mut graph_nodes as *mut SlotMap<DefaultKey, Node>;
//             for &neighbor in neighbors.iter() {
//                 let buf = unsafe { &mut (*graph_nodes_ptr)[neighbor].buffer };
//                 inputs.push(buf);
//             }
//         }
//         graph_nodes[*node].process(&inputs);
//         println!();
//     }

//     println!("final buffer: {:?}", graph_nodes[b].buffer);
// }
