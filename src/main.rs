use crate::hardcore_combs::hardcore_combs;

pub mod hardcore;
pub mod hardcore_combs;
pub mod propp_wilson;
pub mod pwl;
pub mod util;

fn main() {
    const GRAPH_SIZE: usize = 3;
    let result: Vec<[[bool; GRAPH_SIZE]; GRAPH_SIZE]> = hardcore_combs::hardcore_combs();
    for solution in &result {
        hardcore::print_board2(solution);
    }
    println!("Wynik = {}", result.len());
}
