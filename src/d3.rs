use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, AddAssign};
use std::path::Path;

// for part 1
fn part_one(square: i32) -> i32 {
    // only defined for square >= 1
    if square < 1 {
        panic!("steps() called with square < 1");
    }

    // special case to avoid remainder mod 0
    if square == 1 {
        return 0;
    }

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

// for part 2
#[derive(Copy, Clone)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

fn spiral_direction(loc: &Vector2) -> Vector2 {
    if loc.y <= -loc.x.abs() {
        Vector2 { x: 1, y: 0 }
    } else if loc.x <= -loc.y.abs() {
        Vector2 { x: 0, y: -1 }
    } else if loc.y >= loc.x.abs() {
        Vector2 { x: -1, y: 0 }
    } else {
        Vector2 { x: 0, y: 1 }
    }
}

struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn empty() -> Self {
        Self { grid: Vec::new() }
    }

    fn internal_idx(external_idx: i32) -> usize {
        if external_idx >= 0 {
            2 * (external_idx as usize)
        } else {
            (-2 * external_idx) as usize - 1
        }
    }

    fn write(&mut self, loc: &Vector2, val: i32) {
        let ix = Grid::internal_idx(loc.x);
        let iy = Grid::internal_idx(loc.y);
        while ix + 1 > self.grid.len() {
            self.grid.push(Vec::new());
        }
        while iy + 1 > self.grid[ix].len() {
            self.grid[ix].push(0);
        }
        self.grid[ix][iy] = val;
    }

    fn read(&self, loc: &Vector2) -> i32 {
        let ix = Grid::internal_idx(loc.x);
        let iy = Grid::internal_idx(loc.y);
        if ix + 1 > self.grid.len() {
            0
        } else if iy + 1 > self.grid[ix].len() {
            0
        } else {
            self.grid[ix][iy]
        }
    }

    fn sum_neighbors(&self, loc: &Vector2) -> i32 {
        let mut acc = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx != 0) | (dy != 0) {
                    acc += self.read(&(*loc + Vector2 { x: dx, y: dy }));
                }
            }
        }
        return acc;
    }
}

fn part_two(threshold: i32) -> i32 {
    let mut loc = Vector2 { x: 0, y: 0 };
    let mut value = 1;
    let mut grid = Grid::empty();
    grid.write(&loc, value);
    while value <= threshold {
        loc += spiral_direction(&loc);
        value = grid.sum_neighbors(&loc);
        grid.write(&loc, value);
    }
    return value;
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
    let input_num: i32 = input.parse().unwrap();

    println!("part 1: {}", part_one(input_num));

    println!("part 2: {}", part_two(input_num));
}
