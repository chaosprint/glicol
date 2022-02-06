use glicol::{Engine};

#[test]
fn helloworld() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("aa: sin 440");
    engine.make_graph().unwrap();
}

#[test]
fn connetion() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("aa: sin 440 >> mul 0.5");
    engine.make_graph().unwrap();
}

#[test]
fn reference() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("aa: sin 440 >> mul ~mod
    ~mod: sin 0.3 >> add 0.5 >> mul 0.5");
    engine.make_graph().unwrap();
}

#[test]
fn seq() { //_60 _ ~a _~b ~c_~a 
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("aa: seq 60 >> mul ~mod
    ~mod: sin 0.3 >> add 0.5 >> mul 0.5");
    engine.make_graph().unwrap();
}