pub fn averageu32(numbers: &Vec<u32>) -> f32 {
    numbers.iter().sum::<u32>() as f32 / numbers.len() as f32
}

pub fn averagef64(numbers: &Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() as f64 / numbers.len() as f64
}

pub fn absolute_error(result: f64, real_val: f64) -> f64 {
    return (result - real_val).abs();
}

pub fn averageu64(numbers: &Vec<u32>) -> f64 {
    numbers.iter().sum::<u32>() as f64 / numbers.len() as f64
}
