use glicol::Engine; 
fn main() {
    let mut engine = Engine::<8>::new();
    engine.update_with_code(r#"o: msgsynth \saw 0.01 0.1"#);
    println!("next block {:?}", engine.next_block(vec![]));
}