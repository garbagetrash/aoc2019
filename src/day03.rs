extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashMap;
use regex::Regex;


struct Wires {
    wire1_cnt: u64,
    wire2_cnt: u64,
}

pub fn load_input() -> HashMap::<(i64, i64), Wires> {
    let f = BufReader::new(File::open("inputs/03.txt").unwrap());
    let lines: Vec<String> = f.lines().map(|x| x.unwrap()).collect();
    let l1 = lines[0].clone();
    let re = Regex::new(r"^([URDL])(\d+)").unwrap();
    for el in l1.split(",") {
        let cap = re.captures(el).unwrap();
        let dir = &cap[1];
        let len = &cap[2];
        println!("{:?}", dir);
        println!("{:?}", len);
    }
    let l2 = lines[1].clone();
    HashMap::new()
}

pub fn do_a_thing() -> u64 {
    0
}

pub fn part1(_input: HashMap::<(i64, i64), Wires>) -> u64 {
    do_a_thing()
}

pub fn part2(_input: HashMap::<(i64, i64), Wires>) -> u64 {
    do_a_thing()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(do_a_thing(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(do_a_thing(), 0);
    }
}
