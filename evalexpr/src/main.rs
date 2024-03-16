use std::io;
mod evalexpr;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    println!("{0}", evalexpr::evaluate(&input));
}
