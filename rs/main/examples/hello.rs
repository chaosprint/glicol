use glicol::Engine; 
// use glicol::{EngineError, get_error_info};

// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<32>::new();
    engine.update_with_code(r#"o: [0.1=>100, 1/2=> 1.0]"#);
    // engine.update_with_code(r#"o: sin 440"#);
    // engine.update_with_code(r#"// a sawtooth osc chained with a onepole filter
    // // the first meta is to write a saw manually
    // out: meta `
    //     f = 220.;
    //     output.pad(128, 0.0);
    //     for i in 0..128 {
    //         output[i] = p * 2. - 1.;
    //         p += f / sr;
    //     };
    //     if p > 1.0 { p -= 1.0 };
    //     output
    // ` >> meta `
    //     r = 1./2000.;
    //     if phase == 0.0 {
    //         z = 0.0
    //     }
    //     output.pad(128, 0.0);
    //     b = (-2.0 * PI() * r).exp();
    //     a = 1.0 - b;
    //     for i in 0..128 {
    //         y = input[i] * a + b * z;
    //         output[i] = y;
    //         z = y;
    //     };
    //     output
    // `
    // // if the script has an input, you can use the "input" variable
    // // the "input" is a 128-size array in web audio"#);
    // match engine.update("o: imp 100 >> mul ~mod
    // ~mo: sin 1 >> mul 0.5 >> add 0.5") {
    //     Ok(_) => {},
    //     Err(e) => {
    //         println!("{:?}", e);
    //         // match e {
    //         //     EngineError::ParsingError(e) => {
    //         //         println!("{:?}", get_error_info(e))
    //         //     },
    //         //     _ => unimplemented!()
    //         // }
    //     }
    // };
    // println!("refpairlist {:?}", engine.refpairlist);
    // engine.update("o: saw 500 >> lpf 100.0 1.0;

    // ~mod: sin 0.2 >> mul 200.0 >> add 500.0");
    // engine.add_sample("\\bb", &[1.0], 1);
    // engine.update(r#"o: imp 1 >> sp \808_0 >> delayms ~mod

    // ~mod: sin 0.2 >> mul 100 >> add 200"#);
    // for e in engine.context.graph.edges(engine.context.destination) {
    //     println!("destinations {:?}", e);
    // }
    println!("next block {:?}", engine.next_block(vec![]));
    // engine.send_msg("o", 0, (0, "1."));
    // engine.next_block();
}