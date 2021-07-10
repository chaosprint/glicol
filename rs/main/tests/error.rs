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

// #[derive(Debug, PartialEq)]
// pub enum EngineError {
//     NonExistControlNodeError(String), // handled
//     ParameterError((usize, usize)), // handled
//     SampleNotExistError((usize, usize)), // handled
//     InsufficientParameter((usize, usize)),
//     NotModuableError((usize, usize)),
//     ParaTypeError((usize, usize)),
//     NodeNameError((String, usize, usize)),  // handled
//     ParsingError(pest::error::Error<glicol_parser::Rule>), // handled
//     HandleNodeError, // handled
// }

#[test]
fn wrongnodename() {
    let mut engine = Engine::new(44100);
    engine.set_code("nn: dasadsdas 42");
    assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
}

// this might be report as non existent ref
#[test]
fn incompletenodename() {
    let mut engine = Engine::new(44100);
    engine.set_code("nn: si");
    assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
    // engine.make_graph().unwrap()
    // assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
}

#[test]
fn noparas() {
    let mut engine = Engine::new(44100);
    engine.set_code("nn: sin");
    assert_err!(engine.make_graph(), Err(EngineError::InsufficientParameter(_)));
    // engine.make_graph().unwrap()
    // assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
}

// #[test]
// fn wrongnodename() {
//     let mut engine = Engine::new(44100);
//     engine.set_code("nn:dasadsdas 42");
//     assert_err!(engine.make_graph(), Err(EngineError::ParaTypeError(_)));
// }