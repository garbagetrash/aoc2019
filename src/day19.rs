extern crate ncurses;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

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

    render(map);

    sum
}

pub fn part2(input: &Vec<i64>) -> i64 {
    0
}
