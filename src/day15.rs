extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

use crate::computer::{run, ProgramState};

use regex::Regex;


pub fn load_input(name: &str) -> Vec<i64> {
    let mut f = File::open(name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let mut output = Vec::new();
    for el in buffer.split(",") {
        if let Ok(x) = el.trim().parse::<i64>() {
            output.push(x)
        }
    }
    output
}

pub enum Move {
    North = 1,
    South,
    West,
    East,
}

pub enum Status {
    HitWall,
    MoveSuccess,
    FoundOxygen,
}

pub fn part1(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(input);

    loop {
        // Move command
        //
        // Send command to droid
        //
        // Wait for droid to finish move operation
        //
        // Report on status via output
        break;
    }

    0
}

pub fn part2(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(input);
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}
