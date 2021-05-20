#[test]
fn hello_livecoding () {
    use glicol::{Engine, EngineError};  
    let mut engine = Engine::new();
    engine.set_code("aa: sin 440");
    engine.make_graph();
}