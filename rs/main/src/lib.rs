pub mod util; use util::makenode;
pub mod error; pub use error::{EngineError, get_error_info};
use std::collections::HashMap;
use petgraph::{graph::NodeIndex};
use glicol_parser::{get_ast, GlicolPara}; 
use glicol_synth::{AudioContext, AudioContextConfig, NodeData, BoxedNodeSend, Buffer, Message};
use lcs_diff::{diff, DiffResult};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;

pub struct Engine<'a, const N: usize> {
    pub context: AudioContext<N>,
    code: &'static str,
    ast: HashMap<&'a str, (Vec<&'a str>, Vec<Vec<GlicolPara<'a>>>)>,
    new_ast: HashMap<&'a str, (Vec<&'a str>, Vec<Vec<GlicolPara<'a>>>)>,
    pub index_info: HashMap<&'a str, Vec<NodeIndex>>,
    pub index_info_backup: HashMap<&'a str, Vec<NodeIndex>>,
    temp_node_index: Vec<NodeIndex>,
    node_add_list: Vec<(&'a str, usize, GlicolNodeData<N>)>,
    node_remove_list: Vec<(&'a str, usize)>,
    node_update_list: Vec<(&'a str, usize, Vec<GlicolPara<'a>>)>,
    pub refpairlist: Vec<(Vec<&'a str>, &'a str, usize)>,
    pub samples_dict: HashMap<&'a str, (&'a [f32], usize)>
}

