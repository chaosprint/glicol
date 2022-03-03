use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device_i = host.default_input_device().unwrap();
    let device_o = host.default_output_device().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let stream_i = device_i.build_input_stream(
        &cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(44100),
            buffer_size: cpal::BufferSize::Fixed(128),
        },
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for i in data {
                tx.send(*i).unwrap();
            }
        },
        |err: cpal::StreamError| {
            eprintln!("error: {}", err);
        },
    )?;
    let stream_o = device_o.build_output_stream(
        &cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(44100),
            buffer_size: cpal::BufferSize::Fixed(128),
        },
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for i in data.iter_mut() {
                *i = rx.recv().unwrap_or(0.0);
            }
        },
        |err: cpal::StreamError| {
            eprintln!("error: {}", err);
        },
    )?;
    stream_i.play()?;
    stream_o.play()?;
    std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
}