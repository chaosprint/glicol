// o: saw 400 >> lpf "100@0.0 200@0.5"(1) 1.0

use glicol::Engine; 
fn main() {
    let mut engine = Engine::<8>::new();
    engine.update_with_code(r#"o: saw 400 >> lpf "100@0.0 200@0.5"(1) 1.0"#);
    println!("next block {:?}", engine.next_block(vec![]));
}