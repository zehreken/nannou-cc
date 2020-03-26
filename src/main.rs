use std::env;

mod a0;
pub use crate::a0::*;
mod a1;
pub use crate::a1::*;
mod a2;
pub use crate::a2::*;

const A0: &str = "--a0";
const A1: &str = "--a1";
const A2: &str = "--a2";
const A3: &str = "--a3";
const A4: &str = "--a4";
const A5: &str = "--a5";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "OPTIONS:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
            A0, A1, A2, A3, A4, A5
        );
    } else if args.len() > 2 {
        println!("Too many arguments!");
    } else {
        let arg: &str = &args[1][..];
        match arg {
            A0 => start_a0(),
            A1 => start_a1(),
            A2 => start_a2(),
            _ => println!("Unknown arguement!"),
        }
    }
}
