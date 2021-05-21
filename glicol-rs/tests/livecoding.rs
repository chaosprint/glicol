#[test]
fn hello_livecoding () {
    use glicol::{Engine};  
    let mut engine = Engine::new();
    engine.set_code("aa: sin 440");
    engine.make_graph();
}

#[test]
fn am () {
    use glicol::{Engine};  
    let mut engine = Engine::new();
    engine.set_code("am: sin 440 >> mul ~mod\n\n~mod: sin 10 >> mul 0.2 >> add 0.5");
    engine.make_graph();
}