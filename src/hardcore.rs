use rand::Rng;
use std::vec;

use crate::util;

pub fn is_feasible<const GRAPH_SIZE: usize>(board: &[[bool; GRAPH_SIZE]; GRAPH_SIZE]) -> bool {
    for i in 0..GRAPH_SIZE {
        for j in 0..GRAPH_SIZE {
            if board[i][j] == true && !are_neighbours_0(i, j, board) {
                return false;
            }
        }
    }
    return true;
}

pub fn create_chessboard<const GRAPH_SIZE: usize>(board: &mut [[bool; GRAPH_SIZE]; GRAPH_SIZE]) {
    for i in 0..GRAPH_SIZE {
        for j in 0..GRAPH_SIZE {
            if i % 2 == 0 && j % 2 == 0 {
                board[i][j] = true;
            }
            if i % 2 == 1 && j % 2 == 1 {
                board[i][j] = true;
            }
        }
    }
}

pub fn print_board2<const GRAPH_SIZE: usize>(board: &[[bool; GRAPH_SIZE]; GRAPH_SIZE]) {
    println!("Board:");
    for i in 0..GRAPH_SIZE {
        for j in 0..GRAPH_SIZE {
            print!("{} ", board[i][j] as usize);
        }
        println!("");
    }
}

pub fn are_neighbours_0<const GRAPH_SIZE: usize>(
    a: usize,
    b: usize,
    board: &[[bool; GRAPH_SIZE]; GRAPH_SIZE],
) -> bool {
    if a > 0 && board[a - 1][b] {
        return false;
    }
    if a < GRAPH_SIZE - 1 && board[a + 1][b] {
        return false;
    }
    if b > 0 && board[a][b - 1] {
        return false;
    }
    if b < GRAPH_SIZE - 1 && board[a][b + 1] {
        return false;
    }
    return true;
}

pub fn hardcore_update<const GRAPH_SIZE: usize>(
    a: usize,
    b: usize,
    coin_flip: u32,
    board: &mut [[bool; GRAPH_SIZE]; GRAPH_SIZE],
) {
    if coin_flip == 1 && are_neighbours_0::<GRAPH_SIZE>(a, b, board) {
        board[a][b] = true;
    } else {
        board[a][b] = false;
    }
}

pub fn hardcore_try<const GRAPH_SIZE: usize>(steps: usize) -> bool {
    let mut board = [[false; GRAPH_SIZE]; GRAPH_SIZE];
    for _step in 0..steps {
        let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
        let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
        let coin_flip = rand::thread_rng().gen_range(0..2);
        hardcore_update(a, b, coin_flip, &mut board);
    }
    return board == [[false; GRAPH_SIZE]; GRAPH_SIZE];
}

pub fn hardcore_tries<const GRAPH_SIZE: usize>(steps: usize, tries: usize) -> usize {
    let mut sum = 0;
    for _ in 0..tries {
        sum += hardcore_try::<GRAPH_SIZE>(steps) as usize;
    }
    return sum;
}

pub fn hardcore_go<const GRAPH_SIZE: usize>(steps: usize, tries: usize) -> (f64, f64) {
    let sum = hardcore_tries::<GRAPH_SIZE>(steps, tries);
    let mean = sum as f64 / tries as f64;
    let a = sum as f64 * (1.0 - mean).powi(2);
    let b = (tries - sum) as f64 * (0.0 - mean).powi(2);
    let var = (a + b) / (tries - 1) as f64;
    return (mean, var);
}

pub fn real_mean<const GRAPH_SIZE: usize>() -> f64 {
    let expected_val;
    if GRAPH_SIZE == 3 {
        expected_val = 1.0 / 63.0;
    } else if GRAPH_SIZE == 4 {
        expected_val = 1.0 / 1234.0;
    } else if GRAPH_SIZE == 5 {
        expected_val = 1.0 / 55447.0;
    } else {
        expected_val = -1.0;
    }
    return expected_val;
}

pub fn hardcore_error<const GRAPH_SIZE: usize>(result: f64) -> f64 {
    return (result - real_mean::<GRAPH_SIZE>()).abs();
}

pub fn hardcore_goes<const GRAPH_SIZE: usize>(
    goes: usize,
    tries: usize,
    steps: usize,
) -> (f64, f64) {
    println!("Go for tries = {}, steps = {}", tries, steps);
    let expected_val = real_mean::<GRAPH_SIZE>();
    let mut errors: Vec<f64> = vec![];
    for _ in 0..goes {
        let (mean, _) = hardcore_go::<GRAPH_SIZE>(steps, tries);
        println!("Result = {}", mean);
        errors.push((mean - expected_val).abs());
    }
    let average_abs_error = util::averagef64(&errors);
    return (average_abs_error, average_abs_error / expected_val);
}

pub fn hardcore_simulations<const GRAPH_SIZE: usize>(goes: usize) {
    println!("Simulation for GRAPH_SIZE = {}", GRAPH_SIZE);
    let tries;
    let steps;
    if GRAPH_SIZE == 3 {
        steps = [1024, 64, 128, 256, 512];
        tries = [1000000, 10000, 100000];
    } else if GRAPH_SIZE == 4 {
        steps = [64, 128, 256, 512, 1024];
        tries = [10000, 100000, 1000000];
    } else if GRAPH_SIZE == 5 {
        steps = [128, 256, 512, 1024, 2048];
        tries = [10000000, 0, 0];
    } else {
        return;
    }
    for tr in tries {
        for step in steps {
            let (error_avg, err_rel) = hardcore_goes::<GRAPH_SIZE>(goes, tr, step);
            println!("Error avg = {}, err_rel = {}", error_avg, err_rel);
        }
    }
}
