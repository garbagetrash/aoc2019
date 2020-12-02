extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

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

pub struct PosPattern {
    rows: usize,
    row: usize,
    idx: usize,
    ixmod: usize,
    check: usize,
}

impl PosPattern {
    pub fn new(rows: usize, row: usize) -> PosPattern {
        PosPattern {
            rows,
            row,
            idx: row,
            ixmod: 4 * (row + 1),
            check: 2 * (row + 1) - 1,
        }
    }
}

impl Iterator for PosPattern {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.idx >= self.rows {
                break;
            }

            let ix = self.idx % self.ixmod;
            if ix >= self.row && ix < self.check {
                self.idx += 1;
                return Some(self.idx - 1);
            } else {
                self.idx += self.ixmod;
            }
        }

        None
    }
}

pub struct NegPattern {
    rows: usize,
    row: usize,
    idx: usize,
    ixmod: usize,
    check: usize,
}

impl NegPattern {
    pub fn new(rows: usize, row: usize) -> NegPattern {
        NegPattern {
            rows,
            row,
            idx: row,
            ixmod: 4 * (row + 1),
            check: 3 * (row + 1),
        }
    }
}

impl Iterator for NegPattern {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.idx >= self.rows {
                break;
            }

            let ix = (self.idx + 1) % self.ixmod;
            if ix >= self.check {
                self.idx += 1;
                return Some(self.idx - 1);
            } else {
                self.idx += self.ixmod;
            }
        }

        None
    }
}

pub fn phase2(
    input: &Vec<i32>,
    size: usize,
    offset: usize,
) -> Vec<i32> {
    let input_len = input.len();
    println!("input_len: {}", input_len);

    let mut output = Vec::with_capacity(input_len);
    output = (offset..size).collect::<Vec<usize>>().par_iter().map(|i| {

        // Prepare some iterators to grab the indexes we need
        let pidxs = PosPattern::new(size, *i);
        let nidxs = NegPattern::new(size, *i);

        // Sum the values at the indexes
        let r1: i32 = pidxs.map(|i|  input[i - offset]).sum();
        let r2: i32 = nidxs.map(|i| -input[i - offset]).sum();

        (r1 + r2).abs() % 10
    }).collect();

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

    let offset: usize = input
        .iter()
        .take(7)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    let n_inputs = 10000;
    let size = input.len() * n_inputs;
    let mut step: Vec<i32> = input.iter().cycle().take(size).map(|x| *x).collect();
    println!("{:?}", &step[..10]);
    step = step[offset..].to_vec();

    for i in 0..100 {
        println!("i: {}", i);
        step = phase2(&step, size, offset);
        println!("{:?}", &step[..10]);
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
        assert_eq!(0, 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}
