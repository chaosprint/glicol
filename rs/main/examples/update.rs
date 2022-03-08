use glicol::Engine;
// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<128>::new();
    // engine.update(r#"o: constsig 42 >> mul 0.3"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul ~mod; ~mod: constsig 0.9"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul 0.3"#);
    // engine.next_block();
    // engine.update(r#"o: constsig 42 >> mul ~mod; ~mod: constsig 0.9"#);
    // engine.next_block();
   
    engine.update(r#"o: constsig 42. >> add 0 >> mul ~mod; ~mod: constsig 0.5"#);
    engine.next_block();
    engine.update(r#"o: constsig 42. >> mul ~mod; ~mod: constsig 0.5"#);
    engine.next_block();
}