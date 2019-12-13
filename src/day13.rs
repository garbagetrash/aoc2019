extern crate ncurses;

use std::collections::HashMap;
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

pub fn part1(input: &Vec<i64>) -> usize {
    let mut state = ProgramState::new(input);
    let mut screen = HashMap::new();
    let mut output = vec![];
    loop {
        if let Some(out) = run(0, &mut state) {
            output.push(out);
        } else {
            break;
        }
    }

    for chunk in output.chunks(3) {
        screen.insert((chunk[0], chunk[1]), chunk[2]);
    }

    let mut cntr = 0;
    for (_key, value) in screen {
        if value == 2 {
            cntr += 1;
        }
    }
    cntr
}

#[allow(dead_code)]
pub fn render_screen(screen: &HashMap<(i64, i64), i64>, score: i64) {
    for ((x, y), id) in screen {
        match *id {
            0 => mvprintw(*y as i32, *x as i32, " "),
            1 => mvprintw(*y as i32, *x as i32, "#"),
            2 => mvprintw(*y as i32, *x as i32, "-"),
            3 => mvprintw(*y as i32, *x as i32, "="),
            4 => mvprintw(*y as i32, *x as i32, "o"),
            _ => continue,
        };
    }

    // Render score
    mvprintw(26, 12, format!("Score: {}", score).as_str());

    refresh();
}

pub fn part2(input: &Vec<i64>) -> u64 {
    let mut state = ProgramState::new(input);
    state.memory[0] = 2;
    let mut screen = HashMap::new();
    let mut score = 0;
    let mut ballx = 0;
    let mut paddlex = 0;

    let render_mode = false;

    if render_mode {
        initscr();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }
    loop {
        let mut inst = Vec::with_capacity(3);

        for ((x, _y), id) in &screen {
            if *id == 3 {
                paddlex = *x;
            } else if *id == 4 {
                ballx = *x;
            }
        }

        let mut dir = 0;
        if ballx > paddlex {
            dir = 1;
        } else if ballx < paddlex {
            dir = -1;
        }

        // Grab next instruction
        if let Some(out) = run(dir, &mut state) {
            inst.push(out);
        } else {
            // Halted
            break;
        }
        if let Some(out) = run(dir, &mut state) {
            inst.push(out);
        }
        if let Some(out) = run(dir, &mut state) {
            inst.push(out);
        }

        if inst[0] == -1 && inst[1] == 0 {
            // Update score
            score = inst[2];
        } else {
            screen.insert((inst[0], inst[1]), inst[2]);
        }

        if render_mode {
            render_screen(&screen, score);
        }
    }
    if render_mode {
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        endwin();
    }
    score as u64
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
