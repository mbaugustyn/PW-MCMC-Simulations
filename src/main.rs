use crate::hardcore_combs::hardcore_combs;
pub mod hardcore;
pub mod hardcore_combs;
pub mod propp_wilson;
pub mod pwl;
pub mod util;

fn main() {
    for _ in 0..5 {
        pwl::pwl_test::<4, 1000>();
    }
}
