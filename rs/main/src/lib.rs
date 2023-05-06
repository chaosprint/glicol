// todo: When error, the error info still updates some nodes..

pub mod util; use util::makenode;
pub mod error; pub use error::{EngineError, get_error_info};
use hashbrown::HashMap;
use petgraph::{graph::NodeIndex};
use glicol_parser::{get_ast}; 
use glicol_synth::{AudioContext, AudioContextConfig, NodeData, BoxedNodeSend, Buffer, Message, GlicolPara, Pass};
use lcs_diff::{diff, DiffResult};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;

pub struct Engine<const N: usize> {
    pub context: AudioContext<N>,
    code: String,
    ast: HashMap<String, (Vec<String>, Vec<Vec<GlicolPara>>)>,
    new_ast: HashMap<String, (Vec<String>, Vec<Vec<GlicolPara>>)>,
    pub index_info: HashMap<String, Vec<NodeIndex>>,
    pub index_info_backup: HashMap<String, Vec<NodeIndex>>,
    temp_node_index: Vec<NodeIndex>, // created in the adding process, will be deleted if err
    node_add_list: Vec<(String, usize, GlicolNodeData<N>)>,
    node_remove_list: Vec<(String, usize)>,
    node_index_to_remove: Vec<NodeIndex>, // stored in order not to touch the graph if err
    node_update_list: Vec<(String, usize, Vec<GlicolPara>)>,
    pub refpairlist: Vec<(Vec<String>, String, usize)>,
    pub samples_dict: HashMap<String, (&'static [f32], usize, usize)>,
    bpm: f32,
    sr: usize,
    track_amp: f32,
    seed: usize,
    clock: usize,
    pub livecoding: bool,
    need_update: bool,
}

impl<const N: usize> Engine<N> {
    pub fn new() -> Self {
        let mut context = AudioContext::<N>::new(AudioContextConfig::default());
        let index = context.add_stereo_node( Pass {});
        let mut index_info = HashMap::new();
        index_info.insert(format!("~input"), vec![index]);
        Self {
            context,
            ast: HashMap::new(),
            new_ast: HashMap::new(),
            code: "".to_owned(),
            index_info: index_info.clone(),
            index_info_backup: index_info.clone(),
            temp_node_index: vec![],
            node_add_list: vec![],
            node_remove_list: vec![],
            node_index_to_remove: vec![],
            node_update_list: vec![],
            refpairlist: vec![],
            samples_dict: HashMap::new(),
            bpm: 120.,
            sr: 44100,
            track_amp: 1.0,
            seed: 42,
            clock: 0,
            livecoding: true,
            need_update: false,
        }
    }

    pub fn send_msg(&mut self, msg: &str) {
        let commands: String = msg.chars().filter(|c| !c.is_whitespace()).collect::<_>();
        for command in commands.split(";") {
            if command == "" {
                continue
            } else {
                let list = command.split(",").collect::<Vec<_>>();
                if list.len() < 4 {
                    continue; // todo: this should be an error
                }
                let chain_name = list[0];
                let chain_pos = match list[1].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => 0
                };
                let param_pos = match list[2].parse::<u8>() {
                    Ok(v) => v,
                    Err(_) => 0
                };
                if self.index_info.contains_key(chain_name) {
                    match list[3].parse::<f32>(){
                        // todo: check the name and pos
                        Ok(v) => {
                            self.context.graph[
                                self.index_info[chain_name][chain_pos]
                            ].node.send_msg(Message::SetToNumber(param_pos, v))
                        },
                        Err(_) => {
                            self.context.graph[
                                self.index_info[chain_name][chain_pos]
                            ].node.send_msg(Message::SetToSymbol(param_pos, list[3].to_owned()));
                        }
                    };
                }
            }
        }
    }

    // for bela adc, in the utils.rs, the adc will become a pass node
    // the pass node connect to ~adc1 for example as reference
    // then all we need to do is to create these reference in the engine

