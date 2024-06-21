use glicol::Engine;
fn main() {
    let mut engine = Engine::<8>::new();
    engine
        .update_with_code(r#"o: msgsynth \saw 0.01 0.1"#)
        .unwrap();
    println!("next block {:?}", engine.next_block(vec![]));
}
