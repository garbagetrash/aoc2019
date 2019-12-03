extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashMap;
use regex::Regex;


#[derive(Debug)]
pub struct Wires {
    wire1_cnt: u64,
    wire2_cnt: u64,
    wire1_len: u64,
    wire2_len: u64,
}

fn add_wires(pos: &mut (i64, i64), wire: u64, wire_len: &mut u64, dir: &str, len: u64, map: &mut HashMap::<(i64, i64), Wires>) {

    for _cnt in 0..len {
        // Increment wire
        if dir == "U" {
            (*pos).0 += 1;
        } else if dir == "R" {
            (*pos).1 += 1;
        } else if dir == "D" {
            (*pos).0 -= 1;
        } else if dir == "L" {
            (*pos).1 -= 1;
        }

        *wire_len += 1;

        // Add to hashmap
        if let Some(wires) = (*map).get_mut(pos) {
            if wire == 0 {
                (*wires).wire1_cnt += 1;
                if (*wires).wire1_len == 0 {
                    (*wires).wire1_len = *wire_len;
                }
            } else if wire == 1 {
                (*wires).wire2_cnt += 1;
                if (*wires).wire2_len == 0 {
                    (*wires).wire2_len = *wire_len;
                }
            } else {
                println!("THIS WIRE DOESNT EXIST WTF MATE");
            }
        } else {
            let mut wires = Wires { wire1_cnt: 0, wire2_cnt: 0, wire1_len: 0, wire2_len: 0 };
            if wire == 0 {
                wires.wire1_cnt += 1;
                wires.wire1_len = *wire_len;
            } else if wire == 1 {
                wires.wire2_cnt += 1;
                wires.wire2_len = *wire_len;
            } else {
                println!("THIS WIRE DOESNT EXIST WTF MATE");
            }
            (*map).insert(pos.clone(), wires);
        }
    }
}

pub fn load_input() -> Vec<String> {
    let f = BufReader::new(File::open("inputs/03.txt").unwrap());
    f.lines().map(|x| x.unwrap()).collect()
}

pub fn parse_input(lines: &Vec<String>) -> HashMap::<(i64, i64), Wires> {
    let mut map = HashMap::new();
    let re = Regex::new(r"^([URDL])(\d+)").unwrap();

    let l1 = lines[0].clone();
    let mut pos = (0, 0);
    let mut wire_len = 0;
    for el in l1.split(",") {
        let cap = re.captures(el).unwrap();
        let dir = &cap[1];
        let len = cap[2].parse::<u64>().unwrap();
        add_wires(&mut pos, 0, &mut wire_len, dir, len, &mut map);
    }

    let l2 = lines[1].clone();
    let mut pos = (0, 0);
    let mut wire_len = 0;
    for el in l2.split(",") {
        let cap = re.captures(el).unwrap();
        let dir = &cap[1];
        let len = cap[2].parse::<u64>().unwrap();
        add_wires(&mut pos, 1, &mut wire_len, dir, len, &mut map);
    }
    map
}

pub fn part1(input: &Vec<String>)-> u64 {
    let map = parse_input(&input);

    let mut min_dist = std::u64::MAX;
    for (pos, wires) in map {
        if wires.wire1_cnt > 0 && wires.wire2_cnt > 0 {
            let dist = pos.0.abs() + pos.1.abs();
            if (dist as u64) < min_dist {
                min_dist = dist as u64;
            }
        }
    }
    min_dist
}

pub fn part2(input: &Vec<String>) -> u64 {
    let map = parse_input(&input);

    let mut min_wire_len = std::u64::MAX;
    for (_pos, wires) in map {
        if wires.wire1_cnt > 0 && wires.wire2_cnt > 0 {
            let wire_len = wires.wire1_len + wires.wire2_len;
            if (wire_len as u64) < min_wire_len {
                min_wire_len = wire_len as u64;
            }
        }
    }
    min_wire_len
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")];
        assert_eq!(part1(&input), 159);
        let input = vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")];
        assert_eq!(part1(&input), 135);
    }

    #[test]
    fn test_part2() {
        let input = vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")];
        assert_eq!(part2(&input), 610);
        let input = vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")];
        assert_eq!(part2(&input), 410);
    }
}
