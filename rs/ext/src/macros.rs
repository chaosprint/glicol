#[macro_export]
macro_rules! amplfo {
    ($size: expr =>  $data: expr) => {
        AmpLFO::<$size>::new($data)
    };
}

#[macro_export]
macro_rules! plate {
    ($size: expr => $data: expr) => {
        Plate:: <$size>::new($data)
    };
}
