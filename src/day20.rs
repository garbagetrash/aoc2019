extern crate ncurses;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use ncurses::*;


#[derive(Debug)]
pub enum Tile {
    Floor,
    Wall,
    Portal(String),
}

pub fn load_input(name: &str) -> HashMap<(i32, i32), Tile> {
    let mut f = File::open(name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let mut output = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for c in buffer.chars() {
        let point = (x, y);
        match c {
            '.' => {
                output.insert(point, Tile::Floor);
            },
            '#' => {
                output.insert(point, Tile::Wall);
            },
            '\n' => {
                x = -1;
                y += 1;
            },
            ' ' => (),
            _ => {
                output.insert(point, Tile::Portal(c.to_string()));
            },
        }
        x += 1;
    }

    output
}

#[allow(dead_code)]
pub fn render(map: &HashMap<(i32, i32), Tile>) {
    let mut min_x = 0;
    let mut min_y = 0;
    for (x, y) in map.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
    }
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    clear();
    for ((x, y), tile) in map {
        match tile {
            Tile::Floor => mvprintw(y - min_y, x - min_x, "."),
            Tile::Wall => mvprintw(y - min_y, x - min_x, "#"),
            Tile::Portal(c) => mvprintw(y - min_y, x - min_x, &c.to_string()),
        };
    }

    refresh();
    getch();
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    endwin();
}

pub fn find_portals(input: &HashMap<(i32, i32), Tile>) -> HashMap<(i32, i32), (i32, i32)> {
    let mut portals: HashMap<String, Vec<((i32, i32), (i32, i32))>> = HashMap::new();
    for (pt, tile) in input {
        match tile {
            Tile::Portal(c) => {
                let right_pt = (pt.0 + 1, pt.1);
                let lower_pt = (pt.0, pt.1 + 1);
                if let Some(tile) = input.get(&right_pt) {
                    match tile {
                        Tile::Portal(c2) => {
                            let rrpt = (pt.0 + 2, pt.1);
                            let lrpt = (pt.0 - 1, pt.1);

                            // This case is when we are X_.
                            if let Some(tile2) = input.get(&rrpt) {
                                match tile2 {
                                    Tile::Floor => {
                                        let s = vec![(*c).clone(), (*c2).clone()].join("");
                                        if let Some(v) = portals.get_mut(&s) {
                                            v.push((right_pt, rrpt));
                                        } else {
                                            portals.insert(s, vec![(right_pt, rrpt)]);
                                        }
                                    },
                                    _ => (),
                                }
                            }

                            // This case is when we are .X_
                            if let Some(tile2) = input.get(&lrpt) {
                                match tile2 {
                                    Tile::Floor => {
                                        let s = vec![(*c).clone(), (*c2).clone()].join("");
                                        if let Some(v) = portals.get_mut(&s) {
                                            v.push((*pt, lrpt));
                                        } else {
                                            portals.insert(s, vec![(*pt, lrpt)]);
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        },
                        _ => {},
                    }
                }
                if let Some(tile) = input.get(&lower_pt) {
                    match tile {
                        Tile::Portal(c2) => {
                            let llpt = (pt.0, pt.1 + 2);
                            let ulpt = (pt.0, pt.1 - 1);

                            // This case is when we are X
                            //                          _
                            //                          .
                            if let Some(tile2) = input.get(&llpt) {
                                match tile2 {
                                    Tile::Floor => {
                                        let s = vec![(*c).clone(), (*c2).clone()].join("");
                                        if let Some(v) = portals.get_mut(&s) {
                                            v.push((lower_pt, llpt));
                                        } else {
                                            portals.insert(s, vec![(lower_pt, llpt)]);
                                        }
                                    },
                                    _ => (),
                                }
                            }

                            // This case is when we are .
                            //                          X
                            //                          _
                            if let Some(tile2) = input.get(&ulpt) {
                                match tile2 {
                                    Tile::Floor => {
                                        let s = vec![(*c).clone(), (*c2).clone()].join("");
                                        if let Some(v) = portals.get_mut(&s) {
                                            v.push((*pt, ulpt));
                                        } else {
                                            portals.insert(s, vec![(*pt, ulpt)]);
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        },
                        _ => {},
                    }
                }
            },
            _ => (),
        }
    }
    println!("{:?}", portals);

    let mut output = HashMap::new();
    for (name, v) in portals {
        if name != "AA" && name != "ZZ" {
            let pt1 = v[0];
            let pt2 = v[1];
            output.insert(pt1.0, pt2.1);
            output.insert(pt2.0, pt1.1);
        }
    }

    output
}

pub fn part1(input: &HashMap<(i32, i32), Tile>) -> i64 {
    render(input);
    let portals = find_portals(input);
    println!("portals: {:?}", portals);
    0
}

pub fn part2(input: &HashMap<(i32, i32), Tile>) -> i64 {
    0
}