    #[cfg(feature="bela")]
    pub fn make_adc_node(&mut self, chan:usize) {
        for i in 0..chan {
            // create a node
            let index = self.context.add_mono_node(Pass{});

            // create a default track from adc1 ~ adc$chan
            self.index_info.insert(format!("~adc{}", i), vec![index]);

            // self.adc_nodes.push(index);
            // let source = self.graph.add_node( 
            //     NodeData::new1( BoxedNodeSend::new( AdcSource {} ) )
            // );
            // self.adc_source_nodes.push(source);
            // self.graph.add_edge(source, index, ());
        }
    }

    #[cfg(feature="bela")]
    pub fn set_adc_node_buffer(&mut self, buf: &[f32], chan: usize,
        frame: usize, _interleave: bool) {
        
        for c in 0..chan {
            self.context.graph[
                self.index_info[
                    &format!("~adc{}",c)
                ][0]
            ].buffers[0].copy_from_slice(&buf[c*frame..(c+1)*frame]);
        }
    }

    #[cfg(feature="use-samples")]
    pub fn add_sample(&mut self, name:&str, sample: &'static [f32], channels: usize, sr: usize ) {
        self.samples_dict.insert(name.to_owned(), (sample, channels, sr));
    }

    pub fn update_with_code(&mut self, code: &str) {
        if code != &self.code {
            self.code = code.to_owned();
            self.need_update = true;
        }
    }

