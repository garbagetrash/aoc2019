use itertools::Itertools;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

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

pub fn parse_param(param: i64, mode: u32, memory: &mut Vec<i64>) -> i64 {
    match mode {
        0 => {
            if param < 0 {
                panic!("Addresses can't be negative!");
            } else {
                memory[param as usize]
            }
        }
        1 => param,
        _ => {
            panic!("Invalid mode!");
        }
    }
}

pub struct ProgramState {
    memory: Vec<i64>,
    ic: usize,
    done: bool,
}

pub fn run(input: i64, state: &ProgramState) -> (i64, ProgramState) {
    let mut input_copy = state.memory.clone();
    let mut ic = state.ic;
    let re = Regex::new(r"^(\d*?)(\d{1,2})$").unwrap();

    let mut input_consumed = false;

    loop {
        let opcode = input_copy[ic];
        // Parse opcode for mode using regex
        let opstr = opcode.to_string();
        let cap = re.captures(&opstr[..]).unwrap();
        let opcode = cap[cap.len() - 1].parse::<i64>().unwrap();
        let imarr: Vec<u32> = cap[1].chars().map(|c| c.to_digit(10).unwrap()).collect();

        match opcode {
            1 => {
                // Add
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(input_copy[ic + 1], mode, &mut input_copy);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let value_b = parse_param(input_copy[ic + 2], mode, &mut input_copy);

                // Destination mode must always be position
                let c = input_copy[ic + 3] as usize;
                let result = value_a + value_b;
                input_copy[c] = result;
                ic += 4;
            }
            2 => {
                // Multiply
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(input_copy[ic + 1], mode, &mut input_copy);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let value_b = parse_param(input_copy[ic + 2], mode, &mut input_copy);

                // Destination mode must always be position
                let c = input_copy[ic + 3] as usize;
                let result = value_a * value_b;
                input_copy[c] = result;
                ic += 4;
            }
            3 => {
                // Store input in memory
                if !input_consumed {
                    let a = input_copy[ic + 1] as usize;
                    input_copy[a] = input;
                    ic += 2;
                    input_consumed = true;
                } else {
                    let state = ProgramState { memory: input_copy, ic: ic, done: false };
                    return (0, state);
                }
            }
            4 => {
                // Load output from memory
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                let output = value_a;
                ic += 2;

                let state = ProgramState { memory: input_copy, ic: ic, done: false };
                return (output, state);
            }
            5 => {
                // Jump if true
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy) as usize;

                if p1 != 0 {
                    ic = p2;
                } else {
                    ic += 3;
                }
            }
            6 => {
                // Jump if false
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy) as usize;

                if p1 == 0 {
                    ic = p2;
                } else {
                    ic += 3;
                }
            }
            7 => {
                // Less than
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy);

                let p3 = input_copy[ic + 3] as usize;

                if p1 < p2 {
                    input_copy[p3] = 1;
                } else {
                    input_copy[p3] = 0;
                }
                ic += 4;
            }
            8 => {
                // Equals
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy);

                let p3 = input_copy[ic + 3] as usize;

                if p1 == p2 {
                    input_copy[p3] = 1;
                } else {
                    input_copy[p3] = 0;
                }
                ic += 4;
            }
            99 => {
                let out_state = ProgramState { memory: input_copy, ic: ic, done: true };
                return (0, out_state);
            }
            _ => panic!("Unknown opcode: {:?}", opcode),
        }
    }
}

pub fn part1(input: &Vec<i64>) -> i64 {

    let phase_perm = (0..5).permutations(5);

    let mut max_signal = std::i64::MIN;
    for phases in phase_perm {

        let stateA = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateB = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateC = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateD = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateE = ProgramState { memory: input.clone(), ic: 0, done: false };

        // A
        let (_, stateA) = run(phases[0], &stateA);
        let (invalue, _stateA) = run(0, &stateA);

        // B
        let (_, stateB) = run(phases[1], &stateB);
        let (invalue, _stateB) = run(invalue, &stateB);

        // C
        let (_, stateC) = run(phases[2], &stateC);
        let (invalue, _stateC) = run(invalue, &stateC);

        // D
        let (_, stateD) = run(phases[3], &stateD);
        let (invalue, _stateD) = run(invalue, &stateD);

        // E
        let (_, stateE) = run(phases[4], &stateE);
        let (signal, _stateE) = run(invalue, &stateE);

        if signal > max_signal {
            max_signal = signal;
        }
    }
    max_signal
}

pub fn part2(input: &Vec<i64>) -> i64 {

    let phase_perm = (5..10).permutations(5);
    let mut max_signal = std::i64::MIN;

    for phases in phase_perm {
        let stateA = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateB = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateC = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateD = ProgramState { memory: input.clone(), ic: 0, done: false };
        let stateE = ProgramState { memory: input.clone(), ic: 0, done: false };

        // Initialize
        let (_, mut stateA) = run(phases[0], &stateA);
        let (_, mut stateB) = run(phases[1], &stateB);
        let (_, mut stateC) = run(phases[2], &stateC);
        let (_, mut stateD) = run(phases[3], &stateD);
        let (_, mut stateE) = run(phases[4], &stateE);

        let mut temp = 0;

        // Loop over feedback
        loop {

            // A
            let (invalue, stateAa) = run(temp, &stateA);
            if stateAa.done {
                break;
            }
            temp = invalue;
            stateA = stateAa;

            // B
            let (invalue, stateBa) = run(temp, &stateB);
            temp = invalue;
            stateB = stateBa;

            // C
            let (invalue, stateCa) = run(temp, &stateC);
            temp = invalue;
            stateC = stateCa;

            // D
            let (invalue, stateDa) = run(temp, &stateD);
            temp = invalue;
            stateD = stateDa;

            // E
            let (invalue, stateEa) = run(temp, &stateE);
            temp = invalue;
            stateE = stateEa;
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
        assert_eq!(0, 0);
    }

    #[test]
    fn test_part2() {
        let input = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(part2(&input), 139629729);
    }
}
