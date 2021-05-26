use dasp_graph::{Buffer, Input, Node};
use super::super::{Engine, Rule, NodeData, BoxedNodeSend, EngineError, ndef};
use pest::iterators::Pairs;

ndef!(Plate, new2, {
    "~dry: ~input

    ~wet: ~dry >> onepole 0.7
    >> delay 50.0 >> allpass 4.771 0.75 >> allpass 3.595 0.75
    >> allpass 12.72 0.625 >> allpass 9.307 0.625
    >> add ~back
    >> allpass ~mod 0.7X
    
    ~mod: sin 0.1 >> linrange 26.0 35.0
    
    ~aa: ~wet >> delayn 394.0
    
    ~ab: ~aa >> delayn 2800.0
    
    ~ac: ~ab >> delayn 1204.0
    
    ~ba: ~ac >> delayn 2000.0 >> onepole 0.1
    >> allpass 7.596 0.5
    
    ~bb: ~ba >> allpass 35.78 0.5
    
    ~bc: ~bb >> allpass ~mod 0.5
    
    ~ca: ~bc >> delayn 179.0
    
    ~cb: ~ca >> delayn 2679.0
    
    ~cc: ~cb >> delayn 3500.0 >> mul 0.3
    
    ~da: ~cc >> allpass 30.0 0.7 >> delayn 522.0
    
    ~db: ~da >> delayn 2400.0
    
    ~dc: ~db >> delayn 2400.0
    
    ~ea: ~dc >> onepole 0.1 >> allpass 6.2 0.7
    
    ~eb: ~ea >> allpass 34.92 0.7
    
    ~fa: ~eb >> allpass 20.4 0.7 >> delayn 1578.0
    
    ~fb: ~fa >> delayn 2378.0
    
    ~back: ~fb >> delayn 2500.0 >> mul 0.3
    
    ~subtract_left: ~bb >> add ~db >> add ~ea >> add ~fa >> mul -1.0
    
    ~left: ~aa >> add ~ab >> add ~cb >> add ~subtract_left
    >> mul {a} >> add ~drym
    
    ~sub_right: ~eb >> add ~ab >> add ~ba >> add ~ca >> mul -1.0
    
    ~right ~da >> add ~db >> add ~fb >> add ~sub_right
    >> mul {a} >> add ~drym
    
    ~drym: ~dry >> mul 0.9
    
    out: mix ~left ~right"
});