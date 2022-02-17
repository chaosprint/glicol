#![feature(test)]
extern crate test;

use glicol::{Engine};
use test::Bencher;

#[bench]
fn script(b: &mut Bencher) {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code(r#"d: script "
    
        let x = [];
        for i in 0..128 {
            let pha = phase / (44100.0 / 440.0);
            x.push(sin(pha * 2.0 * PI()));
            phase += 1.0;
        };
        x"

    "#);
    b.iter(|| {
        engine.make_graph().unwrap();
        engine.gen_next_buf(&mut [0.0;128]).unwrap();
    });
}

#[bench]
fn sin(b: &mut Bencher) {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code(r#"d: sin 440"#);
    b.iter(|| {
        engine.make_graph().unwrap();
        engine.gen_next_buf(&mut [0.0;128]).unwrap();
    });
}