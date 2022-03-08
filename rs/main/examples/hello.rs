use glicol::Engine;
// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<128>::new();
    engine.update("o: constsig 42 >>mul ~am; ~am: constsig 0.3");
    println!("refpairlist {:?}", engine.refpairlist);
    // engine.update("o: saw 500 >> lpf 100.0 1.0;

    // ~mod: sin 0.2 >> mul 200.0 >> add 500.0");
    // engine.add_sample("\\bb", &[1.0], 1);
    // engine.update(r#"o: imp 1 >> sp \808_0 >> delayms ~mod

    // ~mod: sin 0.2 >> mul 100 >> add 200"#);
    // for e in engine.context.graph.edges(engine.context.destination) {
    //     println!("destinations {:?}", e);
    // }
    engine.next_block();
    // engine.send_msg("o", 0, (0, "1."));
    // engine.next_block();
}