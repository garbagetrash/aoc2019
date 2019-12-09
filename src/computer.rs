extern crate regex;
use regex::Regex;

pub enum ProgramStatus {
    Running,
    Halted,
}

pub struct ProgramState {
    pub memory: Vec<i64>,
    pub ic: usize,
    pub relative_base: i64,
    pub status: ProgramStatus,
}

impl ProgramState {
    pub fn new(memory: &Vec<i64>) -> ProgramState {
        let mut copy = memory.clone();
        copy.extend_from_slice(&vec![0; 10000].as_slice());
        ProgramState {
            memory: copy,
            ic: 0,
            relative_base: 0,
            status: ProgramStatus::Running,
        }
    }

    pub fn write(&mut self, value: i64, position: usize) {
        if position > self.memory.len() {
            let needs = position - self.memory.len() + 1;
            self.memory.extend_from_slice(&vec![0; needs].as_slice());
        }
        self.memory[position] = value;
    }
}

pub fn param_parse(param: i64, mode: u32, state: &ProgramState) -> i64 {
    let output = {
        match mode {
            0 => {
                if param < 0 {
                    panic!("Cannot dereference negative address: {}", param);
                } else {
                    state.memory[param as usize]
                }
            }
            1 => param,
            2 => {
                if (param + state.relative_base) < 0 {
                    panic!(
                        "Cannot dereference negative address: {}",
                        param + state.relative_base
                    );
                } else {
                    state.memory[(param + state.relative_base) as usize]
                }
            }
            _ => panic!("Unrecognized mode"),
        }
    };
    output
}

pub fn addr_parse(param: i64, mode: u32, state: &ProgramState) -> usize {
    let output = {
        match mode {
            0 => param as usize,
            2 => (param + state.relative_base) as usize,
            _ => panic!("Unrecognized mode"),
        }
    };
    output
}

pub fn run(input: i64, state: &mut ProgramState) -> Option<i64> {
    let re = Regex::new(r"^(\d*?)(\d{1,2})$").unwrap();

    let mut input_consumed = false;

    loop {
        let opcode = state.memory[state.ic];
        // Parse opcode for mode using regex
        let opstr = opcode.to_string();
        let cap = re.captures(&opstr[..]).unwrap();
        let opcode = cap[cap.len() - 1].parse::<i64>().unwrap();
        let mut modes: Vec<u32> = cap[1]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .collect();

        match opcode {
            1 => {
                // Add
                let n_params = 3;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let params =
                    &state.memory[state.ic + 1..state.ic + 1 + n_params];

                let a = param_parse(params[0], modes[0], state);
                let b = param_parse(params[1], modes[1], state);
                let addr = addr_parse(params[2], modes[2], state);

                state.write(a + b, addr);
                state.ic += n_params + 1;
            }
            2 => {
                // Multiply
                let n_params = 3;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let params =
                    &state.memory[state.ic + 1..state.ic + 1 + n_params];

                let a = param_parse(params[0], modes[0], state);
                let b = param_parse(params[1], modes[1], state);
                let addr = addr_parse(params[2], modes[2], state);

                state.write(a * b, addr);
                state.ic += n_params + 1;
            }
            3 => {
                // Store input in memory
                if !input_consumed {
                    let n_params = 1;
                    let implicit_zeros = n_params - modes.len();
                    modes
                        .extend_from_slice(&vec![0; implicit_zeros].as_slice());
                    let param = state.memory[state.ic + 1];

                    let addr = addr_parse(param, modes[0], state);

                    state.write(input, addr);
                    state.ic += n_params + 1;
                    input_consumed = true;
                } else {
                    return None;
                }
            }
            4 => {
                // Load output from memory, return output
                let n_params = 1;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let param = state.memory[state.ic + 1];

                let output = param_parse(param, modes[0], state);
                state.ic += n_params + 1;

                return Some(output);
            }
            5 => {
                // Jump if true
                let n_params = 2;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let params =
                    &state.memory[state.ic + 1..state.ic + 1 + n_params];

                let p1 = param_parse(params[0], modes[0], state);
                let p2 = param_parse(params[1], modes[1], state);

                if p1 != 0 {
                    state.ic = p2 as usize;
                } else {
                    state.ic += n_params + 1;
                }
            }
            6 => {
                // Jump if false
                let n_params = 2;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let params =
                    &state.memory[state.ic + 1..state.ic + 1 + n_params];

                let p1 = param_parse(params[0], modes[0], state);
                let p2 = param_parse(params[1], modes[1], state);

                if p1 == 0 {
                    state.ic = p2 as usize;
                } else {
                    state.ic += n_params + 1;
                }
            }
            7 => {
                // Less than
                let n_params = 3;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let params =
                    &state.memory[state.ic + 1..state.ic + 1 + n_params];

                let p1 = param_parse(params[0], modes[0], state);
                let p2 = param_parse(params[1], modes[1], state);
                let addr = addr_parse(params[2], modes[2], state);

                if p1 < p2 {
                    state.memory[addr] = 1;
                } else {
                    state.memory[addr] = 0;
                }
                state.ic += n_params + 1;
            }
            8 => {
                // Equals
                let n_params = 3;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let params =
                    &state.memory[state.ic + 1..state.ic + 1 + n_params];

                let p1 = param_parse(params[0], modes[0], state);
                let p2 = param_parse(params[1], modes[1], state);
                let addr = addr_parse(params[2], modes[2], state);

                if p1 == p2 {
                    state.memory[addr] = 1;
                } else {
                    state.memory[addr] = 0;
                }
                state.ic += n_params + 1;
            }
            9 => {
                // Relative base offset
                let n_params = 1;
                let implicit_zeros = n_params - modes.len();
                modes.extend_from_slice(&vec![0; implicit_zeros].as_slice());
                let param = state.memory[state.ic + 1];

                let p1 = param_parse(param, modes[0], state);

                state.relative_base += p1;
                state.ic += n_params + 1;
            }
            99 => {
                // Halt the program
                state.status = ProgramStatus::Halted;
                return None;
            }
            _ => panic!("Unknown opcode: {:?}", opcode),
        }
    }
}
