use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal, FromIterator};
use dasp_interpolate::{
    Interpolator,
    sinc::Sinc,
    linear::Linear,
};
// use dasp_signal::{self as signal, Signal};

fn main() {
    let mut rb = ring_buffer::Fixed::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // rb.set_first(7);
    for i in 11..19 {
        println!("read 0 {:?}", rb[7]);
        let o = rb.push(i  );
        println!("o {:?} rb {:?}", o, rb);
        

    }

    // let v = vec![];
    // let mut source = signal::from_iter::<Vec<f32>>(v);
    // let a = source.next();
    // let b = source.next();
    // let interp = Linear::new(a, b);
    // let interp_vec: Vec<_> = source.scale_hz(interp, 1./7.).take(7).collect();
    // println!("{:?}", interp_vec);

}