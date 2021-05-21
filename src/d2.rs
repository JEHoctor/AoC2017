use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn p2_row(nums: &mut Vec<i32>) -> i32 {
    nums.sort();

    for (i, n1) in nums.iter().enumerate() {
        for n2 in nums[i+1..].iter() {
            if n2 % n1 == 0 {
                return n2 / n1;
            }
        }
    }

    panic!("did not find a pair of numbers such that one divides the other");
}


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
    let mut row_init = false;
    let mut word_start = 0;
    let mut in_word = false;

    // state variables for computing part 1
    let mut acc = 0;
    let mut max_sf = 0;
    let mut min_sf = 0;

    // state variables for computing part 2
    let mut acc2 = 0;
    let mut nums = Vec::new();

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
                        nums.clear();
                        row_init = true;
                    }
                    nums.push(value);
                }
                in_word = false;
                if (c == '\n') & row_init {
                    acc += max_sf - min_sf;
                    acc2 += p2_row(&mut nums);
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

    println!("part 2: {}", acc2);
}
