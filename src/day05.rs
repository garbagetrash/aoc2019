use std::fs::File;
use std::io::prelude::*;

use crate::computer::{run, ProgramState};

pub fn load_input() -> Vec<i64> {
    let mut f = File::open("inputs/05.txt").unwrap();
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
    let mut state = ProgramState::new(input);
    let mut output = Some(0);

    while output == Some(0) {
        output = run(1, &mut state);
    }
    output.expect("No output returned!")
}

pub fn part2(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(input);
    run(5, &mut state).expect("No output returned!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut state = ProgramState::new(&input);
        assert_eq!(run(8, &mut state), Some(1));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(9, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(7, &mut state), Some(0));

        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut state = ProgramState::new(&input);
        assert_eq!(run(8, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(9, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(7, &mut state), Some(1));

        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let mut state = ProgramState::new(&input);
        assert_eq!(run(8, &mut state), Some(1));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(9, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(7, &mut state), Some(0));

        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let mut state = ProgramState::new(&input);
        assert_eq!(run(8, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(9, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(7, &mut state), Some(1));

        let input =
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let mut state = ProgramState::new(&input);
        assert_eq!(run(0, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(9, &mut state), Some(1));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(7, &mut state), Some(1));

        let input = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let mut state = ProgramState::new(&input);
        assert_eq!(run(0, &mut state), Some(0));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(9, &mut state), Some(1));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(7, &mut state), Some(1));

        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];

        let mut state = ProgramState::new(&input);
        assert_eq!(run(8, &mut state), Some(1000));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(9, &mut state), Some(1001));

        let mut state = ProgramState::new(&input);
        assert_eq!(run(7, &mut state), Some(999));
    }
}
