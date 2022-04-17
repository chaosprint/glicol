use glicol::Engine; 
// use glicol::{EngineError, get_error_info};

// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<8>::new();
    engine.update_with_code(r#"o: pattern_synth `` 1"#);
    println!("next block {:?}", engine.next_block(vec![]));
}