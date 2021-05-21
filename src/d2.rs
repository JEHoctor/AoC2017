use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // get the path to the input from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/day/two/input", args[0]);
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

    // state variables for parsing the block of numbers
    let mut acc = 0;
    let mut max_sf = 0;
    let mut min_sf = 0;
    let mut row_init = false;
    let mut word_start = 0;
    let mut in_word = false;

    for (i, c) in input.chars().enumerate() {
        match c {
            '\n' | '\t' => {
                if in_word {
                    let value = (&input[word_start..i]).parse::<i32>().unwrap();
                    if row_init {
                        if value > max_sf {
                            max_sf = value;
                        } else if value < min_sf {
                            min_sf = value;
                        }
                    } else {
                        max_sf = value;
                        min_sf = value;
                        row_init = true;
                    }
                }
                in_word = false;
                if (c == '\n') & row_init {
                    acc += max_sf - min_sf;
                    row_init = false;
                }
            },
            _ => {
                if !in_word {
                    in_word = true;
                    word_start = i;
                }
            },
        }
    }

    println!("part 1: {}", acc);

    // println!("part 2: {}", );
}