    pub fn update(&mut self) -> Result<(), EngineError>  {
        self.parse()?;
        self.make_graph()?;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.context.reset();
        self.ast.clear();
        self.new_ast.clear();
        self.code = "".to_owned();
        self.index_info.clear();
        self.index_info_backup.clear();
        self.temp_node_index.clear();
        self.node_add_list.clear();
        self.node_remove_list.clear();
        self.node_index_to_remove.clear();
        self.node_update_list.clear();
        self.refpairlist.clear();
        self.samples_dict.clear();
        self.bpm = 120.;
        self.track_amp = 1.0;
        self.seed = 42;
        self.clock = 0;
        self.livecoding = true;
        self.need_update = false;
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
        self.temp_node_index.clear();
        self.node_index_to_remove.clear();
        self.refpairlist.clear(); // we recalculate all the sidechains since some index can change

        // also remove the whole chain in_old but not_in_new, after ensuring there is no problem with new stuff
        // println!("\n\nold ast {:?}\n\n new {:?}", self.ast, self.new_ast);
        for (key, node_info_tuple) in &mut self.new_ast {
            if self.ast.contains_key(key) {
                let old_chain = &self.ast[key].0;
                let new_chain = &node_info_tuple.0;
                let old_chain_para = &self.ast[key].1;
                let new_chain_para = &mut node_info_tuple.1;
                for action in diff(old_chain, new_chain) {
                    match action {
                        DiffResult::Common(v) => {
                            // let common_node_name = v.data;
                            let old_i = v.old_index.unwrap();
                            let new_i = v.new_index.unwrap();
                            // println!("common {:?}", v);
                            // println!("common node: old_index {:?}", old_i);
                            // println!("common node: new_i {:?}", new_i);
                            // println!("common node old para {:?}", old_chain_para[old_i]);
                            // println!("common node new para {:?}", new_chain_para[new_i]);
                            if old_chain_para[old_i] != new_chain_para[new_i] {
                                self.node_update_list.push(
                                    ((*key).clone(), // which chain
                                    new_i, // where in chain
                                    new_chain_para[new_i].clone() // new paras
                                ))
                            } else {
                                // the paras can be the same
                                // but if the paras are refs, the source chain of refs can change
                                // e.g. the main chain is o: constsig 42 >> mul ~a
                                // the ref ~a: constsig 0.5 becomes ~a: constsig 0.5 >> mul 0.5
                                // need to reconnect them with the ref source
                                // note that when update, the reflist is cleared,
                                // so we will need to rebuild all the ref connection anyway
                                let mut reflist = vec![];
                                for para in &new_chain_para[new_i] {
                                    match para {
                                        GlicolPara::Reference(v) => {
                                            reflist.push(v.to_string());
                                        },
                                        GlicolPara::Sequence(seqs) => {
                                            for seq in seqs {
                                                match &seq.1 {
                                                    GlicolPara::Reference(v) => {
                                                        reflist.push(v.to_owned());
                                                    },
                                                    _ => {}
                                                }
                                            }
                                        },
                                        _ => {},
                                    }
                                }
                                if !reflist.is_empty() {
                                    self.refpairlist.push((reflist, (*key).clone(), new_i));
                                }
                            }
                        },
                        DiffResult::Removed(v) => {
                            let old_i = v.old_index.unwrap();
                            self.node_remove_list.push(((*key).clone(), old_i));
                            // println!("Removed {:?}", v)
                        },
                        DiffResult::Added(v) => {
                            // println!("Added {:?}", v);
                            let new_i = v.new_index.unwrap();
                            let insert_i = v.new_index.unwrap();
                            let nodename = v.data;
                            let mut paras = &mut new_chain_para[new_i];
                            let (nodedata, reflist) = makenode(
                                &nodename, 
                                &mut paras,
                                &self.samples_dict, 
                                self.sr,
                                self.bpm,
                                self.seed                    
                            )?;
                            if !reflist.is_empty() {
                                self.refpairlist.push((reflist, (*key).clone(), insert_i));
                            }
                           
                            self.node_add_list.push(((*key).clone(), insert_i, nodedata));                            
                        },
                    }
                }
                // println!("diff {:?}", diff(old_chain, new_chain));
            } else {
                for i in 0..node_info_tuple.0.len() {
                    let name = &node_info_tuple.0[i];
                    let mut paras = node_info_tuple.1[i].clone();
                    let (nodedata, reflist)  = makenode(
                        name, 
                        &mut paras, 
                        &self.samples_dict,
                        self.sr,
                        self.bpm,
                        self.seed
                    )?;
                    if !reflist.is_empty() {
                        self.refpairlist.push((reflist, (*key).clone(), i));
                    }         
                    // println!("self.node_add_list {:?} {}", key, i);
                    self.node_add_list.push(((*key).clone(), i, nodedata));
                };
            }
        }
        Ok(())
    }

    pub fn make_graph(&mut self) -> Result<(), EngineError> {

        self.handle_remove_chain();
        self.handle_node_remove();
        self.handle_node_add();
        match self.handle_node_update() {
            Ok(_) => {},
            Err(e) => self.clean_up(e)?
        };
        
        match self.handle_ref_check() {
            Ok(_) => {
                // println!(" ref check &self.node_index_to_remove {:?}", &self.node_index_to_remove);
                for id in &self.node_index_to_remove {
                    self.context.graph.remove_node(*id);
                }
            },
            Err(e) => self.clean_up(e)?
        };
        
        self.handle_connection();
        self.ast = self.new_ast.clone();
        self.index_info_backup = self.index_info.clone();
        Ok(())
    }

    pub fn clean_up(&mut self, e: EngineError) -> Result<(), EngineError> {
        // remove the added node
        // use the old index
        for id in &self.temp_node_index {
            // println!("graph.remove_node in clean_up {:?}", *id);
            self.context.graph.remove_node(*id);
        }
        self.index_info = self.index_info_backup.clone();
        return Err(e)
    }

