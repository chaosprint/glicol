use glicol::Engine;

fn main() {
    let mut engine = Engine::<128>::new();
    // engine.set_code("o: sin 440.0 >> mul 0.5 ");
    // engine.update();
    // engine.next_block();
    // engine.set_code("a: constsig 42 >> mul 0.1");
    // engine.update();
    engine.next_block(vec![]);
    // println!("index_info {:?}", engine.index_info);
    // engine.send_msg("o", 0, (0, "440."));
    // engine.next_block();
}