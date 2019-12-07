extern crate regex;
use regex::Regex;

pub struct ProgramState {
    pub memory: Vec<i64>,
    pub ic: usize,
    pub done: bool,
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

pub fn run(input: i64, state: &mut ProgramState) -> i64 {
    let re = Regex::new(r"^(\d*?)(\d{1,2})$").unwrap();

    let mut input_consumed = false;

    loop {
        let opcode = state.memory[state.ic];
        // Parse opcode for mode using regex
        let opstr = opcode.to_string();
        let cap = re.captures(&opstr[..]).unwrap();
        let opcode = cap[cap.len() - 1].parse::<i64>().unwrap();
        let imarr: Vec<u32> =
            cap[1].chars().map(|c| c.to_digit(10).unwrap()).collect();

        match opcode {
            1 => {
                // Add
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(
                    state.memory[state.ic + 1],
                    mode,
                    &mut state.memory,
                );

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let value_b = parse_param(
                    state.memory[state.ic + 2],
                    mode,
                    &mut state.memory,
                );

                // Destination mode must always be position
                let c = state.memory[state.ic + 3] as usize;
                let result = value_a + value_b;
                state.memory[c] = result;
                state.ic += 4;
            }
            2 => {
                // Multiply
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(
                    state.memory[state.ic + 1],
                    mode,
                    &mut state.memory,
                );

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let value_b = parse_param(
                    state.memory[state.ic + 2],
                    mode,
                    &mut state.memory,
                );

                // Destination mode must always be position
                let c = state.memory[state.ic + 3] as usize;
                let result = value_a * value_b;
                state.memory[c] = result;
                state.ic += 4;
            }
            3 => {
                // Store input in memory
                if !input_consumed {
                    let a = state.memory[state.ic + 1] as usize;
                    state.memory[a] = input;
                    state.ic += 2;
                    input_consumed = true;
                } else {
                    return 0;
                }
            }
            4 => {
                // Load output from memory
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(
                    state.memory[state.ic + 1],
                    mode,
                    &mut state.memory,
                );
                let output = value_a;
                state.ic += 2;

                return output;
            }
            5 => {
                // Jump if true
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(
                    state.memory[state.ic + 1],
                    mode,
                    &mut state.memory,
                );

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(
                    state.memory[state.ic + 2],
                    mode,
                    &mut state.memory,
                ) as usize;

                if p1 != 0 {
                    state.ic = p2;
                } else {
                    state.ic += 3;
                }
            }
            6 => {
                // Jump if false
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(
                    state.memory[state.ic + 1],
                    mode,
                    &mut state.memory,
                );

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(
                    state.memory[state.ic + 2],
                    mode,
                    &mut state.memory,
                ) as usize;

                if p1 == 0 {
                    state.ic = p2;
                } else {
                    state.ic += 3;
                }
            }
            7 => {
                // Less than
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(
                    state.memory[state.ic + 1],
                    mode,
                    &mut state.memory,
                );

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(
                    state.memory[state.ic + 2],
                    mode,
                    &mut state.memory,
                );

                let p3 = state.memory[state.ic + 3] as usize;

                if p1 < p2 {
                    state.memory[p3] = 1;
                } else {
                    state.memory[p3] = 0;
                }
                state.ic += 4;
            }
            8 => {
                // Equals
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(
                    state.memory[state.ic + 1],
                    mode,
                    &mut state.memory,
                );

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(
                    state.memory[state.ic + 2],
                    mode,
                    &mut state.memory,
                );

                let p3 = state.memory[state.ic + 3] as usize;

                if p1 == p2 {
                    state.memory[p3] = 1;
                } else {
                    state.memory[p3] = 0;
                }
                state.ic += 4;
            }
            99 => {
                state.done = true;
                return 0;
            }
            _ => panic!("Unknown opcode: {:?}", opcode),
        }
    }
}
