use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glicol_synth::{
    operator::{Add, Mul},
    oscillator::SinOsc,
    AudioContextBuilder,
};

fn next_block_benchmark(c: &mut Criterion) {
    let mut context = AudioContextBuilder::<128>::new()
        .sr(44100)
        .channels(2)
        .build(); //black_box(200.)
    let sin1 = context.add_stereo_node(SinOsc::new());
    let mul1 = context.add_stereo_node(Mul::new(0.1));
    let sin2 = context.add_stereo_node(SinOsc::new().freq(black_box(200.)));
    let mul2 = context.add_stereo_node(Mul::new(300.));
    let add2 = context.add_stereo_node(Add::new(600.));
    context.connect(sin1, mul1);
    context.connect(sin2, mul2);
    context.connect(mul2, add2);
    context.connect(add2, sin1);
    context.connect(mul1, context.destination);

    c.bench_function("next_block", |b| {
        b.iter(|| {
            context.next_block();
        })
    });
}

criterion_group!(benches, next_block_benchmark);
criterion_main!(benches);
