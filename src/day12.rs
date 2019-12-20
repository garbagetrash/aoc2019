extern crate num;
extern crate regex;

use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

pub fn load_input(name: &str) -> Vec<String> {
    let f = BufReader::new(File::open(name).unwrap());
    f.lines().map(|x| x.unwrap()).collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    let moons = parse_input(input);
    simulate(&moons, 1000)
}

pub fn part2(input: &Vec<String>) -> u64 {
    let mut moons = parse_input(input);
    let ref_moons = moons.clone();

    let x0 = ref_moons.iter().map(|m| m.x).collect::<Vec<_>>();
    let y0 = ref_moons.iter().map(|m| m.y).collect::<Vec<_>>();
    let z0 = ref_moons.iter().map(|m| m.z).collect::<Vec<_>>();
    let vx0 = ref_moons.iter().map(|m| m.vx).collect::<Vec<_>>();
    let vy0 = ref_moons.iter().map(|m| m.vy).collect::<Vec<_>>();
    let vz0 = ref_moons.iter().map(|m| m.vz).collect::<Vec<_>>();

    let mut x_cands = vec![];
    let mut y_cands = vec![];
    let mut z_cands = vec![];
    let mut vx_cands = vec![];
    let mut vy_cands = vec![];
    let mut vz_cands = vec![];

    let mut i = 1_u64;
    loop {
        time_step(&mut moons);

        let xs = moons.iter().map(|m| m.x).collect::<Vec<_>>();
        let ys = moons.iter().map(|m| m.y).collect::<Vec<_>>();
        let zs = moons.iter().map(|m| m.z).collect::<Vec<_>>();
        let vxs = moons.iter().map(|m| m.vx).collect::<Vec<_>>();
        let vys = moons.iter().map(|m| m.vy).collect::<Vec<_>>();
        let vzs = moons.iter().map(|m| m.vz).collect::<Vec<_>>();

        if xs == x0 {
            x_cands.push(i);
        }
        if ys == y0 {
            y_cands.push(i);
        }
        if zs == z0 {
            z_cands.push(i);
        }
        if vxs == vx0 {
            vx_cands.push(i);
        }
        if vys == vy0 {
            vy_cands.push(i);
        }
        if vzs == vz0 {
            vz_cands.push(i);
        }

        if x_cands.len() > 1
            && y_cands.len() > 1
            && z_cands.len() > 1
            && vx_cands.len() > 1
            && vy_cands.len() > 1
            && vz_cands.len() > 1
        {
            break;
        }
        i += 1;
    }

    use crate::day12::num::Integer;

    // Find LCM of our number, keeping in mind that the very first instance of
    // each position corresponds to just prior to the velocities zeroing out.
    let xylcm = x_cands[1].lcm(&y_cands[1]);
    let zvxlcm = z_cands[1].lcm(&vx_cands[1]);
    let vyvzlcm = vy_cands[1].lcm(&vz_cands[1]);
    let lcm0 = xylcm.lcm(&zvxlcm);
    let output = lcm0.lcm(&vyvzlcm);

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = load_input("inputs/12a.txt");
        let moons = parse_input(&input);
        assert_eq!(simulate(&moons, 10), 179);

        let input = load_input("inputs/12b.txt");
        let moons = parse_input(&input);
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