    pub fn handle_ref_check(&self) -> Result<(), EngineError> {
        // ref pair is like (~mod -> a node [e.g key: out, pos_in_chain: 3])
        // ref check should use the new ast hashmap
        // because old ast hashmap has something that may need to be deleted
        // println!("ref check {:?}", self.refpairlist);

        for refpair in &self.refpairlist {
            for refname in &refpair.0 {
                // println!("ref check {} {}", self.new_ast.contains_key(refname), refname);
                if refname.contains("..") {
                    // println!("look for {}", &refname.replace("..", ""));
                    let mut count = 0;
                    for key in self.index_info.keys() {
                        if ((*key).clone()).starts_with(&refname.replace("..", "")) {
                            count += 1;
                        }
                    }
                    if count == 0 {
                        return Err(EngineError::NonExistReference(refname.to_owned()))
                    }
                } else {
                    if !self.new_ast.contains_key(refname) && !self.index_info.contains_key(refname) {
                        return Err(EngineError::NonExistReference(refname.to_owned()))
                    }
                }
                
                
            }
        }
        Ok(())
    }

    pub fn handle_remove_chain(&mut self) {
        // there are some chains show up in old_ast but not in new ast
        for key in self.ast.keys() {
            if !self.new_ast.contains_key(key) {
                // println!("remove {:?}", key);
                for index in &self.index_info[key] {
                    // self.context.graph.remove_node(*index);
                    self.node_index_to_remove.push(*index)
                }
                self.index_info.remove_entry(key);       
            }
        }
    }

