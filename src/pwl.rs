use crate::hardcore::{self, hardcore_go, hardcore_goes};
use rand::Rng;

pub fn calculate_r(sample_var: f64, err: f64) -> usize {
    return ((1.96 * 1.96 * sample_var) / (err * err)) as usize;
}

pub fn pwl_test() -> f64 {
    const ERR: f64 = 0.0001;
    const GRAPH_SIZE: usize = 4;
    const STEPS: usize = 128;
    const M: usize = 100000;
    const GOES: usize = 1000;
    let expected_val;
    if GRAPH_SIZE == 3 {
        expected_val = 1 as f64 / 63 as f64;
    } else if GRAPH_SIZE == 4 {
        expected_val = 1 as f64 / 1234 as f64;
    } else if GRAPH_SIZE == 5 {
        expected_val = 1 as f64 / 55447 as f64;
    } else {
        return -1.0;
    }

    let (_, var_sample) = hardcore_go::<GRAPH_SIZE>(STEPS, M);
    println!("s_m = {}", var_sample);
    let r: usize = calculate_r(var_sample, ERR);
    println!("R = {}", r);

    let mut count_b = 0;
    for go in 0..GOES {
        let (result_mean, _) = hardcore_go::<GRAPH_SIZE>(STEPS, r);
        if go % 100 == 0 {
            println!("count_b = {}", (count_b as f64 / go as f64));
            println!("Go = {}, result = {}", go, result_mean);
        }
        if (result_mean - expected_val).abs() < ERR {
            count_b += 1;
        }
    }
    return count_b as f64 / GOES as f64;
}
