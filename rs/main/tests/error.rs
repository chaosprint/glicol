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

/// There are so many possible errors.

// #[test]
// fn seq_para() {
//     let mut engine = Engine::<128>::new(44100);
//     engine.set_code("nn: seq \\wrong");
//     engine.make_graph().unwrap();
// }



#[test]
fn wrongnodename() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("nn: dasadsdas 42");
    assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
}

// this might be report as non existent ref
#[test]
fn incompletenodename() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("nn: si");
    assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
    // engine.make_graph().unwrap()
    // assert_err!(engine.make_graph(), Err(EngineError::NodeNameError(_)));
}

#[test]
fn noparas() {
    let mut engine = Engine::<128>::new(44100);
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


#[test]
fn missing_paras() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("~trigger: speed 8.0 >> seq 60 >> mul 2.0
    ~env: ~trigger >> envperc >> mul 0.2");
    // engine.make_graph().unwrap();
    assert_err!(engine.make_graph(), Err(EngineError::ParsingIncompleteError(_)));
}

#[test]
fn missing_paras_begin() {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("o: sin ");
    assert_err!(engine.make_graph(), Err(EngineError::InsufficientParameter(_)));
    // assert!(matches!(engine.make_graph().unwrap_err(), EngineError::ParsingIncompleteError(70)));
}