use std::env;
mod sketches;
use sketches::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "OPTIONS:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
            A0, A1, A2, A3, A4, A5, A6, A7, A8,
        );
    } else if args.len() > 2 {
        println!("Too many arguments!");
    } else {
        let arg: &str = &args[1][..];
        match arg {
            A0 => start_a0(),
            A1 => start_a1(),
            A2 => start_a2(),
            A3 => start_a3(),
            A4 => start_a4(),
            A5 => start_a5(),
            A6 => start_a6(),
            A7 => start_a7(),
            A8 => start_a8(),
            A9 => start_a9(),
            _ => eprintln!("Unknown argument!"),
        }
    }
}
