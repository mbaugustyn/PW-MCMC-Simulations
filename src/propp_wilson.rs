use crate::hardcore;
use crate::hardcore_combs;
use crate::util;
use rand::Rng;
use std::vec;

fn hardcore_update_anti<const GRAPH_SIZE: usize>(
    a: usize,
    b: usize,
    coin_flip: u32,
    states: &mut Vec<[[bool; GRAPH_SIZE]; GRAPH_SIZE]>,
) {
    if states.len() != 2 {
        println!("There should be only 2 states");
    }
    let state0_copy = states[0].clone();
    let state1_copy = states[1].clone();

    /* State 0 */
    if coin_flip == 1 && hardcore::are_neighbours_0::<GRAPH_SIZE>(a, b, &state1_copy) {
        states[0][a][b] = true;
    } else {
        states[0][a][b] = false;
    }
    /* State 1 */
    if coin_flip == 1 && hardcore::are_neighbours_0::<GRAPH_SIZE>(a, b, &state0_copy) {
        states[1][a][b] = true;
    } else {
        states[1][a][b] = false;
    }
}

fn states_converged<const GRAPH_SIZE: usize>(
    states: &Vec<[[bool; GRAPH_SIZE]; GRAPH_SIZE]>,
) -> bool {
    let mut previous_state: &[[bool; GRAPH_SIZE]; GRAPH_SIZE] = &states[0];
    for state in states.iter().skip(1) {
        if state != previous_state {
            return false;
        }
        previous_state = state;
    }
    return true;
}

/* Return if we generated a comb 0 and how many 2^m steps we needed to converge */
// PW type 1 -> classic PW
// PW type 2 -> PW Monotone Sandiwching
// PW type 3 -> PW Anti Monotone Sandiwching
fn propp_wilson_hardcore_try<const GRAPH_SIZE: usize>(pw_type: usize) -> (bool, u32) {
    let mut m: u32 = 1;
    let mut update_function = vec![];

    let mut states_original = vec![];
    if pw_type == 1 {
        states_original = hardcore_combs::hardcore_combs::<GRAPH_SIZE>();
    } else {
        states_original.push([[false; GRAPH_SIZE]; GRAPH_SIZE]);
        let chessboard_config = [[true; GRAPH_SIZE]; GRAPH_SIZE];
        // hardcore::create_chessboard(&mut chessboard_config);
        states_original.push(chessboard_config);
    }

    let mut states = states_original.clone();

    while !states_converged(&states) {
        states = states_original.clone();
        /* Create new U_i */
        for _ in 0..i32::pow(2, m) {
            let a = rand::thread_rng().gen_range(0..GRAPH_SIZE);
            let b = rand::thread_rng().gen_range(0..GRAPH_SIZE);
            let coin_flip = rand::thread_rng().gen_range(0..2);
            update_function.push((a, b, coin_flip));
        }
        /* Use update function for all of the states*/
        for t in (0..i32::pow(2, m)).rev() {
            let (a, b, coin_flip) = update_function[t as usize];
            if pw_type == 3 {
                hardcore_update_anti(a, b, coin_flip, &mut states);
            } else {
                for state in &mut states {
                    hardcore::hardcore_update(a, b, coin_flip, state);
                }
            }
        }
        m += 1;
    }
    if !hardcore::is_feasible(&states[0]) {
        println!("Produces an unfeasible result");
    }
    /* All states at this point are the same and we generated an unbiased sample */
    let is_comb0 = states[0] == [[false; GRAPH_SIZE]; GRAPH_SIZE];
    return (is_comb0, m);
}

/* Returns mean of steps and comb0 */
fn propp_wilson_hardcore_tries<const GRAPH_SIZE: usize>(
    tries: usize,
    pw_type: usize,
) -> (f64, f32) {
    let mut steps: Vec<u32> = vec![];
    let mut counter = 0;
    for _ in 0..tries {
        let (is_comb0, m) = propp_wilson_hardcore_try::<GRAPH_SIZE>(pw_type);
        steps.push(m);
        counter += is_comb0 as usize;
    }
    return (counter as f64 / tries as f64, util::averageu32(&steps));
}

fn propp_wilson_hardcore_runs<const GRAPH_SIZE: usize>(
    tries: usize,
    goes: usize,
    pw_type: usize,
) -> (f64, f64) {
    let mut errors: Vec<f64> = vec![];
    println!("Tries = {}", tries);
    for go in 0..goes {
        let (res, avg_steps) = propp_wilson_hardcore_tries::<GRAPH_SIZE>(tries, pw_type);
        errors.push(hardcore::hardcore_error::<GRAPH_SIZE>(res));
        println!("Go = {}, res = {}, avg_steps = {}", go, res, avg_steps);
    }
    let expected_val = hardcore::real_mean::<GRAPH_SIZE>();
    let average_abs_error = util::averagef64(&errors);
    return (average_abs_error, average_abs_error / expected_val);
}

pub fn propp_wilson_hardore_simulations<const GRAPH_SIZE: usize>(pw_type: usize) {
    if pw_type < 1 || pw_type > 3 {
        println!("Incorrect pw_type [{}]", pw_type);
        return;
    }
    println!("PW for GRAPH_SIZE = {}, type = {}", GRAPH_SIZE, pw_type);
    let tries;
    if GRAPH_SIZE == 3 {
        tries = [10000, 100000, 1000000];
    } else if GRAPH_SIZE == 4 {
        tries = [10000, 100000, 1000000];
    } else if GRAPH_SIZE == 5 {
        tries = [1000000, 10000000, 1];
    } else {
        return;
    }
    const GOES: usize = 5;
    for tr in tries {
        let (avg_abs_err, avg_rel_err) =
            propp_wilson_hardcore_runs::<GRAPH_SIZE>(tr, GOES, pw_type);
        println!(
            "Avg abs err = {}, avg_rel_err = {}",
            avg_abs_err, avg_rel_err
        );
    }
}
