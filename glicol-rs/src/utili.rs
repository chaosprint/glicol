use super::{EngineError};

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

pub fn code_hack(a: &String) -> Result<String, EngineError> {
    let q: String = a.replace("\n", " \n");
    let v: Vec<&str> = q.split(" ").collect();
    println!("{:?}", v);
    let mut b = "".to_string();
    let mut skip = false;
    for (i, c) in v.iter().enumerate() {
        if *c == "sin" || *c == "saw" || *c == "squ" {
            if v[i+1].parse::<f32>().is_ok() {
                b += "const ";
                b += v[i+1];
                b += " >> ";
                b += c;
                skip = true;
            } else {
                b += c;
                b += " ";
            }
            // println!("{:?} {:?}", i, c);
        } else {
            if skip {
                b += " 1 ";
                skip = false;
            } else {
                b += c;
                b += " ";
            }
        }
    };
    Ok(b)
}