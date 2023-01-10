use glicol::Engine; 
// use glicol::{EngineError, get_error_info};

// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<32>::new();
    engine.update_with_code(r#"o: expr `4.2`"#); // y=math::sin(2*PI*x);x+=440.0/sr;y
    println!("next block {:?}", engine.next_block(vec![]));
}