use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // get the path to the input from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/day/one/input", args[0]);
        std::process::exit(1);
    }
    let input_path = Path::new(&args[1]);

    // open the file
    let display = input_path.display();
    let mut file = match File::open(&input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    // remove the new line at the end of the input
    input.pop();

    // compute the sum as described in the problem statement
    let mut acc = 0;
    let mut prev_char = input.chars().last().unwrap();
    for c in input.chars() {
        if c == prev_char {
            acc += c.to_digit(10).unwrap();
        }
        prev_char = c;
    }
    println!("part 1: {}", acc);

    // solution to part 2
    let mut acc2 = 0;
    let half_input_len = input.len() / 2;
    let first_half = &input[..half_input_len];
    let second_half = &input[half_input_len..];
    for (c1, c2) in first_half.chars().zip(second_half.chars()) {
        if c1 == c2 {
            acc2 += c1.to_digit(10).unwrap();
        }
    }

    // We need to double acc2 because the sum as specified should have been
    // over the full list.
    acc2 *= 2;
    println!("part 2: {}", acc2);
}
