pub struct Plate {
    graph: GlicolGraph,
}

impl Plate {
    pub fn new() -> Self {

    }
    pub fn sr() -> Self {}

    pub fn room() -> Self {}

    pub fn mix() -> Self {}

    pub fn build() -> Self {
        let graph = make_graph!{
            out: sin 440.0 >> mul _am;
            _am: sin 10.0 >> mul 0.1 >> add 0.5;
        }
    }
}

impl Node for Plate {

}