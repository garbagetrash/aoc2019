use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use crate::computer::{run, ProgramState, ProgramStatus};

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
            },
            Orientation::RIGHT => {
                if turn_dir == 0 {
                    self.orientation = Orientation::UP;
                } else {
                    self.orientation = Orientation::DOWN;
                }
            },
            Orientation::DOWN => {
                if turn_dir == 0 {
                    self.orientation = Orientation::RIGHT;
                } else {
                    self.orientation = Orientation::LEFT;
                }
            },
            Orientation::LEFT => {
                if turn_dir == 0 {
                    self.orientation = Orientation::UP;
                } else {
                    self.orientation = Orientation::DOWN;
                }
            },
        }
    }

    pub fn move_ahead(&mut self) {
        match self.orientation {
            Orientation::UP => {
                self.y -= 1;
            },
            Orientation::RIGHT => {
                self.x += 1;
            },
            Orientation::DOWN => {
                self.y += 1;
            },
            Orientation::LEFT => {
                self.x -= 1;
            },
        }
    }
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
        } else {
            panel.insert((robot.x, robot.y), 0);
        }

        // 0 = black, 1 = white
        let new_color = run(curr_color as i64, &mut state).expect("Should be 0 or 1");

        // 0 = left, 1 = right
        let turn_dir = run(0, &mut state).expect("Should be 0 or 1");

        match state.status {
            ProgramStatus::Halted => break,
            _ => {
                // Paint current square
                if let Some(color) = panel.get_mut(&(robot.x, robot.y)) {
                    color = new_color;
                } else {
                    panic!("This should have just been added to the HashMap");
                }

                // Turn
                robot.turn(turn_dir as u8);

                // Move
                robot.move_ahead();
            },
        }
    }
    panel.len()
}

pub fn part2(input: &Vec<i64>) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(0, 0);
    }
}
