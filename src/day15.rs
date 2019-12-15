extern crate ncurses;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

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


#[derive(Clone)]
pub enum Move {
    North = 1,
    South,
    West,
    East,
}

pub enum Status {
    HitWall,
    MoveSuccess,
    FoundOxygen,
}

impl From<i64> for Status {
    fn from(val: i64) -> Self {
        match val {
            0 => Status::HitWall,
            1 => Status::MoveSuccess,
            2 => Status::FoundOxygen,
            _ => panic!("Unknown output"),
        }
    }
}

#[derive(Debug)]
pub enum Block {
    Wall,
    Open,
    Oxygen,
}

pub struct Droid {
    pub x: i32,
    pub y: i32,
}

pub fn next_pos(pos: &(i32, i32), dir: &Move) -> (i32, i32) {
    let mut output = pos.clone();
    match dir {
        Move::North => output.1 -= 1,
        Move::South => output.1 += 1,
        Move::West => output.0 -= 1,
        Move::East => output.0 += 1,
    }
    output
}

impl Droid {
    pub fn new(x: i32, y: i32) -> Droid {
        Droid {
            x,
            y,
        }
    }

    pub fn move_dir(&mut self, dir: &Move) {
        let new = next_pos(&(self.x, self.y), dir);
        self.x = new.0;
        self.y = new.1;
    }
}

#[derive(Debug)]
pub struct Map {
    pub map: HashMap<(i32, i32), Block>,
    pub unexplored: HashSet<(i32, i32)>,
}

impl Map {
    pub fn new() -> Map {
        let mut map = HashMap::new();
        map.insert((0, 0), Block::Open);

        let mut set = HashSet::new();
        set.insert(next_pos(&(0, 0), &Move::North));
        set.insert(next_pos(&(0, 0), &Move::South));
        set.insert(next_pos(&(0, 0), &Move::West));
        set.insert(next_pos(&(0, 0), &Move::East));
        Map {
            map: map,
            unexplored: set,
        }
    }

    pub fn insert(&mut self, key: (i32, i32), value: Block) {
        self.map.insert(key, value);
        if let Some(value) = self.unexplored.get(&key) {
            self.unexplored.remove(&key);
        }
    }

    pub fn update_unexplored(&mut self, pos: &(i32, i32)) {
        // Adds new unexplored tiles to four adjacent squares to pos.
        let mut new_dirs = vec![];
        new_dirs.push(next_pos(pos, &Move::North));
        new_dirs.push(next_pos(pos, &Move::South));
        new_dirs.push(next_pos(pos, &Move::West));
        new_dirs.push(next_pos(pos, &Move::East));

        for new in new_dirs {
            if let Some(value) = self.map.get(&new) {
                continue;
            } else {
                self.unexplored.insert(new);
            }
        }
    }

    pub fn render(&self, droid: &Droid) {
        let mut min_x = 0;
        let mut min_y = 0;
        for (x, y) in self.map.keys() {
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
        }
        initscr();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        for ((x, y), block) in &self.map {
            match block {
                Block::Wall => mvprintw(y - min_y, x - min_x, "#"),
                Block::Open => mvprintw(y - min_y, x - min_x, "."),
                Block::Oxygen => mvprintw(y - min_y, x - min_x, "O"),
            };
        }
        mvprintw(droid.y - min_y, droid.x - min_x, "D");

        refresh();
        getch();
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        endwin();
    }
}

pub fn part1(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(input);
    let mut map = Map::new();
    let mut droid = Droid::new(0, 0);

    loop {
        let command = Move::East;
        let status = Status::from(run(command.clone() as i64, &mut state).unwrap());

        match status {
            Status::HitWall => {
                let pos = (droid.x, droid.y);
                droid.move_dir(&command);
                map.insert((droid.x, droid.y), Block::Wall);
                droid.x = pos.0;
                droid.y = pos.1;
                println!("Hit a wall");
                break;
            },
            Status::MoveSuccess => {
                droid.move_dir(&command);
                map.insert((droid.x, droid.y), Block::Open);
                map.update_unexplored(&(droid.x, droid.y));
                println!("Move success to: ({}, {})", &droid.x, &droid.y);
            },
            Status::FoundOxygen => {
                droid.move_dir(&command);
                map.insert((droid.x, droid.y), Block::Oxygen);
                map.update_unexplored(&(droid.x, droid.y));
                println!("Found Oxygen: ({}, {})", droid.x, droid.y);
            },
        }
    }
    map.render(&droid);
    println!("{:?}", map);
    0
}

pub fn part2(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(input);
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}
