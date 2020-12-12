use glicol::Engine;
use glicol::EngineError;

fn main () -> Result<(), EngineError>{
    let mut engine = Engine::new();
    engine.set_code("a: imp 4410.0");
    engine.update();
    engine.make_graph()?;
    let _result = engine.gen_next_buf_128()?;
    // for i in 0..128 {
    // println!("{:?}", result.0);
    // };
    Ok(())
}