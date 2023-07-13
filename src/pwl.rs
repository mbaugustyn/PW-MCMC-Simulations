use crate::hardcore::hardcore_go;

fn calculate_r(sample_var: f64, err: f64) -> usize {
    return ((1.96 * 1.96 * sample_var) / (err * err)) as usize;
}

pub fn pwl_test<const GRAPH_SIZE: usize, const GOES: usize>() -> f64 {
    let err;
    let expected_val;
    let steps;
    let m;
    if GRAPH_SIZE == 3 {
        expected_val = 1 as f64 / 63 as f64;
        err = 0.001;
        steps = 128;
        m = 100000;
    } else if GRAPH_SIZE == 4 {
        expected_val = 1 as f64 / 1234 as f64;
        err = 0.0001;
        steps = 256;
        m = 1000000;
    } else if GRAPH_SIZE == 5 {
        expected_val = 1 as f64 / 55447 as f64;
        err = 0.00001;
        steps = 512;
        m = 10000000;
    } else {
        return -1.0;
    }
    println!(
        "PWL for graph_size = {}, goes = {}, err = {}, expected_val = {}, steps = {}, m = {}",
        GRAPH_SIZE, GOES, err, expected_val, steps, m
    );

    let (_, var_sample) = hardcore_go::<GRAPH_SIZE>(steps, m);
    println!("s_m = {}", var_sample);
    let r: usize = calculate_r(var_sample, err);
    println!("R = {}", r);

    let mut count_b = 0;
    for _ in 0..GOES {
        let (result_mean, _) = hardcore_go::<GRAPH_SIZE>(steps, r);
        if (result_mean - expected_val).abs() < err {
            count_b += 1;
        }
    }
    println!("Result = {}", count_b as f64 / GOES as f64);
    return 0.0;
}
