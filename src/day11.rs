extern crate ncurses;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::computer::{run, ProgramState};

use ncurses::*;

pub fn load_input() -> Vec<i64> {
    let mut f = File::open("inputs/11.txt").unwrap();
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

pub enum Orientation {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            orientation: Orientation::UP,
        }
    }

    pub fn turn(&mut self, turn_dir: u8) {
        // 0 = left, 1 = right
        match self.orientation {
            Orientation::UP => {
                if turn_dir == 0 {
                    self.orientation = Orientation::LEFT;
                } else {
                    self.orientation = Orientation::RIGHT;
                }
            }
            Orientation::RIGHT => {
                if turn_dir == 0 {
                    self.orientation = Orientation::UP;
                } else {
                    self.orientation = Orientation::DOWN;
                }
            }
            Orientation::DOWN => {
                if turn_dir == 0 {
                    self.orientation = Orientation::RIGHT;
                } else {
                    self.orientation = Orientation::LEFT;
                }
            }
            Orientation::LEFT => {
                if turn_dir == 0 {
                    self.orientation = Orientation::DOWN;
                } else {
                    self.orientation = Orientation::UP;
                }
            }
        }
    }

    pub fn move_ahead(&mut self) {
        match self.orientation {
            Orientation::UP => {
                self.y -= 1;
            }
            Orientation::RIGHT => {
                self.x += 1;
            }
            Orientation::DOWN => {
                self.y += 1;
            }
            Orientation::LEFT => {
                self.x -= 1;
            }
        }
    }
}

#[allow(dead_code)]
pub fn print_panel(panel: &HashMap<(i32, i32), u8>) {
    let mut min_x = 0;
    let mut min_y = 0;
    for (x, y) in panel.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
    }

    // Ncurses stuff
    initscr();

    for ((x, y), color) in panel.iter() {
        if *color == 0 {
            mvprintw(y - min_y, x - min_x, ".");
        } else {
            mvprintw(y - min_y, x - min_x, "#");
        }
    }
    refresh();
    getch();
    endwin();
}

pub fn part1(input: &Vec<i64>) -> usize {
    // Hashmap represents the whole panel, tuple (x, y) is key, value is 0 or 1
    // to represent the spot on the panel being black or white respectively
    let mut panel: HashMap<(i32, i32), u8> = HashMap::new();
    let mut robot = Robot::new();
    let mut state = ProgramState::new(&input);

    // Main loop
    loop {
        let mut curr_color = 0;
        if let Some(color) = panel.get_mut(&(robot.x, robot.y)) {
            curr_color = *color;
        }

        // 0 = black, 1 = white
        if let Some(new_color) = run(curr_color as i64, &mut state) {
            // 0 = left, 1 = right
            if let Some(turn_dir) = run(0, &mut state) {
                // Paint current square
                if let Some(color) = panel.get_mut(&(robot.x, robot.y)) {
                    *color = new_color as u8;
                } else {
                    panel.insert((robot.x, robot.y), new_color as u8);
                }

                // Turn
                robot.turn(turn_dir as u8);

                // Move
                robot.move_ahead();
            } else {
                panic!("Second break, shouldn't see this");
            }
        } else {
            break;
        }
    }
    panel.len()
}

pub fn part2(input: &Vec<i64>) -> &str {
    // Hashmap represents the whole panel, tuple (x, y) is key, value is 0 or 1
    // to represent the spot on the panel being black or white respectively
    let mut panel: HashMap<(i32, i32), u8> = HashMap::new();
    let mut robot = Robot::new();
    let mut state = ProgramState::new(&input);

    // Main loop
    let mut first_loop = true;
    loop {
        let mut curr_color = 0;
        if first_loop {
            curr_color = 1;
            first_loop = false;
        }
        if let Some(color) = panel.get_mut(&(robot.x, robot.y)) {
            curr_color = *color;
        }

        // 0 = black, 1 = white
        if let Some(new_color) = run(curr_color as i64, &mut state) {
            // 0 = left, 1 = right
            if let Some(turn_dir) = run(0, &mut state) {
                // Paint current square
                if let Some(color) = panel.get_mut(&(robot.x, robot.y)) {
                    *color = new_color as u8;
                } else {
                    panel.insert((robot.x, robot.y), new_color as u8);
                }

                // Turn
                robot.turn(turn_dir as u8);

                // Move
                robot.move_ahead();
            } else {
                panic!("Second break, shouldn't see this");
            }
        } else {
            break;
        }
    }
    //print_panel(&panel);
    "CGPJCGCL"
}
