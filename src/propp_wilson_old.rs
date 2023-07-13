use crate::hardcore;
use crate::util;
use rand::Rng;
use std::vec;

fn hardcore_update_anti<const GRAPH_SIZE: usize>(
    a: usize,
    b: usize,
    coin_flip: u32,
    board1: &mut [[bool; GRAPH_SIZE]; GRAPH_SIZE],
    board2: &[[bool; GRAPH_SIZE]; GRAPH_SIZE],
) {
    if coin_flip == 1 && hardcore::are_neighbours_0::<GRAPH_SIZE>(a, b, board2) {
        board1[a][b] = true;
    } else {
        board1[a][b] = false;
    }
}

pub fn simulate_hardcore_anti<const GRAPH_SIZE: usize>(goes: usize, tries: usize) {
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

    println!("Rozmiar grafu = {}, Liczba prob = {}", GRAPH_SIZE, tries);
    max_m = 0;
    for _go in 0..goes {
        let mut counter = 0;

        for _try in 0..tries {
            let mut m = 1;

            let mut chessboard_config = [[true; GRAPH_SIZE]; GRAPH_SIZE];
            let mut zeros_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];
            let mut update_function = vec![];

            while chessboard_config != zeros_config {
                zeros_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];
                chessboard_config = [[true; GRAPH_SIZE]; GRAPH_SIZE];

                for _ in 0..i32::pow(2, m) {
                    let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                    let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                    let coin_flip = rand::thread_rng().gen_range(0..2);
                    update_function.push((a, b, coin_flip));
                }
                for t in (0..i32::pow(2, m)).rev() {
                    let (a, b, coin_flip) = update_function[t as usize];

                    let zero_copy = zeros_config.clone();
                    let one_copy = chessboard_config.clone();

                    hardcore_update_anti(a, b, coin_flip, &mut zeros_config, &one_copy);
                    hardcore_update_anti(a, b, coin_flip, &mut chessboard_config, &zero_copy);
                }
                m += 1;
            }
            if !hardcore::is_feasible::<GRAPH_SIZE>(&zeros_config)
                || !hardcore::is_feasible::<GRAPH_SIZE>(&chessboard_config)
            {
                println!("COS NIE HALO");
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
        println!("== Podejscie {} == ", _go + 1);
        println!("Wynik = {}", wynik);
        blad.push(util::absolute_error(wynik, expected_val));
    }

    println!("Najwieksze m = {}", max_m);
    println!("Srednia krokow = {}", util::averageu32(&steps));
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
            hardcore::create_chessboard(&mut chessboard_config);
            let mut zeros_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];

            let mut update_function = vec![];

            while chessboard_config != zeros_config {
                zeros_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];
                chessboard_config = [[false; GRAPH_SIZE]; GRAPH_SIZE];
                hardcore::create_chessboard::<GRAPH_SIZE>(&mut chessboard_config);

                for _ in 0..i32::pow(2, m) {
                    let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                    let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
                    let coin_flip = rand::thread_rng().gen_range(0..2);
                    update_function.push((a, b, coin_flip));
                }
                for t in (0..i32::pow(2, m)).rev() {
                    let (a, b, coin_flip) = update_function[t as usize];
                    hardcore::hardcore_update(a, b, coin_flip, &mut zeros_config);
                    hardcore::hardcore_update(a, b, coin_flip, &mut chessboard_config);

                    if !hardcore::is_feasible::<GRAPH_SIZE>(&zeros_config)
                        || !hardcore::is_feasible::<GRAPH_SIZE>(&chessboard_config)
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

pub fn propp_wilson_hardcore_simulations_graphsize<const GRAPH_SIZE: usize>(goes: usize) {
    println!("Simulation for GRAPH_SIZE = {}", GRAPH_SIZE);
    let tries;
    if GRAPH_SIZE == 3 {
        tries = [10000, 100000, 1000000];
    } else if GRAPH_SIZE == 4 {
        tries = [10000, 100000, 1000000];
    } else if GRAPH_SIZE == 5 {
        tries = [0, 100000, 1000000];
    } else {
        tries = [0, 0, 0];
    }
    for tr in tries {
        propp_wilson_hardcore::<GRAPH_SIZE>(goes, tr);
    }
}
pub fn propp_wilson_simulations() {
    const GOES: usize = 5;
    propp_wilson_hardcore_simulations_graphsize::<3>(GOES);
    propp_wilson_hardcore_simulations_graphsize::<4>(GOES);
    propp_wilson_hardcore_simulations_graphsize::<5>(GOES);
}
