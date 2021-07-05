use glicol::{Engine};

#[test]
fn hello_livecoding() {
    let mut engine = Engine::new(44100);
    engine.set_code("aa: sin 440");
    engine.make_graph().unwrap();
}

#[test]
fn am() {
    let mut engine = Engine::new(44100);
    engine.set_code("am: sin 440 >> mul ~mod; ~mod: sin 10 >> mul 0.2 >> add 0.5");
    engine.make_graph().unwrap();
}

#[test]
fn noise() {
    let mut engine = Engine::new(44100);
    engine.set_code("nn: noise 42");
    engine.make_graph().unwrap();
}

#[test]
fn rlpf() {
    let mut engine = Engine::new(44100);
    engine.set_code("nn: noise 42 >> rlpf 300.0 1.0");
    engine.make_graph().unwrap();
}