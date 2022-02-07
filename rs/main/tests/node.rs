use glicol::{Engine};

#[test]
fn shape() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("nn: shape 0.1, 1.0 | 0.1, 0.2");
    engine.make_graph().unwrap();
}

// extension

#[test]
fn bd() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("bd: speed 4.0 >> seq 60 >> bd 0.03");
    engine.make_graph().unwrap();
}

