extern crate regex;

use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

use crate::computer::{run, ProgramState};

use regex::Regex;

pub fn load_input(name: &str) -> Vec<String> {
    let f = BufReader::new(File::open(name).unwrap());
    f.lines().map(|x| x.unwrap()).collect()
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Moon {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub vx: i32,
    pub vy: i32,
    pub vz: i32,
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x: x,
            y: y,
            z: z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    pub fn update_pos(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    pub fn pot_energy(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64 + self.z.abs() as u64
    }

    pub fn kin_energy(&self) -> u64 {
        self.vx.abs() as u64 + self.vy.abs() as u64 + self.vz.abs() as u64
    }

    pub fn total_energy(&self) -> u64 {
        self.pot_energy() * self.kin_energy()
    }
}

pub fn parse_input(input: &Vec<String>) -> Vec<Moon> {
    let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let mut output = vec![];
    for line in input {
        let cap = re.captures(line.as_str()).unwrap();
        let x = cap[1].parse::<i32>().unwrap();
        let y = cap[2].parse::<i32>().unwrap();
        let z = cap[3].parse::<i32>().unwrap();
        let moon = Moon::new(x, y, z);
        output.push(moon);
    }
    output
}

pub fn calc_gravity(moons: &Vec<Moon>) -> Vec<(i32, i32, i32)> {
    let mut output = Vec::with_capacity(moons.len());
    for refmoon in moons {
        let mut gx = 0;
        let mut gy = 0;
        let mut gz = 0;
        for other in moons {
            if refmoon.x > other.x {
                gx -= 1;
            } else {
                gx += 1;
            }
            if refmoon.y > other.y {
                gy -= 1;
            } else {
                gy += 1;
            }
            if refmoon.z > other.z {
                gz -= 1;
            } else {
                gz += 1;
            }
        }
        output.push((gx, gy, gz));
    }
    output
}

pub fn predict_order_change(moons: &Vec<Moon>) -> u64 {
    let ref_moons = moons.clone();
    let idx_iter = (0..moons.len()).combinations(2);

    let mut ts = Vec::with_capacity(idx_iter.count());
    let idx_iter = (0..moons.len()).combinations(2);

    let gravity = calc_gravity(moons);
    for v in idx_iter {
        let m0 = &ref_moons[v[0]];
        let m1 = &ref_moons[v[1]];
        let g0 = gravity[v[0]].0;
        let g1 = gravity[v[1]].0;
        let t = ((m0.x - m1.x) as f64 / (g0 - g1) as f64).sqrt() + 1.0;
        ts.push(t as u64);
    }
    println!("{:?}", ts);
    ts[0]
}

pub fn time_step(moons: &mut Vec<Moon>) {
    // Gravity
    for v in (0..moons.len()).combinations(2) {
        if moons[v[0]].x > moons[v[1]].x {
            moons[v[0]].vx -= 1;
            moons[v[1]].vx += 1;
        } else if moons[v[0]].x < moons[v[1]].x {
            moons[v[1]].vx -= 1;
            moons[v[0]].vx += 1;
        }

        if moons[v[0]].y > moons[v[1]].y {
            moons[v[0]].vy -= 1;
            moons[v[1]].vy += 1;
        } else if moons[v[0]].y < moons[v[1]].y {
            moons[v[1]].vy -= 1;
            moons[v[0]].vy += 1;
        }

        if moons[v[0]].z > moons[v[1]].z {
            moons[v[0]].vz -= 1;
            moons[v[1]].vz += 1;
        } else if moons[v[0]].z < moons[v[1]].z {
            moons[v[1]].vz -= 1;
            moons[v[0]].vz += 1;
        }
    }

    // Velocity
    for moon in moons {
        moon.update_pos();
    }
}

pub fn simulate(in_moons: &Vec<Moon>, n_steps: usize) -> u64 {
    let mut moons = in_moons.clone();
    for _ in 0..n_steps {
        time_step(&mut moons);
    }

    // Total energy
    let mut total_energy = 0;
    for moon in moons {
        total_energy += moon.total_energy();
    }
    total_energy
}

pub fn part1(input: &Vec<String>) -> u64 {
    let mut moons = parse_input(input);
    simulate(&moons, 1000)
}

pub fn part2(input: &Vec<String>) -> u64 {
    let mut moons = parse_input(input);
    let ref_moons = moons.clone();

    let mut i = 1;
    loop {
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        time_step(&mut moons);

        if moons == ref_moons {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = load_input("inputs/12a.txt");
        let moons = parse_input(&input);
        predict_order_change(&moons);
        assert_eq!(simulate(&moons, 10), 179);

        let input = load_input("inputs/12b.txt");
        let moons = parse_input(&input);
        predict_order_change(&moons);
        assert_eq!(simulate(&moons, 100), 1940);
    }

    #[test]
    fn test_part2() {
        let input = load_input("inputs/12a.txt");
        assert_eq!(part2(&input), 2772);

        let input = load_input("inputs/12b.txt");
        assert_eq!(part2(&input), 4686774924);
    }
}