impl<const N: usize> Engine<'static, N> {
    pub fn new() -> Self {
        let context = AudioContext::<N>::new(AudioContextConfig::default());
        Self {
            context,
            ast: HashMap::new(),
            new_ast: HashMap::new(),
            code: "",
            index_info: HashMap::new(),
            index_info_backup: HashMap::new(),
            temp_node_index: vec![],
            node_add_list: vec![],
            node_remove_list: vec![],
            node_update_list: vec![],
            refpairlist: vec![],
            samples_dict: HashMap::new(),
        }
    }

    pub fn add_sample(&mut self, name:&'static str, sample: &'static [f32], channels: usize ) {
        self.samples_dict.insert(name, (sample, channels));
    }

    pub fn update(&mut self, code: &'static str) -> Result<(), EngineError>  {
        self.add_sample(r#"\808_0"#, &[0.42, 0.0], 2);
        self.code = code;
        self.parse()?;
        self.make_graph()?;
        Ok(())
    }

    // prepare the NodeData but do not do anything to the graph connection
    // get: add info, which chain, where, add what node
    // modify info
    // delete info
    // sidechain info, when handling the graph, check if all the sidechain exists
    pub fn parse(&mut self) -> Result<(), EngineError> {
        self.new_ast = get_ast(&self.code)?;
        self.node_add_list.clear();
        self.node_update_list.clear();
        self.node_remove_list.clear();
        self.refpairlist.clear(); // we recalculate all the sidechains since some index can change

        // also remove the whole chain in_old but not_in_new, after ensuring there is no problem with new stuff
        // println!("\n\nold ast {:?}\n\n new {:?}", self.ast, self.new_ast);
        for (key, node_info_tuple) in &self.new_ast {
            if self.ast.contains_key(key) {
                let old_chain = &self.ast[key].0;
                let new_chain = &node_info_tuple.0;
                let old_chain_para = &self.ast[key].1;
                let new_chain_para = &node_info_tuple.1;
                for action in diff(old_chain, new_chain) {
                    match action {
                        DiffResult::Common(v) => {
                            // let common_node_name = v.data;
                            let old_i = v.old_index.unwrap();
                            let new_i = v.new_index.unwrap();
                            println!("common {:?}", v);
                            println!("common node: old_index {:?}", old_i);
                            println!("common node: new_i {:?}", new_i);
                            println!("common node old para {:?}", old_chain_para[old_i]);
                            println!("common node new para {:?}", new_chain_para[new_i]);
                            if old_chain_para[old_i] != new_chain_para[new_i] {
                                self.node_update_list.push(
                                    (key, // which chain
                                    old_i, // where in chain
                                    new_chain_para[new_i].clone() // new paras
                                ))
                            } else {
                                let mut reflist = vec![];
                                for para in &new_chain_para[new_i] {
                                    match para {
                                        GlicolPara::Reference(v) => {
                                            reflist.push(*v);
                                        },
                                        _ => {},
                                    }
                                }
                                if !reflist.is_empty() {
                                    self.refpairlist.push((reflist, key, new_i));
                                }
                            }
                        },
                        DiffResult::Removed(v) => {
                            let old_i = v.old_index.unwrap();
                            self.node_remove_list.push((key, old_i));
                            println!("Removed {:?}", v)
                        },
                        DiffResult::Added(v) => {
                            println!("Added {:?}", v);
                            let new_i = v.new_index.unwrap();
                            let insert_i = v.new_index.unwrap();
                            let nodename = v.data;
                            let mut paras = new_chain_para[new_i].clone();
                            let (nodedata, reflist) = makenode(nodename, &mut paras, &self.samples_dict);
                            if !reflist.is_empty() {
                                self.refpairlist.push((reflist, key, insert_i));
                            }
                           
                            self.node_add_list.push((key, insert_i, nodedata));                            
                        },
                    }
                }
                // println!("diff {:?}", diff(old_chain, new_chain));
            } else {
                for i in 0..node_info_tuple.0.len() {
                    let name = node_info_tuple.0[i];
                    let mut paras = node_info_tuple.1[i].clone();
                    let (nodedata, reflist)  = makenode(name, &mut paras, &self.samples_dict);
                    if !reflist.is_empty() {
                        self.refpairlist.push((reflist, key, i));
                    }         
                    println!("self.node_add_list {:?} {}", key, i);
                    self.node_add_list.push((key, i, nodedata));
                };
            }
        }
        Ok(())
    }

    pub fn make_graph(&mut self) -> Result<(), EngineError> {

        // TODO: the order needs further testing
        // node update and node add will all influence the graph info
        // in node update, the user may provide a new error: e.g. non-exist ref
        // in node add, user may write a new node that uses non-exist sample
        self.handle_remove_chain();
        self.handle_node_remove();
        self.handle_node_add();
        self.handle_node_update();
        match self.handle_ref_check() {
            Ok(_) => {},
            Err(e) => {
                // remove the added node
                // use the old index
                for id in &self.temp_node_index {
                    self.context.graph.remove_node(*id);
                }
                self.index_info = self.index_info_backup.clone();
                return Err(e)
            }
        };
        self.handle_connection();
        self.ast = self.new_ast.clone();
        self.index_info_backup = self.index_info.clone();
        Ok(())
    }

    pub fn handle_ref_check(&self) -> Result<(), EngineError> {
        // ref pair is like (~mod -> a node [e.g key: out, pos_in_chain: 3])
        // ref check should use the new ast hashmap
        // because old ast hashmap has something that may need to be deleted
        println!("ref check {:?}", self.refpairlist);

        for refpair in &self.refpairlist {
            for refname in &refpair.0 {
                println!("ref check {} {}", self.new_ast.contains_key(refname), refname);
                if !self.new_ast.contains_key(refname) {
                    return Err(EngineError::NonExistReference(refname))
                }
            }
        }
        Ok(())
    }

    pub fn handle_remove_chain(&mut self) {
        // there are some chains show up in old_ast but not in new ast
        for key in self.ast.keys() {
            if !self.new_ast.contains_key(key) {
                println!("remove {:?}", key);
                for index in &self.index_info[key] {
                    self.context.graph.remove_node(*index);
                }
                self.index_info.remove_entry(key);       
            }
        }
    }

    pub fn handle_node_add(&mut self) {
        while !self.node_add_list.is_empty() {
            let (key, position_in_chain, nodedata) = self.node_add_list.remove(0); // for insertion, this is better
            if !self.index_info.contains_key(key) {
                self.index_info.insert(key, vec![]);
            };
            let nodeindex = self.context.graph.add_node(nodedata); // TODO: save these id, if there is an error, remove these node
            self.temp_node_index.push(nodeindex);
            if let Some(chain) = self.index_info.get_mut(key) { // TODO: backup the index_info
                chain.insert(position_in_chain, nodeindex);
            }
        }
        println!("node index map {:?}", self.index_info);
    }
    pub fn handle_node_update(&mut self) {
        while !self.node_update_list.is_empty() {
            let (key, position_in_chain, paras) = self.node_update_list.pop().unwrap(); // ok as is it not empty
            println!("handle update {:?} {:?}", key, position_in_chain);
            if let Some(chain) = self.index_info.get_mut(key) {
                for (i, para) in paras.iter().enumerate() {
                    // let node = &self.context.graph[chain[position_in_chain]].node;
                    match para {
                        GlicolPara::Number(v) => self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToNumber(i as u8, *v)),
                        GlicolPara::Reference(s) => {
                            self.refpairlist.push((vec![s], key, position_in_chain));
                        },
                        GlicolPara::Symbol(s) => {
                            self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToSamples(i as u8, self.samples_dict[*s]))
                        },
                        _ => {}
                    }
                }
                self.context.graph[
                            chain[position_in_chain]].node.send_msg(Message::ResetOrder);
                // self.context.send_msg(index: NodeIndex, msg: Message)
            }
        }
    }
    pub fn handle_node_remove(&mut self) {
        while !self.node_remove_list.is_empty() {

            // need to pop from the back so the pos is right
            let (key, position_in_chain) = self.node_remove_list.pop().unwrap();

            // println!("self.index_info {:?}", self.index_info);
            if let Some(chain) = self.index_info.get_mut(key) {
                // println!("chain {:?} position_in_chain {:?}", chain, position_in_chain);
                let node_index = chain[position_in_chain];
                self.context.graph.remove_node(node_index);
                chain.remove(position_in_chain);
            }
        }
    }

    pub fn handle_connection(&mut self) {
        self.context.graph.clear_edges();
        println!("self.index_info in handle_connection{:?}", self.index_info);
        println!("self.refpairlist in handle_connection {:?}", self.refpairlist);
        for refpairs in &self.refpairlist {
            let index = self.index_info[refpairs.1][refpairs.2];
            self.context.graph[index].node.send_msg(Message::ResetOrder);
            for refname in &refpairs.0 {
                self.context.connect(*self.index_info[refname].last().unwrap(), index);
            }
        }
        for (key, chain) in &self.index_info {
            match chain.len() {
                0 => {},
                1 => {
                    if !key.contains("~") {
                        self.context.connect_with_order(chain[0], self.context.destination, 0);
                    }
                },
                2 => {
                    self.context.connect_with_order(chain[0], chain[1], 0);
                    if !key.contains("~") {
                        self.context.connect_with_order(chain[1], self.context.destination, 0);
                    }
                },
                _ => {
                    for i in 0..chain.len() {
                        if i == chain.len() - 1 {
                            if !key.contains("~") {
                                self.context.connect_with_order(chain[i], self.context.destination, 0);
                            }
                        } else {
                            self.context.connect_with_order(chain[i],chain[i+1], 0);
                        }
                    }
                }
            }
        }

    }

    pub fn next_block(&mut self) -> &[Buffer<N>] {  //  -> &Vec<Buffer<N>> 
        self.context.processor.process(&mut self.context.graph, self.context.destination);
        // println!("result {:?}", &self.context.graph[self.context.destination].buffers);
        &self.context.graph[self.context.destination].buffers
    }
}