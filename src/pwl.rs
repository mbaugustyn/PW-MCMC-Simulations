use crate::hardcore;
use rand::Rng;

// if GRAPH_SIZE == 3 {
//     expected_val = 1 as f64 / 63 as f64;
// } else if GRAPH_SIZE == 4 {
//     expected_val = 1 as f64 / 1234 as f64;
// } else if GRAPH_SIZE == 5 {
//     expected_val = 1 as f64 / 55447 as f64;
// } else {
//     return;
// }
pub fn hardcore_run<const GRAPH_SIZE: usize>(steps: usize) -> bool {
    let mut board;
    board = [[false; GRAPH_SIZE]; GRAPH_SIZE];

    for _step in 0..steps {
        let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
        let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
        let coin_flip = rand::thread_rng().gen_range(0..2);
        hardcore::hardcore_update(a, b, coin_flip, &mut board);

        if !hardcore::is_feasible::<GRAPH_SIZE>(&board) {
            println!("COS NIE HALO");
        }
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

pub fn hardcore_sample_var<const GRAPH_SIZE: usize>(M: usize) -> f64 {
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

    let mut sum = 0.0;
    let steps = 128;
    for _ in 0..M {
        let result = hardcore_run::<GRAPH_SIZE>(steps);
        if result {
            sum += f64::powi(f64::abs(1.0 - expected_val), 2);
        } else {
            sum += f64::powi(f64::abs(0.0 - expected_val), 2);
        }
    }
    return sum / (M as f64);
}

pub fn calculate_R(sample_var: f64) -> usize {
    let ERROR = 0.01;
    return ((1.96 * 1.96 * sample_var) / (ERROR * ERROR)) as usize;
}

pub fn interval() -> f64 {
    let S_m = 0.015664619299884355;
    let R = calculate_R(S_m);
    let S_R = hardcore_sample_var::<3>(R);

    println!("R = {}", R);

    return 1.96 * S_R.sqrt() / (R as f64).sqrt();
}
