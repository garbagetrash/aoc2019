use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

use crate::computer::{run, ProgramState};

pub fn load_input() -> Vec<i64> {
    let mut f = File::open("inputs/07.txt").unwrap();
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
    let phase_perm = (0..5).permutations(5);

    let mut max_signal = std::i64::MIN;
    for phases in phase_perm {
        // Initialize amp program states
        let mut amp_states = Vec::with_capacity(5);
        for _ in 0..5 {
            amp_states.push(ProgramState {
                memory: input.clone(),
                ic: 0,
                done: false,
            });
        }

        // Run each amp
        let mut invalue = 0;
        for i in 0..5 {
            run(phases[i], &mut (amp_states[i]));
            invalue = run(invalue, &mut (amp_states[i]));
        }

        if invalue > max_signal {
            max_signal = invalue;
        }
    }
    max_signal
}

pub fn part2(input: &Vec<i64>) -> i64 {
    let phase_perm = (5..10).permutations(5);
    let mut max_signal = std::i64::MIN;

    for phases in phase_perm {
        let mut amp_states = Vec::with_capacity(5);
        for _ in 0..5 {
            amp_states.push(ProgramState {
                memory: input.clone(),
                ic: 0,
                done: false,
            });
        }

        // Initialize
        for i in 0..5 {
            run(phases[i], &mut (amp_states[i]));
        }

        let mut temp = 0;

        // Loop over feedback
        loop {
            let mut break_now = false;
            for i in 0..5 {
                let output = run(temp, &mut (amp_states[i]));
                if amp_states[i].done {
                    break_now = true;
                    break;
                } else {
                    temp = output;
                }
            }

            if break_now {
                break;
            }
        }

        if temp > max_signal {
            max_signal = temp;
        }
    }
    max_signal
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(part1(&input), 43210);

        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23,
            1, 24, 23, 23, 4, 23, 99, 0, 0,
        ];
        assert_eq!(part1(&input), 54321);

        let input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
            1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(part1(&input), 65210);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4,
            27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(part2(&input), 139629729);

        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55,
            1005, 55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008,
            54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56,
            1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(part2(&input), 18216);
    }
}
