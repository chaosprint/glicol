pub fn midi_or_float(num: String) -> f32 {
    if num.contains(".") {
        num.parse::<f32>().unwrap()
    } else {
        let midi = num.parse::<f32>().unwrap();
        if midi == 0.0 {
            0.0
        } else {
            2.0f32.powf((midi - 69.0)/12.0) * 440.0
        }
    }
}