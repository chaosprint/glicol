use glicol::*;

fn main() {
    // let input = "~op1: sin 1500 >> mul ~env_amp >> mul 0.5";
    let input = r#"
    // this is a comment
    o: seq 60 _ _60 >> meta "bd808" \808bd >> lpf ~mod 0.1
    // seq's params have some unique syntax
    ~mod: sin 0.2 >> range 300 800
    "#;
    let tokens = tokenize(input).unwrap();
    println!("tokens {:?}", tokens);
    let res = parse(&tokens);
    println!("res {:?}", res);
}
