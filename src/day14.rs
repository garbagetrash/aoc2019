extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;


pub fn load_input(name: &str) -> Vec<String> {
    let f = BufReader::new(File::open(name).unwrap());
    f.lines().map(|x| x.unwrap()).collect()
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub num: i64,
    pub inputs: Vec<(String, i64)>,
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
            let num = cap[1].parse::<i64>().unwrap();
            let name = cap[2].to_string();
            inputs.push((name.clone(), num));
        }
        let cap = re.captures(out_group).unwrap();
        let num = cap[1].parse::<i64>().unwrap();
        let name = cap[2].to_string();

        let rule = Rule { name: name.clone(), num: num, inputs: inputs };
        output.insert(name, rule);
    }
    output
}

pub fn convert(rule: &Rule, stock: &mut HashMap<String, i64>) {
    if let Some(value) = stock.get_mut(&(*rule).name) {
        *value -= rule.num;
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

pub fn part1(input: &Vec<String>) -> i64 {
    let rule_map = parse_input(input);

    // Keep track of transactions and current stock
    let mut stock: HashMap<String, i64> = HashMap::new();
    stock.insert(String::from("FUEL"), 1);
    let mut done = false;
    loop {
        let mut siter = stock.iter();
        let (mut el, mut num) = siter.next().unwrap();
        while el == "ORE" || *num <= 0 {
            if stock.len() == 1 {
                done = true;
                break;
            }
            if let Some(temp) = siter.next() {
                el = temp.0;
                num = temp.1;
            } else {
                done = true;
                break;
            }
        }

        if done {
            break;
        }

        // Something other than ORE
        let rule = rule_map.get(&String::from(el)).unwrap();
        convert(&rule, &mut stock);
    }
    *stock.get("ORE").unwrap()
}

pub fn enough_stock(rule: &Rule, stock: &HashMap<String, i64>) -> bool {

    for (el, num) in &(*rule).inputs {
        if let Some(value) = stock.get(el) {
            if *value < *num {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

pub fn part2(input: &Vec<String>) -> i64 {
    let rule_map = parse_input(input);

    // Keep track of transactions and current stock
    let mut stock: HashMap<String, i64> = HashMap::new();
    stock.insert(String::from("FUEL"), 1);
    stock.insert(String::from("ORE"), 1000000000000);
    let mut done = false;
    loop {
        if let Some(value) = stock.get_mut(&String::from("FUEL")) {
            if *value <= 0 {
                *value = 1;
            }
        }
        let mut siter = stock.iter();
        let (mut el, _num) = siter.next().unwrap();

        loop {
            // Handle something
            let rule = rule_map.get(&String::from(el)).unwrap();
            if enough_stock(&rule, &stock) {
                convert(&rule, &mut stock);
                break;
            } else {
                if let Some(temp) = siter.next() {
                    el = temp.0;
                } else {
                    done = true;
                    break;
                }
            }
        }

        if done {
            break;
        }
    }
    *stock.get("FUEL").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = load_input("inputs/14a.txt");
        assert_eq!(part1(&input), 31);

        let input = load_input("inputs/14b.txt");
        assert_eq!(part1(&input), 165);

        let input = load_input("inputs/14c.txt");
        assert_eq!(part1(&input), 13312);

        let input = load_input("inputs/14d.txt");
        assert_eq!(part1(&input), 180697);

        let input = load_input("inputs/14e.txt");
        assert_eq!(part1(&input), 2210736);
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}
