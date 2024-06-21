use glicol_parser::*;

#[test]
fn minimal() {
    let res = get_ast("o: sin 440");
    assert!(res.is_ok());
}

#[test]
fn comment_within_chain() {
    let res = get_ast(
        "o: sin 440
    // >> mul 0.5
    >> mul 0.6",
    );
    assert!(res.is_ok());

    let res = get_ast(
        "o: sin 440
// ooooh this is nonsense
>> add 6.00",
    );
    assert!(res.is_ok());
}
