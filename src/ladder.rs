pub fn simulate_ladder<const GRAPH_SIZE: usize>(goes: usize, tries: usize, steps: usize) {
    let mut blad = vec![];
    let mut a;
    let mut coin_flip;
    let expected_val = 1 as f64 / GRAPH_SIZE as f64;

    for _go in 0..goes {
        let mut counter = 0;
        for _try in 0..tries {
            a = 1;

            for _step in 0..steps {
                coin_flip = rand::thread_rng().gen_range(0..2);
                if coin_flip == 1 {
                    a = std::cmp::min(a + 1, GRAPH_SIZE);
                } else {
                    a = std::cmp::max(a - 1, 1);
                }
            }
            if a == 1 {
                counter += 1;
            }
        }
        let wynik = counter as f64 / tries as f64;
        println!("Wynik = {}", wynik);
        blad.push(util::absolute_error(wynik, expected_val));
    }

    println!("Srednia Bledow  = {}", util::averagef64(&blad));
}

fn propp_wilson_ladder<const GRAPH_SIZE: usize>(goes: usize, tries: usize) {
    let mut blad = vec![];
    let mut steps = vec![];
    let mut a;
    let mut b;
    let mut max_m;

    let expected_val = 1 as f64 / GRAPH_SIZE as f64;
    for _go in 0..goes {
        max_m = 0;
        let mut counter = 0;
        for _try in 0..tries {
            let mut m = 1;

            a = 1;
            b = GRAPH_SIZE;

            let mut update_function = vec![];

            while a != b {
                a = 1;
                b = GRAPH_SIZE;
                for _ in 0..i32::pow(2, m) {
                    let coin_flip = rand::thread_rng().gen_range(0..2);
                    update_function.push(coin_flip);
                }

                for t in (0..i32::pow(2, m)).rev() {
                    let coin_flip = update_function[t as usize];
                    if coin_flip == 1 {
                        a = std::cmp::min(a + 1, GRAPH_SIZE);
                        b = std::cmp::min(b + 1, GRAPH_SIZE);
                    } else {
                        a = std::cmp::max(a - 1, 1);
                        b = std::cmp::max(b - 1, 1);
                    }
                }
                m += 1;
            }
            if a == 1 {
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
}
