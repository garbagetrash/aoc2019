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
                    let state = ProgramState {
                        memory: input_copy,
                        ic: ic,
                        done: false,
                    };
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

                let state = ProgramState {
                    memory: input_copy,
                    ic: ic,
                    done: false,
                };
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
                let out_state = ProgramState {
                    memory: input_copy,
                    ic: ic,
                    done: true,
                };
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
        let state_a = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_b = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_c = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_d = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_e = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };

        // A
        let (_, state_a) = run(phases[0], &state_a);
        let (invalue, _state_a) = run(0, &state_a);

        // B
        let (_, state_b) = run(phases[1], &state_b);
        let (invalue, _state_b) = run(invalue, &state_b);

        // C
        let (_, state_c) = run(phases[2], &state_c);
        let (invalue, _state_c) = run(invalue, &state_c);

        // D
        let (_, state_d) = run(phases[3], &state_d);
        let (invalue, _state_d) = run(invalue, &state_d);

        // E
        let (_, state_e) = run(phases[4], &state_e);
        let (signal, _state_e) = run(invalue, &state_e);

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
        let state_a = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_b = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_c = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_d = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };
        let state_e = ProgramState {
            memory: input.clone(),
            ic: 0,
            done: false,
        };

        // Initialize
        let (_, mut state_a) = run(phases[0], &state_a);
        let (_, mut state_b) = run(phases[1], &state_b);
        let (_, mut state_c) = run(phases[2], &state_c);
        let (_, mut state_d) = run(phases[3], &state_d);
        let (_, mut state_e) = run(phases[4], &state_e);

        let mut temp = 0;

        // Loop over feedback
        loop {
            // A
            let (invalue, state_aa) = run(temp, &state_a);
            if state_aa.done {
                break;
            }
            temp = invalue;
            state_a = state_aa;

            // B
            let (invalue, state_ba) = run(temp, &state_b);
            temp = invalue;
            state_b = state_ba;

            // C
            let (invalue, state_ca) = run(temp, &state_c);
            temp = invalue;
            state_c = state_ca;

            // D
            let (invalue, state_da) = run(temp, &state_d);
            temp = invalue;
            state_d = state_da;

            // E
            let (invalue, state_ea) = run(temp, &state_e);
            temp = invalue;
            state_e = state_ea;
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
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(part1(&input), 54321);

        let input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(part1(&input), 65210);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(part2(&input), 139629729);

        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(part2(&input), 18216);
    }
}
