use glicol::Engine;
// use glicol::GlicolNodeInfo;
use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<128>::new();
    engine.update("o: sin 440.0");
    engine.next_block();
    engine.send_msg("o", 0, (0, "1."));
    engine.next_block();
}