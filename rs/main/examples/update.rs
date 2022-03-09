use glicol::Engine;
// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<16>::new();
    // engine.update(r#"o: constsig 42 >> mul 0.3"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul ~mod; ~mod: constsig 0.9"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul 0.3"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul ~mod; ~mod: constsig 0.9"#);
    // engine.next_block();
   
    // engine.update(r#"o: constsig 42. >> add 0 >> mul ~mod; ~mod: constsig 0.5"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: constsig 42. >> mul ~mod; ~mod: constsig 0.5"#).unwrap();
    // engine.next_block();

    // engine.update(r#"o: imp 10 >> mul 0.1"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: saw 56"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: imp 100 >> mul 0.1"#).unwrap();
    // engine.next_block();
    // engine.update(r#"o: imp 100 >> mul ~mod
    // ~mo: sin 10 >> add 1.0"#).unwrap();
    // engine.next_block();
    engine.update(r#"o: sin 110 >> mul 0.1"#).unwrap();
    println!(" engine.next_block() {:?}", engine.next_block());
    engine.update(r#"o: sin 110 >> add 0.0"#).unwrap();
    println!(" engine.next_block() {:?}", engine.next_block());
    engine.update(r#"o: sin 110 >> mul 0.1"#).unwrap();
    println!(" engine.next_block() {:?}", engine.next_block());
}