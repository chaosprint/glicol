use glicol::Engine;
use glicol::GlicolNodeInfo;
use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<128>::new();
    let mut ast = HashMap::new();
    ast.insert("out", vec![ GlicolNodeInfo::ConstSig("42.") ] );
    engine.ast_to_graph(ast);
    // engine.make_graph();
    engine.next_block();
    engine.send_msg("out", 0, (0, "58."));
    engine.next_block();
}