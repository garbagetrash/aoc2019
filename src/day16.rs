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

pub fn phase(input: &Vec<i32>, n_inputs: usize) -> Vec<i32> {
    let base_pattern = vec![0, 1, 0, -1];

    let mut output = Vec::with_capacity(n_inputs * input.len());
    for i in 0..n_inputs * input.len() {
        let mut result = 0;

        for j in 0..n_inputs * input.len() {
            let mut bp_idx = (j + 1) / (i + 1);
            bp_idx = bp_idx % 4;
            let in_idx = j % input.len();
            match base_pattern[bp_idx] {
                1 => result += input[in_idx],
                -1 => result -= input[in_idx],
                _ => continue,
            }
        }
        output.push((result % 10).abs());
    }
    output
}

pub fn part1(input: &Vec<i32>) -> String {
    let mut step = input.clone();

    for _ in 0..100 {
        step = phase(&step, 1);
    }
    step.iter()
        .take(8)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("")
}

pub fn part2(input: &Vec<i32>) -> String {
    let mut step = input.clone();

    for _ in 0..100 {
        step = phase(&step, 1);
    }
    step.iter()
        .take(8)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("")
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