    pub fn handle_node_add(&mut self) {
        while !self.node_add_list.is_empty() {
            let (key, position_in_chain, nodedata) = self.node_add_list.remove(0); // for insertion, this is better
            if !self.index_info.contains_key(&key) {
                self.index_info.insert(key.clone(), vec![]);
            };
            let nodeindex = self.context.graph.add_node(nodedata); // TODO: save these id, if there is an error, remove these node
            self.temp_node_index.push(nodeindex);
            if let Some(chain) = self.index_info.get_mut(&key) { // TODO: backup the index_info
                chain.insert(position_in_chain, nodeindex);
            }
        }
        // println!("node index map after handle add{:?}", self.index_info);
    }
    pub fn handle_node_update(&mut self) -> Result<(), EngineError> {
        while !self.node_update_list.is_empty() {
            let (key, position_in_chain, paras) = self.node_update_list.pop().unwrap(); // ok as is it not empty
            // println!("handle update {:?} {:?}", key, position_in_chain);
            if let Some(chain) = self.index_info.get_mut(&key) {

                // TODO: reset order here, if ref is wrong, cannot be reverted
                // self.context.graph[
                //     chain[position_in_chain]].node.send_msg(Message::ResetOrder);
                for (i, para) in paras.iter().enumerate() {
                    match para {
                        GlicolPara::Number(v) => self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToNumber(i as u8, *v)),
                        GlicolPara::Reference(s) => {
                            self.refpairlist.push((vec![s.to_string()], key.clone(), position_in_chain));
                        },
                        GlicolPara::SampleSymbol(s) => {
                            if !self.samples_dict.contains_key(&*s as &str) {
                                return Err(EngineError::NonExsitSample((s.to_string()).to_owned()))
                            }
                            self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToSamples(i as u8, self.samples_dict[s]))
                        },
                        GlicolPara::Points(_p) => self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetParam(i as u8, para.clone())),
                        GlicolPara::Bool(b) => self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToBool(i as u8, *b)),
                        GlicolPara::Symbol(s) => {
                            self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToSymbol(i as u8, s.to_string()))
                        },
                        GlicolPara::Sequence(events) => {
                            // println!("found seq in update, process it {:?}", events);
                            // todo: an issue is that when you revert it, these messages cannot be undone
                            self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToSeq(i as u8, events.clone())
                            );
                            let mut reflist = vec![];
                            let mut count = 0;
                            let mut order = hashbrown::HashMap::new();
                            for event in events {
                                match &event.1 {
                                    GlicolPara::Reference(s) => { // reflist: ["~a", "~b", "~a"]
                                        if !reflist.contains(s) {
                                            reflist.push(s.clone());
                                            order.insert(s.clone(), count);
                                            count += 1;
                                        }
                                    },
                                    _ => {}
                                }
                            };
                            self.refpairlist.push((reflist, key.clone(), position_in_chain));
                            self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetRefOrder(order)
                            );
                        },
                        GlicolPara::NumberList(l) => {
                            self.context.graph[
                            chain[position_in_chain]].node.send_msg(
                                Message::SetToNumberList(i as u8, l.clone()))
                        },
                        GlicolPara::Pattern(value_time_list, span) => {
                            // todo, differ a symbol pattern and number pattern?
                            let mut samples_dict_selected = HashMap::new();
                            let mut symbol_pattern = vec![];
                            let mut number_pattern = vec![];

                            for value_time in value_time_list.iter() {
                                let time = value_time.1;
                                match &value_time.0 {
                                    GlicolPara::Number(num) => {
                                        number_pattern.push((*num, time))
                                    },
                                    GlicolPara::Symbol(s) => {
                                        if self.samples_dict.contains_key(s) {
                                            samples_dict_selected.insert(s.to_owned(), self.samples_dict[s]);
                                        } else {
                                            return Err(EngineError::NonExsitSample(s.to_owned()))
                                        }
                                        symbol_pattern.push((s.to_string(), time));
                                    }
                                    _ => unimplemented!()
                                };
                                // pattern.push((value, time));
                            }

                            if symbol_pattern.len() != 0 {
                                self.context.graph[
                                    chain[position_in_chain]
                                ].node.send_msg(
                                    Message::SetSamplePattern(symbol_pattern, *span, samples_dict_selected)
                                )
                            } else {
                                self.context.graph[
                                    chain[position_in_chain]
                                ].node.send_msg(
                                    Message::SetPattern(number_pattern, *span)
                                )
                            }
                        },

                        _ => {}
                    }
                }

                // self.context.send_msg(index: NodeIndex, msg: Message)
            }
        }
        Ok(())
    }
    pub fn handle_node_remove(&mut self) {
        while !self.node_remove_list.is_empty() {

            // need to pop from the back so the pos is right
            let (key, position_in_chain) = self.node_remove_list.pop().unwrap();

            // println!("self.index_info {:?}", self.index_info);
            if let Some(chain) = self.index_info.get_mut(&key) { // touch the index is fine, as we have a backup
                // println!("chain {:?} position_in_chain {:?}", chain, position_in_chain);
                let node_index = chain[position_in_chain];
                // self.context.graph.remove_node(node_index);
                self.node_index_to_remove.push(node_index);
                chain.remove(position_in_chain);
            }
        }
    }

    pub fn handle_connection(&mut self) {
        self.context.graph.clear_edges();
        // println!("self.index_info in handle_connection{:?}", self.index_info);
        // println!("self.refpairlist in handle_connection {:?}", self.refpairlist);

        // 
        let mut already_reset = vec![];
        for refpairs in &self.refpairlist {
            let index = self.index_info[&refpairs.1][refpairs.2];
            if !already_reset.contains(&index) {
                self.context.graph[index].node.send_msg(Message::ResetOrder);
                already_reset.push(index.clone());
            }
            for refname in &refpairs.0 {
                if refname.contains("..") {
                    // println!("look for {}", &refname.replace("..", ""));
                    for (key, value) in self.index_info.iter() {
                        if ((*key).clone()).starts_with(&refname.replace("..", "")) {
                            self.context.connect(*value.last().unwrap(), index);
                        }
                    }
                } else {
                    self.context.connect(*self.index_info[refname].last().unwrap(), index);
                }
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

    pub fn next_block(&mut self, buf: Vec<&[f32]>) -> (&[Buffer<N>], [u8; 256]) {  //  -> &Vec<Buffer<N>> 
        if buf.len() > 0 {
            self.context.graph[
                self.index_info[
                    &format!("~input")
                ][0]
            ].buffers[0].copy_from_slice(buf[0]);
        }

        if buf.len() > 1 {
            self.context.graph[
                self.index_info[
                    &format!("~input")
                ][0]
            ].buffers[1].copy_from_slice(buf[1]);
        }
        // if self.livecoding {
        let mut result = [0; 256];
        let one_bar = (240.0 / self.bpm * self.sr as f32) as usize;
        let time_to_update = (self.clock + N) % one_bar <= N;
        if self.need_update && (!self.livecoding || time_to_update) {
            self.need_update = false;
            match self.update() {
                Ok(_) => {
                    for i in 0..256 {
                        result[i] = 0
                    }
                },
                Err(e) => {
                    
                    result[0] = match e {
                        EngineError::ParsingError(_) => 1,
                        EngineError::NonExsitSample(_) => 2,
                        EngineError::NonExistReference(_) => 3,
                    };
                    let error = match e {
                        EngineError::ParsingError(v) => {
                            // println!("catch error in parser; in location: {:?}; line_col: {:?}", v.location, v.line_col);
                            // pest::error::LineColLocation::Pos
                            let location = match v.location {
                                pest::error::InputLocation::Pos(u) => u,
                                _ => unimplemented!()
                            };
                            let (line, col) =  match v.line_col {
                                pest::error::LineColLocation::Pos(u) => u,
                                _ => unimplemented!()
                            };
                            let (positives, negatives) = match &v.variant {
                                pest::error::ErrorVariant::ParsingError{ positives, negatives } => {
                                    (positives, negatives)
                                    // if positives.len() != 0 {
                                    //     print!("\n\nexpecting ");
                                    //     for possible in positives { print!("{:?} ", possible) }
                                    //     print!("\n\n");
                                    // }
                                    // if negatives.len() != 0 {
                                    //     print!("\n\nunexpected element: ");
                                    //     for possible in negatives { print!("{:?} ", possible) }
                                    //     print!("\n\n");
                                    // }
                                },
                                _ => {panic!("unknonw parsing error")}
                            };
                            // let linecode = v.line;
                            // println!("{:?}", v);
                            let res = format!("pos[{:?}], line[{:?}], col[{:?}], positives{:?}, negatives{:?}",
                             location, line, col, positives, negatives);
                            // println!("{}", res);
                            res
                            // match v.variant {
                            //     pest::error::ErrorVariant::ParsingError { positives, negatives} => {
                            //         println!("print expecting {:?} find {:?}", positives, negatives);
                            //         // format!("format expecting {:?} find {:?}", positives, negatives);
                            //         format!("format")
                            //         // return (positives, negatives)              
                            //     },
                            //     _ => {
                            //         unimplemented!();
                            //     }
                            // }
                            
                        },
                        EngineError::NonExsitSample(v) => {
                            format!("cannot use this non-exist samples {}", v)
                        }
                        EngineError::NonExistReference(v) => {
                            format!("cannot use this non-exist reference {}", v)
                        }
                    };
                    let s = error.as_bytes();
                    for i in 2..256 {
                        if i - 2 < s.len() {
                            result[i] = s[i-2]
                        } else {
                            result[i] = 0
                        }
                        
                    }
                }
            }
        }            
        // }
        self.context.processor.process(&mut self.context.graph, self.context.destination);
        // println!("result {:?}", &self.context.graph[self.context.destination].buffers);
        self.clock += N;
        (&self.context.graph[self.context.destination].buffers, result)
    }

    pub fn set_bpm(&mut self, bpm:f32) {
        self.bpm = bpm;
        self.context.send_msg_to_all(Message::SetBPM(bpm));
    }
    pub fn set_sr(&mut self, sr:usize) {self.sr = sr}
    pub fn set_seed(&mut self, seed:usize) {self.seed = seed}
    pub fn set_track_amp(&mut self, amp:f32) {self.track_amp = amp}
}