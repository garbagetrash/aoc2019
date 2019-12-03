use std::io::prelude::*;
use std::fs::File;


pub fn load_input() -> Vec<u64> {
    let mut f = File::open("inputs/02.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let mut output = Vec::new();
    for el in buffer.split(",") {
        if let Ok(x) = el.parse::<u64>() {
            output.push(x)
        }
    }
    output
}

pub fn operation(input: &Vec<u64>) -> Vec<u64> {
    let mut input_copy = input.clone();
    let mut ic = 0;
    loop {
        let opcode = input_copy[ic];
        match opcode {
            1 => {
                let a = input_copy[ic + 1] as usize;
                let b = input_copy[ic + 2] as usize;
                let c = input_copy[ic + 3] as usize;
                let temp = input_copy[a] + input_copy[b];
                input_copy[c] = temp;
            }
            2 => {
                let a = input_copy[ic + 1] as usize;
                let b = input_copy[ic + 2] as usize;
                let c = input_copy[ic + 3] as usize;
                let temp = input_copy[a] * input_copy[b];
                input_copy[c] = temp;
            }
            99 => {
                return input_copy;
            }
            _ => println!("This really shouldn't happen"),
        }

        ic += 4;
    }
}

pub fn program(noun: u64, verb: u64) -> u64 {
    let mut input = load_input();
    input[1] = noun;
    input[2] = verb;
    let output = operation(&input);

    output[0]
}

pub fn part1(input: &Vec<u64>) -> u64 {
    let mut input_copy = input.clone();

    // Set input_copy[1] = 12
    input_copy[1] = 12;

    // Set input_copy[2] = 2
    input_copy[2] = 2;

    let output = operation(&input_copy);

    output[0]
}

pub fn part2(_input: &Vec<u64>) -> u64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let output = program(noun, verb);
            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![1, 0, 0, 0, 99];
        let expected: Vec<u64> = vec![2, 0, 0, 0, 99];
        assert_eq!(operation(&input), expected);

        let input = vec![2, 3, 0, 3, 99];
        let expected: Vec<u64> = vec![2, 3, 0, 6, 99];
        assert_eq!(operation(&input), expected);

        let input = vec![2, 4, 4, 5, 99, 0];
        let expected: Vec<u64> = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(operation(&input), expected);

        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected: Vec<u64> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(operation(&input), expected);

        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected: Vec<u64> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(operation(&input), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(12 * 100 + 2, 1202);
    }
}
