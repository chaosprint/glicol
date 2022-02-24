use glicol::Engine;

fn main() {
    let mut engine = Engine::<128>::new();
    engine.set_code("o: constsig 42.0 >> mul 0.5 ");
    engine.parse();
    engine.handle_connection();
    engine.next_block();
    engine.set_code("o: sin 42.0 >> mul 0.6 >> mul 0.3");
    engine.parse();
    // engine.send_msg("o", 0, (0, "440."));
    // engine.next_block();
}