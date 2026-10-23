use glicol_synth::{
    filter::{ResonantHighPassFilter, ResonantLowPassFilter},
    oscillator::SawOsc,
    signal::ConstSig,
    AudioContextBuilder, Node,
};

const SR: usize = 44100;

/// Runs `source >> filter` for one second and returns every output sample.
fn render(
    source: impl Node<128> + Send + 'static,
    filter: impl Node<128> + Send + 'static,
) -> Vec<f32> {
    let mut context = AudioContextBuilder::<128>::new().sr(SR).channels(1).build();
    let a = context.add_mono_node(source);
    let b = context.add_mono_node(filter);
    context.chain(vec![a, b, context.destination]);
    (0..SR / 128)
        .flat_map(|_| context.next_block()[0].to_vec())
        .collect()
}

fn peak(samples: &[f32]) -> f32 {
    samples.iter().fold(0.0, |m, x| m.max(x.abs()))
}

#[test]
fn highpass_rejects_dc() {
    let out = render(
        ConstSig::new(1.0),
        ResonantHighPassFilter::new().cutoff(90.0).q(1.0),
    );
    let tail = &out[out.len() - 4410..];
    assert!(peak(tail) < 1e-3, "DC leaks through hpf: {}", peak(tail));
}

#[test]
fn highpass_stays_bounded_on_low_saw() {
    let out = render(
        SawOsc::new().freq(65.0),
        ResonantHighPassFilter::new().cutoff(90.0).q(1.0),
    );
    assert!(peak(&out) < 2.0, "hpf output peak {}", peak(&out));
}

#[test]
fn lowpass_passes_dc_at_unity() {
    let out = render(
        ConstSig::new(1.0),
        ResonantLowPassFilter::new().cutoff(1000.0).q(1.0),
    );
    let last = *out.last().unwrap();
    assert!((last - 1.0).abs() < 1e-3, "lpf DC gain {last}");
}
