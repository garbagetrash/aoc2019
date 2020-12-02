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

#[derive(Clone)]
struct BasePattern {
    offset: usize,
    length: usize,
    idx: usize,
}

impl BasePattern {
    fn new(offset: usize, length: usize) -> BasePattern {
        BasePattern { offset, length, idx: 1 }
    }
}

impl Iterator for BasePattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {

        // If we're done, then we're done
        if self.idx >= self.length + 1 {
            return None;
        }

        let ix = self.idx % (self.offset * 4);

        let mut output = Some(0);
        if ix < self.offset {
            output = Some(0)
        } else if ix < 2 * self.offset {
            output = Some(1)
        } else if ix < 3 * self.offset {
            output = Some(0)
        } else {
            output = Some(-1);
        }

        self.idx += 1;
        output
    }
}

pub fn build_pos_idx_table(length: usize) -> Vec<Vec<usize>> {
    let mut output = vec![];
    for i in 1..length {
        let mut line = vec![];
        for j in i..length {
            let ix = j % (i * 4);
            if ix >= i && ix < 2 * i {
                line.push(j);
            }
        }
        output.push(line);
    }

    output
}

pub fn build_neg_idx_table(length: usize) -> Vec<Vec<usize>> {
    let mut output = vec![];
    for i in 1..length {
        let mut line = vec![];
        for j in i..length {
            let ix = j % (i * 4);
            if ix >= 3 * i {
                line.push(j);
            }
        }
        output.push(line);
    }

    output
}

pub fn phase2(input: &Vec<i32>, n_inputs: usize) -> Vec<i32> {

    let input_len = input.len();

    // Build index tables
    let ptable = build_pos_idx_table(input_len * n_inputs);
    let ntable = build_neg_idx_table(input_len * n_inputs);

    let mut output = Vec::with_capacity(n_inputs * input_len);

    for (pidxs, nidxs) in ptable.iter().zip(ntable) {
        let mut result = 0;
        for i in pidxs {
            result += input[i % input_len];
        }
        for i in nidxs {
            result -= input[i % input_len];
        }
        result = result.abs() % 10;
        output.push(result);
    }

    output
}

pub fn phase(input: &Vec<i32>, n_inputs: usize) -> Vec<i32> {
    let base_pattern = vec![0, 1, 0, -1];

    let mut output = Vec::with_capacity(n_inputs * input.len());
    for i in 0..n_inputs * input.len() {
        let mut result = 0;

        for j in i..n_inputs * input.len() {
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
    let mut step2 = input.clone();

    for _ in 0..100 {
        step = phase(&step, 1);
        step2 = phase2(&step2, 1);
    }

    println!("{:?}", &step[0..10]);
    println!("{:?}", &step2[0..10]);

    step.iter()
        .take(8)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("")
}

pub fn part2(input: &Vec<i32>) -> String {
    let mut step = input.clone();
    println!("input.len(): {}", input.len());

    step = phase(&step, 2);

    for i in 1..100 {
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
        let bp = BasePattern::new(5, 21);
        for i in bp {
            println!("{}", i);
        }
        assert_eq!(0, 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}
