extern crate regex;

use itertools::Itertools;
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

pub fn part1(input: &Vec<i64>) -> usize {
    let mut state = ProgramState::new(input);
    let mut screen = HashMap::new();
    let mut output = vec![];
    loop {
        if let Some(out) = run(0, &mut state) {
            output.push(out);
        } else {
            break;
        }
    }

    for chunk in output.chunks(3) {
        screen.insert((chunk[0], chunk[1]), chunk[2]);
    }

    let mut cntr = 0;
    for (key, value) in screen {
        if value == 2 {
            cntr += 1;
        }
    }
    cntr
}

pub fn render_screen(screen: &HashMap<(i32, i32), i32>) {

}
pub fn part2(input: &Vec<i64>) -> u64 {
    let mut state = ProgramState::new(input);
    state.memory[0] = 2;
    let mut screen = HashMap::new();
    let mut score = 0;
    let mut ballx = 0;
    let mut dir = 0;
    let mut paddlex = 0;
    loop {
        let mut inst = Vec::with_capacity(3);

        for ((x, y), id) in &screen {
            if *id == 3 {
                paddlex = *x;
            } else if *id == 4 {
                ballx = *x;
            }
        }

        if ballx > paddlex {
            dir = 1;
        } else if ballx < paddlex {
            dir = -1;
        } else {
            dir = 0;
        }

        // Grab next instruction
        if let Some(out) = run(dir, &mut state) {
            inst.push(out);
        } else {
            // Halted
            break;
        }
        if let Some(out) = run(dir, &mut state) {
            inst.push(out);
        }
        if let Some(out) = run(dir, &mut state) {
            inst.push(out);
        }

        if inst[0] == -1 && inst[1] == 0 {
            // Update score
            score = inst[2];
        } else {
            screen.insert((inst[0], inst[1]), inst[2]);
        }
    }
    score as u64
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
