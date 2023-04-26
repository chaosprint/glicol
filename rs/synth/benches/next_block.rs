use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glicol_synth::{oscillator::SinOsc, AudioContextBuilder};

fn next_block_benchmark(c: &mut Criterion) {
    let mut context = AudioContextBuilder::<128>::new()
        .sr(44100)
        .channels(2)
        .build();
    let node_a = context.add_stereo_node(SinOsc::new().freq(black_box(440.)));
    context.connect(node_a, context.destination);

    c.bench_function("next_block", |b| {
        b.iter(|| {
            context.next_block();
        })
    });
}

criterion_group!(benches, next_block_benchmark);
criterion_main!(benches);
