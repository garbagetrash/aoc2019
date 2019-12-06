use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

pub fn load_input() -> Vec<i64> {
    let mut f = File::open("inputs/05.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let mut output = Vec::new();
    for el in buffer.split(",") {
        if let Ok(x) = el.parse::<i64>() {
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

pub fn run(input: i64, memory: &Vec<i64>) -> i64 {
    let mut input_copy = memory.clone();
    let mut ic = 0;
    let re = Regex::new(r"^(\d*?)(\d{1,2})$").unwrap();

    let mut output = 0;

    loop {
        let opcode = input_copy[ic];
        // Parse opcode for mode using regex
        let opstr = opcode.to_string();
        let cap = re.captures(&opstr[..]).unwrap();
        let opcode = cap[cap.len() - 1].parse::<i64>().unwrap();
        let imarr: Vec<u32> = cap[1].chars().map(|c| c.to_digit(10).unwrap()).collect();

        match opcode {
            1 => {
                println!(
                    "Add - {}, {}, {}, {}",
                    input_copy[ic],
                    input_copy[ic + 1],
                    input_copy[ic + 2],
                    input_copy[ic + 3]
                );
                // Add
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                println!("A mode: {}", mode);
                let value_a = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                println!("A value: {}", value_a);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                println!("B mode: {}", mode);
                let value_b = parse_param(input_copy[ic + 2], mode, &mut input_copy);
                println!("B value: {}", value_b);

                // Destination mode must always be position
                let c = input_copy[ic + 3] as usize;
                let result = value_a + value_b;
                println!("A + B = {}", result);
                input_copy[c] = result;
                ic += 4;
            }
            2 => {
                println!(
                    "Multiply - {}, {}, {}, {}",
                    input_copy[ic],
                    input_copy[ic + 1],
                    input_copy[ic + 2],
                    input_copy[ic + 3]
                );
                // Multiply
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                println!("a: {}", value_a);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let value_b = parse_param(input_copy[ic + 2], mode, &mut input_copy);
                println!("b: {}", value_b);

                // Destination mode must always be position
                let c = input_copy[ic + 3] as usize;
                let result = value_a * value_b;
                println!("A * B = {}", result);
                input_copy[c] = result;
                ic += 4;
            }
            3 => {
                println!("Store - {}, {}", input_copy[ic], input_copy[ic + 1]);
                // Store input in memory
                let a = input_copy[ic + 1] as usize;
                println!("ia: {}", a);
                input_copy[a] = input;
                ic += 2;
            }
            4 => {
                println!("Load - {}, {}", input_copy[ic], input_copy[ic + 1]);
                // Load output from memory
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let value_a = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                println!("a: {}", value_a);
                output = value_a;
                println!("\noutput: {}\n", output);
                ic += 2;
            }
            5 => {
                println!(
                    "Jump if True - {}, {}, {}",
                    input_copy[ic],
                    input_copy[ic + 1],
                    input_copy[ic + 2]
                );
                // Jump if true
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                println!("p1: {}", p1);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy) as usize;
                println!("p2: {}", p2);

                if p1 != 0 {
                    println!("Jump");
                    ic = p2;
                } else {
                    println!("Don't Jump");
                    ic += 3;
                }
            }
            6 => {
                println!(
                    "Jump if False - {}, {}, {}",
                    input_copy[ic],
                    input_copy[ic + 1],
                    input_copy[ic + 2]
                );
                // Jump if false
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                println!("p1: {}", p1);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy) as usize;
                println!("p2: {}", p2);

                if p1 == 0 {
                    println!("Jump");
                    ic = p2;
                } else {
                    println!("Don't Jump");
                    ic += 3;
                }
            }
            7 => {
                println!(
                    "Less Than - {}, {}, {}, {}",
                    input_copy[ic],
                    input_copy[ic + 1],
                    input_copy[ic + 2],
                    input_copy[ic + 3]
                );
                // Less than
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                println!("p1: {}", p1);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy);
                println!("p2: {}", p2);

                let p3 = input_copy[ic + 3] as usize;
                println!("p3: {}", p3);

                if p1 < p2 {
                    input_copy[p3] = 1;
                } else {
                    input_copy[p3] = 0;
                }
                ic += 4;
            }
            8 => {
                println!(
                    "Equals - {}, {}, {}, {}",
                    input_copy[ic],
                    input_copy[ic + 1],
                    input_copy[ic + 2],
                    input_copy[ic + 3]
                );
                // Equals
                let mut mode = 0;
                if imarr.len() > 0 {
                    mode = imarr[imarr.len() - 1];
                }
                let p1 = parse_param(input_copy[ic + 1], mode, &mut input_copy);
                println!("p1: {}", p1);

                mode = 0;
                if imarr.len() > 1 {
                    mode = imarr[imarr.len() - 2];
                }
                let p2 = parse_param(input_copy[ic + 2], mode, &mut input_copy);
                println!("p2: {}", p2);

                let p3 = input_copy[ic + 3] as usize;
                println!("p3: {}", p3);

                if p1 == p2 {
                    input_copy[p3] = 1;
                } else {
                    input_copy[p3] = 0;
                }
                ic += 4;
            }
            99 => {
                println!("Final output: {}", output);
                return output;
            }
            _ => panic!("Unknown opcode: {:?}", opcode),
        }
    }
}

pub fn part1(input: &Vec<i64>) -> i64 {
    run(1, input)
}

pub fn part2(input: &Vec<i64>) -> i64 {
    run(5, input)
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
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(8, &input), 1);
        assert_eq!(run(9, &input), 0);
        assert_eq!(run(7, &input), 0);

        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(8, &input), 0);
        assert_eq!(run(9, &input), 0);
        assert_eq!(run(7, &input), 1);

        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(8, &input), 1);
        assert_eq!(run(9, &input), 0);
        assert_eq!(run(7, &input), 0);

        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(8, &input), 0);
        assert_eq!(run(9, &input), 0);
        assert_eq!(run(7, &input), 1);

        let input = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run(0, &input), 0);
        assert_eq!(run(9, &input), 1);
        assert_eq!(run(7, &input), 1);

        let input = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(run(0, &input), 0);
        assert_eq!(run(9, &input), 1);
        assert_eq!(run(7, &input), 1);

        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(run(8, &input), 1000);
        assert_eq!(run(9, &input), 1001);
        assert_eq!(run(7, &input), 999);
    }
}
