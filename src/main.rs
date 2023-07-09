use std::os::windows::prelude::IntoRawSocket;

pub mod hardcore;
pub mod hardcore_anti;
pub mod pwl;
pub mod util;

fn main() {
    const GOES: usize = 10;
    const TRIES: usize = 1000000;
    const STEPS: usize = 128;
    const GRAPH_SIZE: usize = 3;

    let interval = pwl::interval();
    let Y = 1 as f64 / 63 as f64;
    let mut count = 0;
    for i in 0..TRIES {
        let result = pwl::hardcore_run_ntimes::<GRAPH_SIZE>(1024, 601);
        if i != 0 && i % 100000 == 0 {
            println!(
                "Count = {}, i = {}, % = {} ",
                count,
                i,
                count as f64 / i as f64
            );
        }
        if result > Y - interval && result < Y + interval {
            count += 1;
        }
    }
    println!("Count = {}, % = {} ", count, count as f64 / TRIES as f64);
}
