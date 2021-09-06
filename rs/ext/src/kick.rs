use glicol_macro::make_node;

make_node!{
    @Kick (freq: f32)
    bd: sin ~pitch >> mul ~env >> mul 0.9

    ~trigger: speed 4.0 >> seq 60

    ~env: ~trigger >> envperc 0.01 0.4

    ~env_pitch: ~trigger >> envperc 0.01 0.1

    ~pitch: ~env_pitch >> mul #freq >> add 60
}