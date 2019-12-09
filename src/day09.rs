use std::fs::File;
use std::io::prelude::*;

use crate::computer::{run, ProgramState, ProgramStatus};

pub fn load_input() -> Vec<i64> {
    let mut f = File::open("inputs/09.txt").unwrap();
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

pub fn part1(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(&input);
    let mut value = 0;
    loop {
        let temp = run(1, &mut state);
        match state.status {
            ProgramStatus::Halted => break,
            ProgramStatus::Running => value = temp.unwrap(),
        }
    }
    value
}

pub fn part2(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(&input);
    let mut value = 0;
    loop {
        let temp = run(2, &mut state);
        match state.status {
            ProgramStatus::Halted => break,
            ProgramStatus::Running => value = temp.unwrap(),
        }
    }
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        // Outputs a copy of itself
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101,
            0, 99,
        ];
        let mut state = ProgramState::new(&input);
        let mut output = vec![];
        loop {
            match state.status {
                ProgramStatus::Running => {
                    if let Some(x) = run(0, &mut state) {
                        output.push(x);
                    }
                }
                ProgramStatus::Halted => break,
            }
        }

        // Outputs a 16 digit number
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut state = ProgramState::new(&input);
        let output = run(0, &mut state).unwrap();
        let outlen = output.to_string().chars().collect::<Vec<_>>().len();
        assert_eq!(outlen, 16);

        // Outputs the large center number
        let input = vec![104, 1125899906842624, 99];
        let mut state = ProgramState::new(&input);
        let output = run(0, &mut state).unwrap();
        assert_eq!(output, input[1]);
    }
}
