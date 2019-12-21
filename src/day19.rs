extern crate ncurses;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::computer::{run, ProgramState};

use ncurses::*;

pub enum Tile {
    Open,
    Beam,
}

#[allow(dead_code)]
pub fn render(map: HashMap<(i32, i32), Tile>) {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    clear();
    for ((x, y), tile) in map {
        match tile {
            Tile::Open => mvprintw(y, x, "."),
            Tile::Beam => mvprintw(y, x, "#"),
        };
    }
    refresh();
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    getch();
    endwin();
}

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

pub fn part1(input: &Vec<i64>) -> i64 {
    let mut map = HashMap::new();

    let mut sum = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut state = ProgramState::new(input);
            run(x as i64, &mut state);
            if let Some(value) = run(y as i64, &mut state) {
                if value == 1 {
                    sum += 1;
                    map.insert((x, y), Tile::Beam);
                } else {
                    map.insert((x, y), Tile::Open);
                }
            }
        }
    }

    //render(map);

    sum
}

pub fn check(pt: &(i32, i32), input: &Vec<i64>) -> bool {
    let mut state = ProgramState::new(input);
    run(pt.0 as i64, &mut state);
    if let Some(value) = run(pt.1 as i64, &mut state) {
        if value == 1 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn part2(input: &Vec<i64>) -> i64 {
    let mut map = HashMap::new();

    for x in 0..10 {
        for y in 0..10 {
            let mut state = ProgramState::new(input);
            run(x as i64, &mut state);
            if let Some(value) = run(y as i64, &mut state) {
                if value == 1 {
                    map.insert((x, y), Tile::Beam);
                } else {
                    map.insert((x, y), Tile::Open);
                }
            }
        }
    }

    let mut bpx = 0;
    let mut bpy = 0;
    for (point, tile) in map {
        match tile {
            Tile::Beam => {
                if point.1 > bpy {
                    bpx = point.0;
                    bpy = point.1;
                } else if point.1 == bpy && point.0 < bpx {
                    bpx = point.0;
                    bpy = point.1;
                }
            }
            _ => (),
        }
    }

    // Travel along bottom edge, check (+99, -99) to see if also Beam
    let mut point = (bpx, bpy);
    loop {
        // Increment x
        point.0 += 1;

        // Find new point max y
        loop {
            let cand_point = (point.0, point.1 + 1);
            if !check(&cand_point, input) {
                // Found highest y for this x
                break;
            } else {
                point = cand_point;
            }
        }

        // New point
        if check(&point, input) {
            let other_point = (point.0 + 99, point.1 - 99);
            if check(&other_point, input) {
                break;
            }
        }
    }

    // Want (0, -99) from point
    let upper_left = (point.0, point.1 - 99);
    (upper_left.0 * 10000 + upper_left.1) as i64
}
