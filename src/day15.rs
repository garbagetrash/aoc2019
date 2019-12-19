extern crate ncurses;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

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
    pub map: HashMap<(i32, i32), (Block, Vec<Move>)>,
    pub unexplored: HashSet<(i32, i32)>,
}

impl Map {
    pub fn new() -> Map {
        let mut map = HashMap::new();
        map.insert((0, 0), (Block::Open, vec![]));

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

    pub fn insert(&mut self, key: (i32, i32), value: (Block, Vec<Move>)) {
        if let Some(tempvalue) = self.map.get_mut(&key) {
            // If already present, update path if and only if shorter
            if tempvalue.1.len() > value.1.len() {
                tempvalue.1 = value.1.to_vec();
            }
        } else {
            // If not already present, insert it
            self.map.insert(key, value);
        }
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

    // TODO: Make this actually intelligent
    pub fn path(&self, p1: &(i32, i32), p2: &(i32, i32)) -> Vec<Move> {
        let mut output = vec![];

        // First we backtrack to the origin
        if let Some(value) = self.map.get(p1) {
            let path = &value.1;
            for m in path.iter().rev() {
                match m {
                    Move::North => output.push(Move::South),
                    Move::South => output.push(Move::North),
                    Move::East => output.push(Move::West),
                    Move::West => output.push(Move::East),
                }
            }
        } else {
            panic!("How did we get here if not in map?");
        }

        // Then we forward track to p2
        let mut fwd = vec![];
        let mut done = false;
        for (pt, value) in &self.map {
            match value.0 {
                Block::Wall => continue,
                _ => (),
            }
            let candpts = neighboring_grid_points(&pt);
            for (x, y, m) in candpts {
                if (x, y) == *p2 {
                    fwd.append(&mut value.1.clone());
                    fwd.push(m);
                    done = true;
                    break;
                }
            }
            if done {
                break;
            }
        }
        output.append(&mut fwd);
        output
    }

    pub fn render(&self, droid: &Droid, text: &str) {
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
        for ((x, y), (block, _path)) in &self.map {
            match block {
                Block::Wall => mvprintw(y - min_y, x - min_x, "#"),
                Block::Open => mvprintw(y - min_y, x - min_x, "."),
                Block::Oxygen => mvprintw(y - min_y, x - min_x, "O"),
            };
        }
        mvprintw(droid.y - min_y, droid.x - min_x, "D");
        mvprintw(30, 0, text);

        refresh();
        getch();
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        endwin();
    }
}

pub fn neighboring_grid_points(pt: &(i32, i32)) -> Vec<(i32, i32, Move)> {
    let npt = (pt.0, pt.1 - 1, Move::North);
    let spt = (pt.0, pt.1 + 1, Move::South);
    let ept = (pt.0 + 1, pt.1, Move::East);
    let wpt = (pt.0 - 1, pt.1, Move::West);

    vec![npt, spt, ept, wpt]
}

pub fn part1(input: &Vec<i64>) -> i64 {
    let mut state = ProgramState::new(input);
    let mut map = Map::new();
    let mut droid = Droid::new(0, 0);

    // Explore the entire map
    loop {
        if map.unexplored.len() > 0 {
            let target_pt = map.unexplored.iter().take(1).next().unwrap().clone();

            // Path to target point
            let command_seq = &map.path(&(droid.x, droid.y), &target_pt);
            //map.render(&droid, format!("pos: {:?}\ntarget: {:?}\nseq: {:?}", (droid.x, droid.y), target_pt, command_seq).as_str());
            for command in command_seq {
                if !map.unexplored.contains(&target_pt) {
                    break;
                }
                let status = Status::from(run(command.clone() as i64, &mut state).unwrap());

                let mut new_path = (&map.map.get(&(droid.x, droid.y)).unwrap().1).clone();
                new_path.push(command.clone());

                match status {
                    Status::HitWall => {
                        let pos = (droid.x, droid.y);
                        droid.move_dir(&command);
                        map.insert((droid.x, droid.y), (Block::Wall, new_path.to_vec()));
                        droid.x = pos.0;
                        droid.y = pos.1;
                    },
                    Status::MoveSuccess => {
                        droid.move_dir(&command);
                        map.insert((droid.x, droid.y), (Block::Open, new_path.to_vec()));
                        map.update_unexplored(&(droid.x, droid.y));
                    },
                    Status::FoundOxygen => {
                        droid.move_dir(&command);
                        map.insert((droid.x, droid.y), (Block::Oxygen, new_path.to_vec()));
                        map.update_unexplored(&(droid.x, droid.y));
                    },
                }
            }
        } else {
            // unexplored is now size 0, done exploring!
            break;
        }
    }
    map.render(&droid, "");

    for (pt, (block, path)) in map.map {
        match block {
            Block::Oxygen => return path.len() as i64,
            _ => (),
        }
    }
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
