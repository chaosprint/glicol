use glicol::Engine;
// use glicol::GlicolNodeInfo;
use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<128>::new();
    engine.set_code("o: const_sig 42.0 >> mul 0.1");
    // let mut ast = HashMap::new();
    // ast.insert("out", vec![ GlicolNodeInfo::ConstSig("42.") ] );
    // engine.ast_to_graph(ast);
    // engine.make_graph();
    engine.next_block();
    engine.set_code("o: const_sig 58.0 >> sin 440.0");
    // engine.send_msg("o", 0, (0, "440."));
    engine.next_block();
}