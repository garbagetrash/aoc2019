extern crate regex;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

#[derive(Debug)]
pub struct Wires {}

pub fn load_input() -> Vec<String> {
    let f = BufReader::new(File::open("inputs/06.txt").unwrap());
    f.lines().map(|x| x.unwrap()).collect()
}

pub fn parse_input(input: &Vec<String>) -> (HashMap<String, String>, HashMap<String, u64>) {
    let re = Regex::new(r"^([0-9A-Z]+)\)([0-9A-Z]+)").unwrap();

    let mut mapping = HashMap::new();
    let mut counter = HashMap::new();
    for line in input {
        let cap = re.captures(line.as_str()).unwrap();
        let parent = cap[1].to_string();
        let name = cap[2].to_string();
        mapping.insert(name.clone(), parent);
        counter.insert(name.clone(), 0);
    }

    (mapping, counter)
}

pub fn part1(input: &Vec<String>) -> u64 {
    let (mapping, mut counter) = parse_input(input);
    for (name, orbit) in &mapping {
        let mut count = 0;

        let mut next = orbit;
        loop {
            if next != "COM" {
                next = mapping.get(next).unwrap();
                count += 1;
            } else {
                count += 1;
                break;
            }
        }
        if let Some(x) = counter.get_mut(name) {
            *x = count;
        }
    }

    let mut total_count = 0;
    for (_name, count) in &counter {
        total_count += count;
    }
    total_count
}

pub fn part2(input: &Vec<String>) -> u64 {
    let (mapping, _counter) = parse_input(input);
    let santa = mapping
        .get(&String::from("SAN"))
        .expect("Failed to get Santa!");
    let you = mapping
        .get(&String::from("YOU"))
        .expect("Failed to get you!");

    let mut santa_path = HashSet::new();
    let mut you_path = HashSet::new();

    let mut next = santa;
    loop {
        santa_path.insert(next.clone());
        if next != "COM" {
            next = mapping.get(next).unwrap();
        } else {
            // Last stop!
            break;
        }
    }

    let mut next = you;
    loop {
        you_path.insert(next.clone());
        if next != "COM" {
            next = mapping.get(next).unwrap();
        } else {
            // Last stop!
            break;
        }
    }

    let path: HashSet<_> = santa_path.symmetric_difference(&you_path).collect();

    path.len() as u64
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        assert_eq!(0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}
