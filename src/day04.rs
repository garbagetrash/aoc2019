extern crate regex;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
pub struct Wires {}

pub fn load_input() -> Vec<String> {
    let mut f = File::open("inputs/04.txt").unwrap();
    let mut output = String::new();
    f.read_to_string(&mut output).unwrap();
    vec![output]
}

pub fn parse_input(input: &Vec<String>) -> (u32, u32) {
    let re = Regex::new(r"^(\d+)-(\d+)").unwrap();
    let cap = re.captures(input[0].as_str()).unwrap();
    let lower_bound = cap[1].parse::<u32>().unwrap();
    let upper_bound = cap[2].parse::<u32>().unwrap();
    (lower_bound, upper_bound)
}

pub fn check1(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut double_digit = false;
    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            double_digit = true;
            break;
        }
    }

    if double_digit {
        for i in 0..5 {
            if digits[i] > digits[i + 1] {
                // If not monotonic increasing, return false
                return false;
            }
        }

        // If monotonic increasing and double_digit is set, return true
        return true;
    }

    // If double_digit is not set, return false
    false
}

pub fn check2(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut double_digit = [false; 5];
    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            double_digit[i] = true;
        }
    }

    let mut triple_digit = [false; 4];
    for i in 0..4 {
        if digits[i] == digits[i + 1] && digits[i + 1] == digits[i + 2] {
            triple_digit[i] = true;
        }
    }

    let mut valid_double = false;
    for i in 0..5 {
        if double_digit[i] {
            if i == 0 {
                if !triple_digit[i] {
                    valid_double = true;
                    break;
                }
            } else if i == 4 {
                if !triple_digit[i - 1] {
                    valid_double = true;
                    break;
                }
            } else {
                if !triple_digit[i - 1] && !triple_digit[i] {
                    valid_double = true;
                    break;
                }
            }
        }
    }

    if valid_double {
        for i in 0..5 {
            if digits[i] > digits[i + 1] {
                // If not monotonic increasing, return false
                return false;
            }
        }
        // If valid_double is set and monotonic increasing return true
        return true;
    }

    // If valid_double is not set, return false
    false
}

pub fn part1(input: &Vec<String>) -> u64 {
    let (lower, upper) = parse_input(input);

    let mut count = 0;
    for m in 0..10 {
        for ht in m..10 {
            for tt in ht..10 {
                for t in tt..10 {
                    for h in t..10 {
                        for tens in h..10 {
                            for o in tens..10 {
                                let value = vec![m, ht, tt, t, h, tens, o]
                                    .iter()
                                    .map(|c| c.to_string())
                                    .collect::<Vec<_>>()
                                    .join("")
                                    .parse::<u32>()
                                    .unwrap();
                                if value < upper
                                    && value > lower
                                    && check1(value)
                                {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn part2(input: &Vec<String>) -> u64 {
    let (lower, upper) = parse_input(input);
    let mut count = 0;
    for m in 0..10 {
        for ht in m..10 {
            for tt in ht..10 {
                for t in tt..10 {
                    for h in t..10 {
                        for tens in h..10 {
                            for o in tens..10 {
                                let value = vec![m, ht, tt, t, h, tens, o]
                                    .iter()
                                    .map(|c| c.to_string())
                                    .collect::<Vec<_>>()
                                    .join("")
                                    .parse::<u32>()
                                    .unwrap();
                                if value < upper
                                    && value > lower
                                    && check2(value)
                                {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(check1(111111), true);
        assert_eq!(check1(223450), false);
        assert_eq!(check1(123789), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(check2(112233), true);
        assert_eq!(check2(123444), false);
        assert_eq!(check2(111122), true);
    }
}
