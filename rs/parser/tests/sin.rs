use glicol_parser::*;

#[test]
fn minimal() {
    let res = get_ast("o: sin 440");
    assert!(res.is_ok());
}
