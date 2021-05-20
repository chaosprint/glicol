// #[test]
// fn test_update() {
//     use glicol::Engine;
//     let mut engine = Engine::new();
//     let output_vec: Vec<f32> = vec!();
//     engine.set_code("");
//     engine.make_graph().unwrap();
//     for _ in 0..(43000.0/128.0) as usize {
//         let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
//     }
//     engine.set_code("~aa: sin 440\nout: ~aa 44");

//     engine.make_graph().unwrap();

//     for _ in 0..(43000.0/128.0) as usize {
//         let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
//     }
// }