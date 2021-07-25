use std::convert::TryInto;
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
    fn from_offsets(other_offsets: &[i32]) -> JumpState {
        let offsets = other_offsets.to_vec();
        JumpState {
            offsets: offsets,
            position: 0,
            time: 0,
        }
    }

    fn still_in_maze(&self) -> bool {
        (0 <= self.position) & (self.position < self.offsets.len().try_into().unwrap())
    }

    fn step(&mut self) {
        if !self.still_in_maze() {
            panic!("Can't step when already outside maze!")
        }

        let pos_as_usize: usize = self.position.try_into().unwrap();
        let offset = self.offsets[pos_as_usize];
        self.offsets[pos_as_usize] = offset + 1;
        self.position += offset;
        self.time += 1;
    }
}

fn parse_offsets(input: &str) -> Vec<i32> {
    let mut ret = Vec::<i32>::new();
    for chunk in input[..input.len() - 1].split('\n') {
        ret.push(chunk.parse().unwrap());
    }
    ret
}

// for part 1
fn count_steps(offsets: &[i32]) -> i32 {
    let mut jstate = JumpState::from_offsets(offsets);
    while jstate.still_in_maze() {
        jstate.step();
    }
    jstate.time
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

    let offsets = parse_offsets(&input);

    println!("part 1: {}", count_steps(&offsets));

    // println!("part 2: {}", );
}
