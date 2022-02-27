use glicol::Engine;
// use glicol::GlicolNodeInfo;
use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<128>::new();
    engine.set_code("o: constsig 42.0");
    engine.update();
    engine.next_block();
    engine.send_msg("o", 0, (0, "440."));
    engine.next_block();
}