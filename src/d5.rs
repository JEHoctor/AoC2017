use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct JumpState {
    offsets: Vec<i32>,
    position: i32,
    time: i32,
}

impl JumpState {
    fn from_offsets(other_offsets: &[i32]) -> &mut JumpState {
        let mut offsets = other_offsets.to_vec();
        JumpState { offsets: offsets, position: 0, time: 0}
    }

    fn still_in_maze(&self) -> bool {
        0 <= self.position < self.offsets.len()
    }

    fn step(&mut self) {
        if !self.still_in_maze() {
            panic!("Can't step when already outside maze!")
        }

        let offset = self.offsets[self.position];
        self.offsets[self.position] = offset + 1;
        self.position += offset;
        self.time += 1;
    }
}

fn parse_offsets(&str input) -> &mut Vec<i32> {
    let mut ret = Vec<i32>::new();
    for chunk in input[..input.len()-1].split('\n') {
        ret. chunk.parse().unwrap();
    }
    return ret;
}

// for part 1
fn (mut jumps: &mut Vec<i32>) -> i32{

}

fn main() {
    // get the path to the input from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/day/five/input", args[0]);
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

    // println!("part 1: {}", );

    // println!("part 2: {}", );
}
