use crate::hardcore;
use rand::Rng;
use statrs::statistics::MeanN;
use statrs::statistics::Statistics;

pub fn hardcore_run<const GRAPH_SIZE: usize>(steps: usize) -> bool {
    let mut board;
    board = [[false; GRAPH_SIZE]; GRAPH_SIZE];

    for _step in 0..steps {
        let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
        let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
        let coin_flip = rand::thread_rng().gen_range(0..2);
        hardcore::hardcore_update(a, b, coin_flip, &mut board);
    }
    return board == [[false; GRAPH_SIZE]; GRAPH_SIZE];
}

pub fn hardcore_run_ntimes<const GRAPH_SIZE: usize>(steps: usize, tries: usize) -> f64 {
    let mut sum = 0;
    for _ in 0..tries {
        sum += hardcore_run::<GRAPH_SIZE>(steps) as usize;
    }
    return sum as f64 / tries as f64;
}

pub fn hardcore_sample_var<const GRAPH_SIZE: usize>(m: usize) -> (f64, f64) {
    let steps = 128;
    let mut y: Vec<f64> = Vec::new();
    for _ in 0..m {
        let result = hardcore_run::<GRAPH_SIZE>(steps);
        let x = result as usize;
        y.push(x as f64);
    }
    return (y.clone().variance(), y.clone().mean());
}

pub fn calculate_r(sample_var: f64, err: f64) -> usize {
    return ((1.96 * 1.96 * sample_var) / (err * err)) as usize;
}

pub fn interval<const GRAPH_SIZE: usize>(r: usize) -> (f64, f64) {
    let (s_r, y_r) = hardcore_sample_var::<GRAPH_SIZE>(r);
    let d = 1.96 * s_r.sqrt() / (r as f64).sqrt();
    println!("sr = {}, yr = {}, d = {}", s_r, y_r, d);
    return (y_r - d, y_r + d);
}

pub fn pwl_test() -> f64 {
    const ERR: f64 = 0.001;
    const GRAPH_SIZE: usize = 3;
    const STEPS: usize = 128;
    const M: usize = 100000;

    let (s_m, _) = hardcore_sample_var::<GRAPH_SIZE>(M);
    println!("s_m = {}", s_m);
    let r: usize = calculate_r(s_m, ERR);
    println!("R = {}", r);
    let (left, right) = interval::<GRAPH_SIZE>(r);

    let mut count_int = 0;
    let mut count_b = 0;
    let goes = 10000;
    for go in 0..goes {
        let result = hardcore_run_ntimes::<GRAPH_SIZE>(STEPS, r);
        println!(
            "count_int = {}, count_b = {}, #b = {}",
            (count_int as f64 / go as f64),
            (count_b as f64 / go as f64),
            count_b
        );
        if result > left && result < right {
            count_int += 1;
        }
        if (result - (1 as f64 / 63 as f64)).abs() < ERR {
            count_b += 1;
        }
        // if go % 100 == 0 {
        println!("Go = {}, result = {}", go, result);
        // }
    }
    return count_int as f64 / goes as f64;
}
