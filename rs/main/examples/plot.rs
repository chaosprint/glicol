use glicol::Engine; 
use gnuplot::*;


fn main() {
    let mut engine = Engine::<128>::new();
    engine.update_with_code(r#"o: [0.1=>100, 1/4=> 10.0, 1/3=>50]*(1/2).."#);
    
    // plot part
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..( 220500 / 128) {
        let buf = engine.next_block(vec![]);
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(buf.0[0][i]); // use the buf here
        };
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Glicol output", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(
            &x,
            &y,
            &[Caption("left")],
        );
    fg.show().unwrap();

}