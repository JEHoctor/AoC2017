use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn steps(square: i32) -> i32 {
    // What ring are we on?
    // Ring Numbering:
    //
    //   9 9 9 9 9 9 9 9 9
    //   9 7 7 7 7 7 7 7 9
    //   9 7 5 5 5 5 5 7 9
    //   9 7 5 3 3 3 5 7 9
    //   9 7 5 3 1 3 5 7 9
    //   9 7 5 3 3 3 5 7 9
    //   9 7 5 5 5 5 5 7 9
    //   9 7 7 7 7 7 7 7 9
    //   9 9 9 9 9 9 9 9 9

    let mut ring = 1;
    while ring * ring < square {
        ring += 2;
    }

    let mut steps = ring / 2;
    let ringm2 = ring - 2;
    steps += ((square - ringm2 * ringm2 - 1) % (ring - 1) - (ring / 2 - 1)).abs();
    return steps;
}

fn main() {
    // get the path to the input from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/day/three/input", args[0]);
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

    input.pop(); // drop the trailing newline
    let square: i32 = input.parse().unwrap();

    println!("part 1: {}", steps(square));

    // println!("part 2: {}", );
}
