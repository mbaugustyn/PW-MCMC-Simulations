use rand::Rng;
use std::vec;

use crate::util;

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

pub fn print_board<const GRAPH_SIZE: usize>(board: &[[u32; GRAPH_SIZE]; GRAPH_SIZE]) {
    println!("Board:");
    for i in 0..GRAPH_SIZE {
        for j in 0..GRAPH_SIZE {
            print!("{} ", board[i][j]);
        }
        println!("");
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

pub fn simulate_hardcore<const GRAPH_SIZE: usize>(goes: usize, tries: usize, steps: usize) {
    println!(
        "\nHardcore for goes = {}, tries = {}, steps = {}",
        goes, tries, steps
    );
    let mut board;
    let mut blad = vec![];
    let expected_val;
    if GRAPH_SIZE == 3 {
        expected_val = 1 as f64 / 63 as f64;
    } else if GRAPH_SIZE == 4 {
        expected_val = 1 as f64 / 1234 as f64;
    } else if GRAPH_SIZE == 5 {
        expected_val = 1 as f64 / 55447 as f64;
    } else {
        return;
    }
    for _go in 0..goes {
        let mut counter = 0;
        for _try in 0..tries {
            board = [[false; GRAPH_SIZE]; GRAPH_SIZE];

            for _step in 0..steps {
                let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                let coin_flip = rand::thread_rng().gen_range(0..2);
                hardcore_update(a, b, coin_flip, &mut board);

                if !is_feasible::<GRAPH_SIZE>(&board) {
                    println!("COS NIE HALO");
                }
            }
            if board == [[false; GRAPH_SIZE]; GRAPH_SIZE] {
                counter += 1;
            }
        }
        let wynik = counter as f64 / tries as f64;
        // println!("Wynik = {}", wynik);
        blad.push(util::absolute_error(wynik, expected_val));
    }
    println!("Srednia Bledow  = {}", util::averagef64(&blad));
    println!(
        "Srednia Bledow wzgledna  = {}",
        util::averagef64(&blad) / expected_val
    );
}

fn propp_wilson_hardcore<const GRAPH_SIZE: usize>(goes: usize, tries: usize) {
    let mut steps = vec![];
    let mut blad = vec![];
    let expected_val;
    if GRAPH_SIZE == 3 {
        expected_val = 1 as f64 / 63 as f64;
    } else if GRAPH_SIZE == 4 {
        expected_val = 1 as f64 / 1234 as f64;
    } else if GRAPH_SIZE == 5 {
        expected_val = 1 as f64 / 55447 as f64;
    } else {
        return;
    }
    let mut max_m;

    for _go in 0..goes {
        let mut counter = 0;
        max_m = 0;

        for _try in 0..tries {
            let mut m = 1;

            let mut chessboard_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];
            create_chessboard(&mut chessboard_config);
            let mut zeros_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];

            let mut update_function = vec![];

            while chessboard_config != zeros_config {
                zeros_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];
                chessboard_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];
                create_chessboard::<GRAPH_SIZE>(&mut chessboard_config);

                for _ in 0..i32::pow(2, m) {
                    let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                    let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                    let coin_flip = rand::thread_rng().gen_range(0..2);
                    update_function.push((a, b, coin_flip));
                }
                for t in (0..i32::pow(2, m)).rev() {
                    let (a, b, coin_flip) = update_function[t as usize];
                    hardcore_update(a, b, coin_flip, &mut zeros_config);
                    hardcore_update(a, b, coin_flip, &mut chessboard_config);

                    if !is_feasible::<GRAPH_SIZE>(&zeros_config)
                        || !is_feasible::<GRAPH_SIZE>(&chessboard_config)
                    {
                        println!("COS NIE HALO");
                    }
                }
                m += 1;
            }
            if zeros_config == [[false; GRAPH_SIZE]; GRAPH_SIZE] {
                counter += 1;
            }
            if m > max_m {
                max_m = m;
            }
            steps.push(m);
        }
        let wynik = counter as f64 / tries as f64;
        println!("\n== Podejscie {} == ", _go + 1);
        println!("Wynik = {}", wynik);
        println!("Najwieksze m = {}", max_m);
        blad.push(util::absolute_error(wynik, expected_val));
    }

    println!("Srednia krokow = {}", util::averageu32(&steps));
    println!("Srednia Bledow  = {}", util::averagef64(&blad));
    println!(
        "Srednia Bledow wzgledna  = {}",
        util::averagef64(&blad) / expected_val
    );
}
