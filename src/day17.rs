extern crate ncurses;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

use crate::computer::{run, ProgramState};

use ncurses::*;

pub fn load_input(name: &str) -> Vec<i64> {
    let mut f = File::open(name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let mut output = Vec::new();
    for el in buffer.split(",") {
        if let Ok(x) = el.trim().parse::<i64>() {
            output.push(x)
        }
    }
    output
}

#[derive(Debug)]
pub enum Tile {
    Scaffold,
    Open,
    RobotUp,
    RobotDown,
    RobotLeft,
    RobotRight,
}

pub fn parse_input(input: &Vec<i64>) -> HashMap<(i32, i32), Tile> {
    let mut output = HashMap::new();

    let mut state = ProgramState::new(input);

    let mut x = 0;
    let mut y = 0;
    loop {
        if let Some(out) = run(0, &mut state) {
            match out {
                10 => { // LF
                    y += 1;
                    x = 0;
                },
                35 => { // #
                    output.insert((x, y), Tile::Scaffold);
                    x += 1;
                },
                46 => { // .
                    output.insert((x, y), Tile::Open);
                    x += 1;
                },
                94 => { // ^
                    output.insert((x, y), Tile::RobotUp);
                    x += 1;
                },
                118 => { // v
                    output.insert((x, y), Tile::RobotDown);
                    x += 1;
                },
                60 => { // <
                    output.insert((x, y), Tile::RobotLeft);
                    x += 1;
                },
                62 => { // >
                    output.insert((x, y), Tile::RobotRight);
                    x += 1;
                },
                _ => panic!("Output {} unrecognized"),
            }
        } else {
            break;
        }
    }

    output
}

pub fn is_intersection(pt: &(i32, i32), map: &HashMap<(i32, i32), Tile>) -> bool {
    let x = pt.0;
    let y = pt.1;

    // Current point
    if let Some(tile) = map.get(&(x, y)) {
        match tile {
            Tile::Scaffold => (),
            _ => return false,
        }
    } else {
        return false;
    }

    // Above
    if let Some(tile) = map.get(&(x, y - 1)) {
        match tile {
            Tile::Scaffold => (),
            _ => return false,
        }
    } else {
        return false;
    }

    // Below
    if let Some(tile) = map.get(&(x, y + 1)) {
        match tile {
            Tile::Scaffold => (),
            _ => return false,
        }
    } else {
        return false;
    }

    // Left
    if let Some(tile) = map.get(&(x - 1, y)) {
        match tile {
            Tile::Scaffold => (),
            _ => return false,
        }
    } else {
        return false;
    }

    // Right
    if let Some(tile) = map.get(&(x + 1, y)) {
        match tile {
            Tile::Scaffold => (),
            _ => return false,
        }
    } else {
        return false;
    }

    true
}

pub fn part1(input: &Vec<i64>) -> i64 {
    let map = parse_input(input);

    let mut ap_sum = 0;
    for (point, _tile) in &map {
        if is_intersection(&point, &map) {
            ap_sum += (point.0 * point.1) as i64;
        }
    }

    ap_sum
}

pub fn part2(input: &Vec<i64>) -> i64 {
    0
}
