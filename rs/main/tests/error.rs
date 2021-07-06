use glicol::{Engine, EngineError};

// https://stackoverflow.com/questions/53124930/how-do-you-test-for-a-specific-rust-error
macro_rules! assert_err {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
        }
    }
}

#[test]
fn wrongnodename() {
    let mut engine = Engine::new(44100);
    engine.set_code("nn:dasadsdas 42");
    assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
}

#[test]
fn incompletenodename() {
    let mut engine = Engine::new(44100);
    engine.set_code("nn: si");
    engine.make_graph().unwrap()
    // assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
}

// #[test]
// fn wrongnodename() {
//     let mut engine = Engine::new(44100);
//     engine.set_code("nn:dasadsdas 42");
//     assert_err!(engine.make_graph(), Err(EngineError::ParaTypeError(_)));
// }