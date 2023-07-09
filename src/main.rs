use std::os::windows::prelude::IntoRawSocket;

pub mod hardcore;
pub mod hardcore_anti;
pub mod pwl;
pub mod util;

fn main() {
    println!("Result = {}", pwl::pwl_test());
}
