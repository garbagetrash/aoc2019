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

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub num: u64,
    pub inputs: Vec<(String, u64)>,
}

impl Rule {
    pub fn follow(&self, map: &HashMap<String, Rule>) -> u64 {
        let mut output = 0;
        for (el, num) in &self.inputs {
            match el.as_ref() {
                "ORE" => output += num,
                _ => {
                    let mut final_num = num / self.num;
                    if num % self.num > 0 {
                        final_num += 1;
                    }
                    output += final_num * map.get(el).unwrap().follow(map);
                }
            };
        }
        output
    }
}

pub fn parse_input(input: &Vec<String>) -> HashMap<String, Rule> {
    let re = Regex::new(r"(\d+) ([A-Z]+)").unwrap();

    let mut output  = HashMap::new();
    for line in input {
        let groups = line.split(',').collect::<Vec<_>>();
        let last_group = groups[groups.len() - 1];
        let equality = last_group.split('>').collect::<Vec<_>>();
        let out_group = equality[1];

        let mut inputs = vec![];
        for g in groups {
            let cap = re.captures(g).unwrap();
            let num = cap[1].parse::<u64>().unwrap();
            let name = cap[2].to_string();
            inputs.push((name.clone(), num));
        }
        let cap = re.captures(out_group).unwrap();
        let num = cap[1].parse::<u64>().unwrap();
        let name = cap[2].to_string();

        let rule = Rule { name: name.clone(), num: num, inputs: inputs };
        output.insert(name, rule);
    }
    output
}

pub fn convert(rule: &Rule, stock: &mut HashMap<String, u64>) {
    if let Some(value) = stock.get_mut(&(*rule).name) {
        if *value < rule.num {
            // Not enough stock, this branch corresponds to waste, but is
            // actually allowable since we're kind of working in reverse
            *value = 0;
        } else {
            *value -= rule.num;
        }
        if *value == 0 {
            stock.remove(&(*rule).name);
        }
    } else {
        // No stock of output, panic
        panic!(format!("Cannot run this transaction:\n{:?}", *rule));
    }
    for (el, num) in &rule.inputs {
        if let Some(value) = stock.get_mut(el) {
            // Increase stock of el by num
            *value += num;
        } else {
            stock.insert((*el).clone(), *num);
        }
    }
}

pub fn part1(input: &Vec<String>) -> u64 {
    let rule_map = parse_input(input);

    // Keep track of transactions and current stock
    let mut stock: HashMap<String, u64> = HashMap::new();
    stock.insert(String::from("FUEL"), 1);
    loop {
        let mut siter = stock.iter();
        let (mut el, mut num) = siter.next().unwrap();
        if el == "ORE" {
            if stock.len() == 1 {
                break;
            }
            let temp = siter.next().unwrap();
            el = temp.0;
            num = temp.1;
        }
        // Something other than ORE
        let rule = rule_map.get(&String::from(el)).unwrap();
        println!("\n{:?}", rule);
        println!("stock: {:?}", stock);
        convert(&rule, &mut stock);
        println!("stock: {:?}", stock);
    }
    *stock.iter().next().unwrap().1
}

pub fn part2(input: &Vec<String>) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = load_input("inputs/14a.txt");
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}
