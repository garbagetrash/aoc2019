extern crate regex;

use std::fs::File;
use std::io::prelude::*;


pub fn load_input(name: &str) -> Vec<i32> {
    let mut f = File::open(name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let mut output = Vec::new();
    for el in buffer.trim().chars() {
        output.push(el.to_digit(10).unwrap() as i32);
    }
    output
}

pub fn phase(input: &Vec<i32>) -> Vec<i32> {
    let base_pattern = vec![0, 1, 0, -1];

    let mut output = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let mut result = 0;

        // For each output element first build the pattern
        let mut pattern = Vec::with_capacity(input.len());
        let mut cntr = 1;
        loop {
            let mut bp_idx = cntr / (i + 1);
            bp_idx = bp_idx % base_pattern.len();
            pattern.push(base_pattern[bp_idx]);

            cntr += 1;
            if cntr - 1 >= input.len() {
                break;
            }
        }

        for j in 0..input.len() {
            result += pattern[j] * input[j];
        }
        output.push((result % 10).abs());
    }
    output
}

pub fn part1(input: &Vec<i32>) -> String {
    let mut step = input.clone();

    for _ in 0..100 {
        step = phase(&step);
    }
    step.iter().take(8).map(|c| c.to_string()).collect::<Vec<_>>().join("")
}

pub fn part2(input: &Vec<i32>) -> i64 {
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
