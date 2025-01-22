use glicol::Engine;
fn main() {
    let mut engine = Engine::<8>::new();
    engine
        .update_with_code(r#"o: constsig 42 >> pan 0.9"#)
        .unwrap();
    println!("next block {:?}", engine.next_block(vec![]));
}
