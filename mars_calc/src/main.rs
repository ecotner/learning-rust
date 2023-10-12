use std::io::{self, Write}; // Write is required by .flush()

fn main() {
    print!("Enter your weight in kg (on Earth): ");
    io::stdout().flush().unwrap();
    // assign variables with `let`
    // variables are immutable by default; need to declare mutability with `mut` keyword
    let mut input = String::new(); // create variable-size string buffer
    // read string from standard input (stdin).
    // the read_line function returns a Result object that could either be Ok or Err;
    // Result.unwrap() returns the value wrapped by Ok (if it exists), otherwise it ends
    // the program.
    io::stdin().read_line(&mut input).unwrap();
    // convert string to floating point value.
    // ignore potential parsing errors for now (e.g. expecting a float but getting text).
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {} kg", mars_weight); // the ! means this is a macro, not a function; use cargo-expand to investigate macro expansions
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 // last line of a function without a semicolon is interpreted as return
}
