extern crate glicol;
use glicol::Engine;
extern crate bela;
use bela::*;

use std::env;
use std::fs;

const BLOCK_SIZE: usize = 16;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let code: String = match args.len() {
        1 => {
            "o: sin 440".to_owned()
        }
        2 => {
            args[1].to_owned()
        },
        3 => {
            let filename = &args[2];
            let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
            contents
        },
        _ => unimplemented!()
    };
    run(code).unwrap();
}

fn run(code: String) -> Result<(), error::Error> {
    let mut setup = |context: &mut Context, engine: &mut Engine<BLOCK_SIZE>| -> Result<(), error::Error> {
        
        engine.make_adc_node(context.analog_in_channels());
        // engine.parse();
        println!("adc frames {}", context.analog_frames());
        println!("adc chan {}", context.analog_in_channels());
        println!("adc {:?} len {}", context.analog_in(), context.analog_in().len());
        println!("audio frames {}", context.audio_frames());
        println!("{}", code);
        engine.update_with_code(&code);
        Ok(())
    };
    let mut cleanup = |_context: &mut Context, _user_data: &mut Engine<BLOCK_SIZE>| {
        println!("Cleaning up");
    };

    let mut render = |context: &mut Context, engine: &mut Engine<BLOCK_SIZE>| {
        engine.set_adc_node_buffer(&context.analog_in(), 8, BLOCK_SIZE, false);
        let buf = engine.next_block().0;
        for i in 0..BLOCK_SIZE {
            (*context.audio_out())[i] = buf[0][i];
            (*context.audio_out())[i + BLOCK_SIZE] = buf[1][i];
        }
    };

    let engine = Engine::<BLOCK_SIZE>::new();
    let user_data = AppData::new(engine, &mut render, Some(&mut setup), Some(&mut cleanup));
    let mut bela_app = Bela::new(user_data);
    let mut settings = InitSettings::default();
    // http://docs.bela.io/structBelaInitSettings.html
    settings.set_period_size(BLOCK_SIZE);
    // settings.set_num_analog_in_channels(4);
    settings.set_uniform_sample_rate(true);
    settings.set_interleave(false);
    bela_app.run(&mut settings)
}