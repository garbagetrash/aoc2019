extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;


pub fn load_input(name: &str) -> Vec<String> {
    let f = BufReader::new(File::open(name).unwrap());
    f.lines().map(|x| x.unwrap()).collect()
}

pub struct Rule {
    pub name: String,
    pub num: i32,
    pub inputs: Vec<(String, i32)>,
}

pub fn parse_input(input: &Vec<String>) -> HashMap<String, Rule> {
    let re = Regex::new(r"(\d+) ([A-Z]+)").unwrap();

    let mut output  = HashMap::new();
    for line in input {
        for group in line.split(|c| c == ',' || c == '>') {
            let mut inputs = vec![];
            let cap = re.captures(group).unwrap();
            let out_num = cap[cap.len() - 2].parse::<i32>().unwrap();
            let out_type = cap[cap.len() - 1].to_string();

            for i in (0..cap.len()) {
                println!("{:?}", cap[i].to_string());
            }
            println!("");

            let rule = Rule { name: out_type.clone(), num: out_num, inputs: inputs };
            output.insert(out_type, rule);
        }
    }
    output
}

pub fn part1(input: &Vec<String>) -> usize {
    let rule_map = parse_input(input);
    0
}

pub fn part2(input: &Vec<String>) -> u64 {
    0
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
        assert_eq!(0, 0);
    }
}
