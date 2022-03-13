use glicol::Engine; use glicol::{EngineError, get_error_info};

// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<8>::new();
    engine.update(r#"o: seq _ 60 >> sn 0.1"#).unwrap();
    // match engine.update("o: imp 100 >> mul ~mod
    // ~mo: sin 1 >> mul 0.5 >> add 0.5") {
    //     Ok(_) => {},
    //     Err(e) => {
    //         println!("{:?}", e);
    //         // match e {
    //         //     EngineError::ParsingError(e) => {
    //         //         println!("{:?}", get_error_info(e))
    //         //     },
    //         //     _ => unimplemented!()
    //         // }
    //     }
    // };
    // println!("refpairlist {:?}", engine.refpairlist);
    // engine.update("o: saw 500 >> lpf 100.0 1.0;

    // ~mod: sin 0.2 >> mul 200.0 >> add 500.0");
    // engine.add_sample("\\bb", &[1.0], 1);
    // engine.update(r#"o: imp 1 >> sp \808_0 >> delayms ~mod

    // ~mod: sin 0.2 >> mul 100 >> add 200"#);
    // for e in engine.context.graph.edges(engine.context.destination) {
    //     println!("destinations {:?}", e);
    // }
    println!("next block {:?}", engine.next_block());
    // engine.send_msg("o", 0, (0, "1."));
    // engine.next_block();
}