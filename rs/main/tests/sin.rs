use glicol::*;

#[test]
fn pan() {
    let mut engine = Engine::<128>::new();
    engine.update_with_code(r#"o: sin 440 >> pan 0.5"#);
}
