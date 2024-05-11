// todo: When error, the error info still updates some nodes..

pub mod util;
use std::collections::VecDeque;

use util::makenode;
pub mod error;
pub use error::{get_error_info, EngineError};
use glicol_parser::{get_ast, nodes::{Ast, Component, NumberOrRef}, ToInnerOwned as _};
use glicol_synth::{
    AudioContext, AudioContextConfig, BoxedNodeSend, Buffer, GlicolGraph, GlicolPara, Message, NodeData, Pass
};
use hashbrown::HashMap;
use petgraph::graph::NodeIndex;
use yoke::Yoke;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
type YokedAst = Yoke<Ast<'static>, Box<str>>;

#[derive(Default)]
struct GraphDiff<'engine, const N: usize> {
    node_remove_list: Vec<(&'engine str, usize)>,
    node_update_list: Vec<(&'engine str, usize, Vec<GlicolPara<&'engine str>>)>,
    refpairlist: Vec<(Vec<String>, &'engine str, usize)>,
    node_add_list: VecDeque<(&'engine str, usize, GlicolNodeData<N>)>,
    idx_to_remove: Vec<NodeIndex>
}

pub struct Engine<const N: usize> {
    pub context: AudioContext<N>,
    ast: Option<YokedAst>,
    pub index_info: HashMap<String, Vec<NodeIndex>>,
    pub index_info_backup: HashMap<String, Vec<NodeIndex>>,
    temp_node_index: Vec<NodeIndex>, // created in the adding process, will be deleted if err
    pub samples_dict: HashMap<String, (&'static [f32], usize, usize)>,
    bpm: f32,
    sr: usize,
    track_amp: f32,
    seed: usize,
    clock: usize,
    pub livecoding: bool,
    need_update: bool,
}

impl<const N: usize> Default for Engine<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Engine<N> {
    pub fn new() -> Self {
        let mut context = AudioContext::<N>::new(AudioContextConfig::default());
        let mut index_info = HashMap::new();
        index_info.insert("~input".to_string(), vec![context.add_stereo_node(Pass {})]);
        Self {
            context,
            ast: None,
            index_info: index_info.clone(),
            index_info_backup: index_info.clone(),
            temp_node_index: vec![],
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
        for command in commands.split(';').filter(|c| !c.is_empty()) {
            let mut list = command.split(',');
            let (
                Some(chain_name),
                Some(chain_pos),
                Some(param_pos),
                Some(value)
            ) = (list.next(), list.next(), list.next(), list.next()) else {
                continue; // todo: this should be an error
            };

            let chain_pos = chain_pos.parse::<usize>().unwrap_or_default();
            let param_pos = param_pos.parse::<u8>().unwrap_or_default();
            if self.index_info.contains_key(chain_name) {
                match value.parse::<f32>() {
                    // todo: check the name and pos
                    Ok(v) => self.context.graph[self.index_info[chain_name][chain_pos]]
                        .node
                        .send_msg(Message::SetToNumber(param_pos, v)),
                    Err(_) => {
                        self.context.graph[self.index_info[chain_name][chain_pos]]
                            .node
                            .send_msg(Message::SetToSymbol(param_pos, value.to_string()));
                    }
                };
            }
        }
    }

    // for bela adc, in the utils.rs, the adc will become a pass node
    // the pass node connect to ~adc1 for example as reference
    // then all we need to do is to create these reference in the engine

    #[cfg(feature = "bela")]
    pub fn make_adc_node(&mut self, chan: usize) {
        for i in 0..chan {
            // create a node
            let index = self.context.add_mono_node(Pass {});

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

    #[cfg(feature = "bela")]
    pub fn set_adc_node_buffer(
        &mut self,
        buf: &[f32],
        chan: usize,
        frame: usize,
        _interleave: bool,
    ) {
        for c in 0..chan {
            self.context.graph[self.index_info[&format!("~adc{}", c)][0]].buffers[0]
                .copy_from_slice(&buf[c * frame..(c + 1) * frame]);
        }
    }

    #[cfg(feature = "use-samples")]
    pub fn add_sample(&mut self, name: &str, sample: &'static [f32], channels: usize, sr: usize) {
        self.samples_dict
            .insert(name.to_owned(), (sample, channels, sr));
    }

    pub fn reset(&mut self) {
        self.context.reset();
        self.ast = None;
        self.index_info.clear();
        self.index_info_backup.clear();
        self.temp_node_index.clear();
        self.samples_dict.clear();
        self.bpm = 120.;
        self.track_amp = 1.0;
        self.seed = 42;
        self.clock = 0;
        self.livecoding = true;
        self.need_update = false;
    }

    // This is kind of a mess. But the basic idea is that we want to:
    // 1. Parse this code to create an AST
    // 2. Diff that AST against the current AST, create a `GraphDiff` that represents the
    //       differences. Because that `GraphDiff` will be borrowing from `self`, we need to keep
    //       it all in this function instead of trying to make it cross fn boundaries to do this in
    //       multiple different fns because then we'll get some weird borrow errors.
    // 3. Use that `GraphDiff` to apply the changes that need to be made to `self.context.graph` to
    //       update it fully to reflect the new syntax
    // 4. Set self.ast = new_ast;
    pub fn update_with_code(&mut self, code: &str) -> Result<(), EngineError> {
        // It would be nice to re-use the allocation in self.ast for this Yoke creation, but I
        // can't figure out a way to.
        // We have to make this Yoke here to return it (we can't like make an `Ast` with the same
        // lifetime as `code` 'cause then it won't have the right lifetime to exist within the
        // self.ast `Yoke`, and we can't `self.ast.take()` and then re-use the allocation there
        // 'cause then we'll be unable to do this diffing thing against what it used to be b/c
        // it'll have been overwritten.
        let new_ast: YokedAst = Yoke::try_attach_to_cart(code.to_owned().into_boxed_str(), |code| get_ast(code))?;

        self.temp_node_index.clear();

        let mut graph_diff = GraphDiff::default();

        // also remove the whole chain in_old but not_in_new, after ensuring there is no problem with new stuff
        // println!("\n\nold ast {:?}\n\n new {:?}", self.ast, self.new_ast);

        fn add_nodes<'iter, 'ast: 'iter, const N: usize>(
            chain_name: &'ast str,
            iter: impl Iterator<Item = (usize, &'iter Component<'ast>)>,
            graph_diff: &mut GraphDiff<'ast, N>,
            samples_dict: &HashMap<String, (&'static [f32], usize, usize)>,
            sr: usize,
            bpm: f32,
            seed: usize
        ) -> Result<(), EngineError> {
            for (i, component) in iter {
                let (nodedata, reflist) = makenode(component, samples_dict, sr, bpm, seed)?;

                if !reflist.is_empty() {
                    graph_diff.refpairlist.push((reflist, chain_name, i));
                }

                graph_diff.node_add_list.push_back((chain_name, i, nodedata));
            }
            Ok(())
        }

        if let Some(ref old_ast) = self.ast {
            for (chain_name, new_chain) in &new_ast.get().nodes {
                let Some(old_chain) = old_ast.get().nodes.get(chain_name) else {
                    // if we can't find this chain in the old_ast, we just go through it and add
                    // every single one to the chain
                    add_nodes(
                        chain_name,
                        new_chain.iter().enumerate(),
                        &mut graph_diff,
                        &self.samples_dict,
                        self.sr,
                        self.bpm,
                        self.seed
                    )?;
                    continue
                };

                // we gotta go through every node which exists in the old chain and check if it
                // exists in the new one.
                for (old_idx, old_comp) in old_chain.iter().enumerate() {
                    match new_chain.iter().enumerate().find(|(_, comp)| old_comp == *comp) {
                        // If it exists in the new chain, then we have to update it
                        Some((idx, new_comp)) => {
                            // the paras can be the same
                            // but if the paras are refs, the source chain of refs can change
                            // e.g. the main chain is o: constsig 42 >> mul ~a
                            // the ref ~a: constsig 0.5 becomes ~a: constsig 0.5 >> mul 0.5
                            // need to reconnect them with the ref source
                            // note that when update, the reflist is cleared,
                            // so we will need to rebuild all the ref connection anyway
                            let reflist = new_comp.all_references();
                            if !reflist.is_empty() {
                                let owned_reflist = reflist.into_iter()
                                    .map(|s| s.to_owned())
                                    .collect();
                                graph_diff.refpairlist.push((owned_reflist, chain_name, idx));
                            }
                        },
                        // If it doesn't exist in the new chain,
                        None => graph_diff.node_remove_list.push((chain_name, old_idx)),
                    }
                }

                // then look through the new chain and find everything which doesn't exist in the
                // old one, and was thus inserted, and track it for insertion
                add_nodes(
                    chain_name,
                    new_chain.iter().enumerate()
                        .filter(|(_, comp)| !old_chain.iter().any(|old_comp| old_comp == *comp)),
                    &mut graph_diff,
                    &self.samples_dict,
                    self.sr,
                    self.bpm,
                    self.seed
                )?;
            }

            // there are some chains show up in old_ast but not in new ast
            // so we need to figure out what they are and collect them into a Vec
            graph_diff.idx_to_remove.extend(old_ast.get().nodes.keys()
                .filter(|key| !new_ast.get().nodes.contains_key(*key))
                .flat_map(|key| {
                    // This should be safe to unwrap because index_info should always be consistent
                    // with self.ast, but we're .expect'ing just to provide a good message in case
                    self.index_info.remove_entry(*key)
                        .expect("Index info should be consistent with self.ast, but it turned out to not be. This is a bug.")
                        .1
                }));

            // Then remove them all from self
            while let Some((key, position_in_chain)) = graph_diff.node_remove_list.pop() {
                if let Some(chain) = self.index_info.get_mut(key) {
                    // touch the index is fine, as we have a backup
                    let node_index = chain[position_in_chain];

                    graph_diff.idx_to_remove.push(node_index);
                    chain.remove(position_in_chain);
                }
            }
        } else {
            // if the old ast doesn't exist, just add everything to the new graph
            for (chain_name, new_chain) in &new_ast.get().nodes {
                add_nodes(
                    chain_name,
                    new_chain.iter().enumerate(),
                    &mut graph_diff,
                    &self.samples_dict,
                    self.sr,
                    self.bpm,
                    self.seed
                )?;
            }
        };

        // go through all the nodes that are
        while let Some((key, position_in_chain, nodedata)) = graph_diff.node_add_list.pop_front() {
            // TODO: save these id, if there is an error, remove these node
            let nodeindex = self.context.graph.add_node(nodedata);

            self.temp_node_index.push(nodeindex);
            match self.index_info.get_mut(key) {
                Some(chain) => chain.insert(position_in_chain, nodeindex),
                None => _ = self.index_info.insert(key.to_string(), vec![nodeindex]),
            }
        }

        fn handle_node_update<const N: usize>(
            graph_diff: &mut GraphDiff<'_, N>,
            index_info: &HashMap<String, Vec<NodeIndex>>,
            graph: &mut GlicolGraph<N>,
            samples_dict: &mut HashMap<String, (&'static [f32], usize, usize)>,
        ) -> Result<(), EngineError> {
            while let Some((key, position_in_chain, paras)) = graph_diff.node_update_list.pop() {

                // println!("handle update {:?} {:?}", key, position_in_chain);
                if let Some(chain) = index_info.get(key) {
                    // TODO: reset order here, if ref is wrong, cannot be reverted
                    // self.context.graph[
                    //     chain[position_in_chain]].node.send_msg(Message::ResetOrder);
                    for (i, para) in paras.iter().enumerate() {
                        match para {
                            GlicolPara::Number(v) => graph[chain[position_in_chain]]
                                .node
                                .send_msg(Message::SetToNumber(i as u8, *v)),
                            GlicolPara::Reference(s) => {
                                graph_diff.refpairlist.push((
                                    vec![s.to_string()],
                                    key,
                                    position_in_chain,
                                ));
                            }
                            GlicolPara::SampleSymbol(s) => {
                                let Some(sample) = samples_dict.get(*s) else {
                                    return Err(EngineError::NonExistSample(s.to_string()));
                                };

                                graph[chain[position_in_chain]]
                                    .node
                                    .send_msg(Message::SetToSamples(i as u8, *sample))
                            }
                            GlicolPara::Points(_p) => graph[chain[position_in_chain]]
                                .node
                                .send_msg(Message::SetParam(i as u8, (*para).to_inner_owned())),
                            GlicolPara::Bool(b) => graph[chain[position_in_chain]]
                                .node
                                .send_msg(Message::SetToBool(i as u8, *b)),
                            GlicolPara::Symbol(s) => graph[chain[position_in_chain]]
                                .node
                                .send_msg(Message::SetToSymbol(i as u8, s.to_string())),
                            GlicolPara::Sequence(events) => {
                                // println!("found seq in update, process it {:?}", events);
                                // todo: an issue is that when you revert it, these messages cannot be undone
                                graph[chain[position_in_chain]]
                                    .node
                                    .send_msg(Message::SetToSeq(i as u8, events.to_inner_owned()));
                                let mut reflist: Vec<String> = vec![];
                                let mut count = 0;
                                let mut order = hashbrown::HashMap::new();
                                for event in events {
                                    if let NumberOrRef::Ref(s) = &event.1 {
                                        // reflist: ["~a", "~b", "~a"]
                                        if !reflist.iter().any(|r| r == s) {
                                            reflist.push(s.to_string());
                                            order.insert(s.to_string(), count);
                                            count += 1;
                                        }
                                    }
                                }
                                graph_diff.refpairlist
                                    .push((reflist, key, position_in_chain));
                                graph[chain[position_in_chain]]
                                    .node
                                    .send_msg(Message::SetRefOrder(order));
                            }
                            GlicolPara::NumberList(l) => graph[chain[position_in_chain]]
                                .node
                                .send_msg(Message::SetToNumberList(i as u8, l.clone())),
                            GlicolPara::Pattern(value_time_list, span) => {
                                // todo, differ a symbol pattern and number pattern?
                                let mut samples_dict_selected = HashMap::new();
                                let mut symbol_pattern = vec![];
                                let mut number_pattern = vec![];

                                for value_time in value_time_list.iter() {
                                    let time = value_time.1;
                                    match &value_time.0 {
                                        GlicolPara::Number(num) => number_pattern.push((*num, time)),
                                        GlicolPara::Symbol(s) => {
                                            let Some(sample) = samples_dict.get(*s) else {
                                                return Err(EngineError::NonExistSample(s.to_string()));
                                            };

                                            samples_dict_selected
                                                .insert(s.to_string(), *sample);
                                            symbol_pattern.push((s.to_string(), time));
                                        }
                                        _ => unimplemented!(),
                                    };
                                    // pattern.push((value, time));
                                }

                                if !symbol_pattern.is_empty() {
                                    graph[chain[position_in_chain]].node.send_msg(
                                        Message::SetSamplePattern(
                                            symbol_pattern,
                                            *span,
                                            samples_dict_selected,
                                        ),
                                    )
                                } else {
                                    graph[chain[position_in_chain]]
                                        .node
                                        .send_msg(Message::SetPattern(number_pattern, *span))
                                }
                            }

                            _ => {}
                        }
                    }

                    // self.context.send_msg(index: NodeIndex, msg: Message)
                }
            }
            Ok(())
        }

        if let Err(e) = handle_node_update(
            &mut graph_diff,
            &self.index_info,
            &mut self.context.graph,
            &mut self.samples_dict
        ) {
            return Err(self.clean_up(e));
        };

        // verify that all refs that are mentioned in the refpairlist do, in fact, exist in the
        // new_ast or current index_info and will thus work when we try to connect the graph
        fn handle_ref_check(
            refpairlist: &[(Vec<String>, &str, usize)],
            index_info: &HashMap<String, Vec<NodeIndex>>,
            new_ast: &YokedAst
        ) -> Result<(), EngineError> {
            // ref pair is like (~mod -> a node [e.g key: out, pos_in_chain: 3])
            // ref check should use the new ast hashmap
            // because old ast hashmap has something that may need to be deleted
            // println!("ref check {:?}", self.refpairlist);

            for (names, _, _) in refpairlist {
                for refname in names {
                    // println!("ref check {} {}", self.new_ast.contains_key(refname), refname);
                    if refname.contains("..") {
                        // println!("look for {}", &refname.replace("..", ""));
                        if !index_info.keys().any(|key| key.starts_with(&refname.replace("..", ""))) {
                            return Err(EngineError::NonExistReference(refname.to_owned()));
                        }
                    } else if !new_ast.get().nodes.contains_key(&**refname) && !index_info.contains_key(refname) {
                        return Err(EngineError::NonExistReference(refname.to_owned()));
                    }
                }
            }
            Ok(())
        }

        match handle_ref_check(&graph_diff.refpairlist, &self.index_info, &new_ast) {
            Ok(_) => {
                // println!(" ref check &self.node_index_to_remove {:?}", &self.node_index_to_remove);
                for id in &graph_diff.idx_to_remove {
                    self.context.graph.remove_node(*id);
                }
            }
            Err(e) => return Err(self.clean_up(e)),
        };

        self.context.graph.clear_edges();
        // println!("self.index_info in handle_connection{:?}", self.index_info);
        // println!("self.refpairlist in handle_connection {:?}", self.refpairlist);

        // now we have to go through and actually make the graph with all the connections we have
        let mut already_reset = std::collections::HashSet::new();
        for (reflist, name, new_idx) in &graph_diff.refpairlist {
            let Some(chain) = self.index_info.get(*name) else {
                return Err(EngineError::NonExistReference(name.to_string()));
            };

            let index = chain[*new_idx];

            if !already_reset.contains(&index) {
                self.context.graph[index].node.send_msg(Message::ResetOrder);
                already_reset.insert(index);
            }

            for refname in reflist {
                if refname.contains("..") {
                    // println!("look for {}", &refname.replace("..", ""));
                    for (key, value) in self.index_info.iter() {
                        if key.starts_with(&refname.replace("..", "")) {
                            self.context.connect(*value.last().unwrap(), index);
                        }
                    }
                } else {
                    self.context
                        .connect(*self.index_info[refname].last().unwrap(), index);
                }
            }
        }

        for (key, chain) in &self.index_info {
            for window in chain.windows(2) {
                // this is guaranteed to succeed as long as the argument to windows is 2
                // TODO when array_windows is stabilized, change over to that
                if let [start, end] = window {
                    self.context.connect_with_order(*start, *end, 0);
                }
            }
            if !key.contains('~') {
                if let Some(end) = chain.last() {
                    self.context.connect_with_order(*end, self.context.destination, 0);
                }
            }
        }

        // We can't reuse the allocation here as far as I can tell; see the comment at the top of
        // Self::parse
        self.ast = Some(new_ast);
        self.index_info_backup.clone_from(&self.index_info);
        Ok(())
    }

    pub fn clean_up(&mut self, e: EngineError) -> EngineError {
        // remove the added node
        // use the old index
        for id in &self.temp_node_index {
            // println!("graph.remove_node in clean_up {:?}", *id);
            self.context.graph.remove_node(*id);
        }
        self.index_info.clone_from(&self.index_info_backup);
        e
    }

    pub fn next_block(&mut self, buf: Vec<&[f32]>) -> &[Buffer<N>] {
        //  -> &Vec<Buffer<N>>
        if !buf.is_empty() {
            self.context.graph[self.index_info[&"~input".to_string()][0]].buffers[0]
                .copy_from_slice(buf[0]);
        }

        if buf.len() > 1 {
            self.context.graph[self.index_info[&"~input".to_string()][0]].buffers[1]
                .copy_from_slice(buf[1]);
        }

        self.context
            .processor
            .process(&mut self.context.graph, self.context.destination);
        // println!("result {:?}", &self.context.graph[self.context.destination].buffers);
        self.clock += N;
        &self.context.graph[self.context.destination].buffers
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
        self.context.send_msg_to_all(Message::SetBPM(bpm));
    }
    pub fn set_sr(&mut self, sr: usize) {
        self.sr = sr
    }
    pub fn set_seed(&mut self, seed: usize) {
        self.seed = seed
    }
    pub fn set_track_amp(&mut self, amp: f32) {
        self.track_amp = amp
    }

    #[cfg(test)]
    fn get_ast(&self) -> Option<&Ast<'_>> {
        self.ast.as_ref().map(|y| y.get())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use glicol_parser::nodes::*;

    fn ast_from_nodes<const N: usize>(
        nodes: [(&'static str, Vec<Component<'static>>); N]
    ) -> Ast<'static> {
        Ast { nodes: hashbrown::HashMap::from_iter(nodes) }
    }

    #[test]
    fn adding_nodes() {
        let mut eng = Engine::<128>::new();
        eng.update_with_code("o: saw 440 >> mul 0.3").unwrap();

        assert_eq!(
            eng.get_ast().unwrap(),
            &ast_from_nodes([
                ("o", vec![
                    Component::Saw(Saw {
                        param: NumberOrRef::Number(440.)
                    }),
                    Component::Mul(Mul {
                        param: NumberOrRef::Number(0.3)
                    })
                ])
            ])
        );

        eng.update_with_code("
            o: saw 440 >> mul 0.3
            i: sin 880 >> pan 0.5
        ").unwrap();

        assert_eq!(
            eng.get_ast().unwrap(),
            &ast_from_nodes([
                ("o", vec![
                    Component::Saw(Saw {
                        param: NumberOrRef::Number(440.)
                    }),
                    Component::Mul(Mul {
                        param: NumberOrRef::Number(0.3)
                    })
                ]),
                ("i", vec![
                    Component::Sin(Sin {
                        param: NumberOrRef::Number(880.)
                    }),
                    Component::Pan(Pan {
                        param: NumberOrRef::Number(0.5)
                    })
                ])
            ])
        );

        eng.update_with_code("
            o: saw 440 >> mul i
            i: sin 880 >> pan 0.5
        ").unwrap();

        assert_eq!(
            eng.get_ast().unwrap(),
            &ast_from_nodes([
                ("o", vec![
                    Component::Saw(Saw {
                        param: NumberOrRef::Number(440.)
                    }),
                    Component::Mul(Mul {
                        param: NumberOrRef::Ref("i")
                    })
                ]),
                ("i", vec![
                    Component::Sin(Sin {
                        param: NumberOrRef::Number(880.)
                    }),
                    Component::Pan(Pan {
                        param: NumberOrRef::Number(0.5)
                    })
                ])
            ])
        );
    }

    #[test]
    fn removing_nodes() {
        let mut eng = Engine::<128>::new();

        assert_eq!(
            eng.update_with_code("
                o: saw 440 >> mul i
                i: sin 880 >> pan 0.5
            "),
            Ok(())
        );

        assert_eq!(
            eng.update_with_code("
                o: saw 440 >> mul 0.3
                i: sin 880 >> pan 0.5
            "),
            Ok(())
        );

        assert_eq!(eng.update_with_code("o: saw 440 >> mul 0.3"), Ok(()));
    }
}
