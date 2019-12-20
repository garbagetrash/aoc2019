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

    let mut output = HashMap::new();
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

        let rule = Rule {
            name: name.clone(),
            num: num,
            inputs: inputs,
        };
        output.insert(name, rule);
    }
    output
}

pub fn convert(rule: &Rule, stock: &mut HashMap<String, i64>) {
    let mut mult = 1;
    if let Some(value) = stock.get_mut(&(*rule).name) {
        if *value > rule.num {
            mult = *value / rule.num;
        }
        *value -= mult * rule.num;
    } else {
        // No stock of output, panic
        panic!(format!("Cannot run this transaction:\n{:?}", *rule));
    }
    for (el, num) in &rule.inputs {
        if let Some(value) = stock.get_mut(el) {
            // Increase stock of el by num
            *value += mult * num;
        } else {
            stock.insert((*el).clone(), *num * mult);
        }
    }
}

pub fn solve_part1(input: &Vec<String>, stock: &mut HashMap<String, i64>) {
    let rule_map = parse_input(input);

    // Keep track of transactions and current stock
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
        convert(&rule, stock);
    }
}

pub fn part1(input: &Vec<String>) -> i64 {
    let mut stock = HashMap::new();
    stock.insert(String::from("FUEL"), 1);
    solve_part1(input, &mut stock);
    *stock.get("ORE").unwrap()
}

pub fn part2(input: &Vec<String>) -> i64 {
    // Solve once to get max ore per fuel
    let mut stock = HashMap::new();
    stock.insert(String::from("FUEL"), 1);
    solve_part1(input, &mut stock);
    let mut max_ore_per_fuel = 0;
    if let Some(value) = stock.get("ORE") {
        max_ore_per_fuel = *value;
    }

    let mut stock = HashMap::new();
    let mut fuel_cntr = 0;

    // Now we see what else we can do
    let mut ore_remaining = 1000000000000;
    loop {
        // First do what we know we can all at once
        let mut fuel = ore_remaining / max_ore_per_fuel;
        if fuel == 0 {
            fuel = 1;
        }
        stock.insert(String::from("FUEL"), fuel);
        solve_part1(input, &mut stock);
        if let Some(value) = stock.get("ORE") {
            if *value >= 1000000000000 {
                break;
            } else {
                fuel_cntr += fuel;
                ore_remaining = 1000000000000 - *value;
            }
        }
    }
    fuel_cntr
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
        let input = load_input("inputs/14c.txt");
        assert_eq!(part2(&input), 82892753);

        let input = load_input("inputs/14d.txt");
        assert_eq!(part2(&input), 5586022);

        let input = load_input("inputs/14e.txt");
        assert_eq!(part2(&input), 460664);
    }
}
