use std::char;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// for part 1
fn is_valid(passphrase: &str) -> bool {
    let mut words = HashSet::new();
    for word in passphrase.split(' ') {
        if words.contains(word) {
            return false;
        }
        words.insert(word);
    }
    true
}

// for part 2
fn is_valid2(passphrase: &str) -> bool {
    // Words are now considered the same if they are anagrams, so we now store
    // them with their letters sorted alphabetically.
    let mut sorted_words = HashSet::new();
    for word in passphrase.split(' ') {
        let mut word_letters: Vec<char> = word.chars().collect();
        word_letters.sort();
        let sorted_word: String = word_letters.into_iter().collect();
        if sorted_words.contains(&sorted_word) {
            return false;
        }
        sorted_words.insert(sorted_word);
    }
    true
}

fn main() {
    // get the path to the input from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/day/four/input", args[0]);
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

    // remove trailing new line from input
    input.pop();

    // count valid passphrases for both definitions of valid
    let mut n_valid = 0;
    let mut n_valid2 = 0;
    for passphrase in input.split('\n') {
        if is_valid(passphrase) {
            n_valid += 1;
        }
        if is_valid2(passphrase) {
            n_valid2 += 1;
        }
    }

    println!("part 1: {}", n_valid);

    println!("part 2: {}", n_valid2);
}
