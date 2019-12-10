extern crate num;

use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asteroid {
    pos: (i32, i32),
}

impl Asteroid {
    pub fn new(pos: (i32, i32)) -> Asteroid {
        Asteroid { pos }
    }

    pub fn find_dir(&self, other: &Asteroid) -> (i32, i32) {
        use crate::day10::num::Integer;

        let mut numerator = other.pos.1 - self.pos.1;
        let mut denominator = other.pos.0 - self.pos.0;

        if numerator == 0 && denominator == 0 {
            return (0, 0);
        }

        let mygcd = numerator.gcd(&denominator);

        numerator /= mygcd;
        denominator /= mygcd;

        (numerator, denominator)
    }

    pub fn find_ang(&self, other: &Asteroid) -> Option<i64> {
        let (y, x) = self.find_dir(other);
        if x == 0 && y == 0 {
            return None;
        }
        let fx = x as f64;
        let fy = y as f64;
        let mut ang = -((-fy).atan2(fx) - std::f64::consts::FRAC_PI_2);
        if ang < 0.0 {
            ang += 2.0 * std::f64::consts::PI;
        }
        Some((ang * 10000.) as i64)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Rel {
    ang: i64,
    range: i32,
    refast: Asteroid,
    other: Asteroid,
}

impl Rel {
    pub fn new(refast: &Asteroid, other: &Asteroid) -> Rel {
        let refast = refast.clone();
        let other = other.clone();
        let x = other.pos.0 - refast.pos.0;
        let y = other.pos.1 - refast.pos.1;
        let fx = x as f64;
        let fy = y as f64;
        let mut ang = -((-fy).atan2(fx) - std::f64::consts::FRAC_PI_2);
        if ang < 0.0 {
            ang += 2.0 * std::f64::consts::PI;
        }
        let ang = (ang * 10000.) as i64;
        let range = x * x + y * y;

        Rel {
            ang: ang,
            range: range,
            refast: refast,
            other: other,
        }
    }
}

impl Ord for Rel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.cmp(&other.range)
    }
}

impl PartialOrd for Rel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn load_input(name: &str) -> Vec<String> {
    let f = BufReader::new(File::open(name).unwrap());
    f.lines().map(|line| line.unwrap()).collect()
}

pub fn parse_input(input: &Vec<String>) -> Vec<Asteroid> {
    let mut output = vec![];
    for (y, line) in input.iter().enumerate() {
        for (x, point) in line.chars().enumerate() {
            if point == '#' {
                output.push(Asteroid::new((x as i32, y as i32)));
            }
        }
    }
    output
}

pub fn part1(input: &Vec<String>) -> usize {
    let asteroids = parse_input(&input);

    let mut max = 0;
    for a in &asteroids {
        let mut set = HashSet::new();
        for other in &asteroids {
            let dir = a.find_dir(&other);
            if dir.0 != 0 || dir.1 != 0 {
                set.insert(dir);
            }
        }
        if set.len() > max {
            max = set.len();
        }
    }
    max
}

pub fn part2(input: &Vec<String>) -> i32 {
    let asteroids = parse_input(&input);

    let mut max = 0;
    let mut max_asteroid = asteroids[0].clone();
    for a in &asteroids {
        let mut set = HashSet::new();
        for other in &asteroids {
            let dir = a.find_dir(&other);
            if dir.0 != 0 || dir.1 != 0 {
                set.insert(dir);
            }
        }
        if set.len() > max {
            max = set.len();
            max_asteroid = a.clone();
        }
    }

    let mut map: HashMap<i64, Vec<_>> = HashMap::new();
    for other in &asteroids {
        if let Some(ang) = max_asteroid.find_ang(&other) {
            let rel = Rel::new(&max_asteroid, &other);
            if let Some(v) = map.get_mut(&ang) {
                v.push(rel);
            } else {
                map.insert(rel.ang, vec![rel]);
            }
        }
    }

    for (_k, v) in map.iter_mut() {
        if v.len() > 1 {
            v.sort();
        }
    }

    let mut counter = 0;
    loop {
        for ang in 0..64000 {
            if let Some(v) = map.get_mut(&ang) {
                if v.len() > 0 {
                    let goal = v.remove(0);
                    counter += 1;

                    if counter == 200 {
                        return goal.other.pos.0 * 100 + goal.other.pos.1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = load_input("inputs/10a.txt");
        assert_eq!(part1(&input), 8);

        let input = load_input("inputs/10b.txt");
        assert_eq!(part1(&input), 33);

        let input = load_input("inputs/10c.txt");
        assert_eq!(part1(&input), 35);

        let input = load_input("inputs/10d.txt");
        assert_eq!(part1(&input), 41);

        let input = load_input("inputs/10e.txt");
        assert_eq!(part1(&input), 210);
    }

    #[test]
    fn test_part2() {
        let input = load_input("inputs/10e.txt");
        assert_eq!(part2(&input), 802);
    }
}
