extern crate ncurses;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use std::thread::JoinHandle;
use std::sync::mpsc;

use ncurses::*;


#[derive(Debug, Clone)]
pub enum Tile {
    Start,
    End,
    Floor,
    Water,
    Wall,
    Portal(String),
}

#[derive(Debug, Clone)]
pub enum Move {
    North = 1,
    South,
    West,
    East,
}

pub fn neighboring_grid_points(pt: &(i32, i32), portals: &HashMap<(i32, i32), (i32, i32)>) -> Vec<(i32, i32, Move)> {
    let mut npt = (pt.0, pt.1 - 1, Move::North);
    let mut spt = (pt.0, pt.1 + 1, Move::South);
    let mut ept = (pt.0 + 1, pt.1, Move::East);
    let mut wpt = (pt.0 - 1, pt.1, Move::West);

    // Handle portals
    if let Some(portal_pt) = portals.get(&(npt.0, npt.1)) {
        npt = (portal_pt.0, portal_pt.1, npt.2);
    }
    if let Some(portal_pt) = portals.get(&(spt.0, spt.1)) {
        spt = (portal_pt.0, portal_pt.1, spt.2);
    }
    if let Some(portal_pt) = portals.get(&(ept.0, ept.1)) {
        ept = (portal_pt.0, portal_pt.1, ept.2);
    }
    if let Some(portal_pt) = portals.get(&(wpt.0, wpt.1)) {
        wpt = (portal_pt.0, portal_pt.1, wpt.2);
    }

    vec![npt, spt, ept, wpt]
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

pub fn start_render_thread() -> (mpsc::Sender<Option<HashMap<(i32, i32), Tile>>>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();

    let handle: thread::JoinHandle<()> = thread::spawn(move || {

        let window = initscr();
        start_color();
        init_pair(1, COLOR_WHITE, COLOR_BLACK);
        init_pair(2, COLOR_BLUE, COLOR_BLACK);
        init_pair(3, COLOR_GREEN, COLOR_BLACK);
        init_pair(4, COLOR_RED, COLOR_BLACK);

        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        noecho();
        nodelay(window, true);
        keypad(stdscr(), true);

        // Offset for display height
        let mut offset = 0;

        loop {
            // Handle input if button pressed (non-blocking getch call)
            let c = getch();
            if c == KEY_DOWN {
                offset += 1;
                clear();
            } else if c == KEY_UP {
                offset -= 1;
                clear();
            }

            // Handle message if one available
            if let Some(map) = rx.recv().unwrap() {
                render(&map, &offset);
            } else {
                break;
            }
        }

        clear();
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        refresh();
        endwin();
    });

    (tx, handle)
}

#[allow(dead_code)]
pub fn render(map: &HashMap<(i32, i32), Tile>, offset: &i32) {
    let mut min_x = 0;
    let min_y = offset;
    for (x, _y) in map.keys() {
        if *x < min_x {
            min_x = *x;
        }
    }

    for ((x, y), tile) in map {
        attron(COLOR_PAIR(1));
        match tile {
            Tile::Start => {
                attron(COLOR_PAIR(3));
                let out = mvprintw(y - min_y, x - min_x, "S");
                attroff(COLOR_PAIR(3));
                out
            },
            Tile::End => {
                attron(COLOR_PAIR(4));
                let out = mvprintw(y - min_y, x - min_x, "E");
                attron(COLOR_PAIR(4));
                out
            },
            Tile::Floor => mvprintw(y - min_y, x - min_x, "."),
            Tile::Water => {
                attron(COLOR_PAIR(2));
                let out = mvprintw(y - min_y, x - min_x, "~");
                attroff(COLOR_PAIR(2));
                out
            },
            Tile::Wall => mvprintw(y - min_y, x - min_x, "#"),
            Tile::Portal(c) => mvprintw(y - min_y, x - min_x, &c.to_string()),
        };
        attroff(COLOR_PAIR(1));
    }

    refresh();
    thread::sleep(time::Duration::from_millis(33));
}

pub fn path(p1: &(i32, i32), p2: &(i32, i32), map: &HashMap<(i32, i32), (Tile, Vec<Move>)>, portals: &HashMap<(i32, i32), (i32, i32)>) -> Vec<Move> {
    let mut cloud: HashMap<(i32, i32), Vec<Move>> = HashMap::new();
    cloud.insert(*p1, vec![]);

    loop {
        // Find new_points
        let mut new_points = HashMap::new();
        for (point, path) in &cloud {
            for tup in neighboring_grid_points(point, portals) {
                let new_pt = (tup.0, tup.1);
                let mut new_path = path.clone();
                new_path.push(tup.2);
                if !cloud.contains_key(&new_pt) {
                    new_points.insert(new_pt, new_path);
                }
            }
        }

        // Insert new_points into cloud
        for (point, path) in &new_points {
            if let Some((tile, _)) = map.get(point) {
                match tile {
                    Tile::Wall => continue,
                    _ => (),
                }
            }

            if let Some(existing_path) = cloud.get_mut(point) {
                if path.len() < existing_path.len() {
                    *existing_path = (*path).clone();
                }
            } else {
                cloud.insert(point.clone(), path.clone());
            }
        }

        if cloud.contains_key(p2) {
            break;
        }
    }

    (*cloud.get(p2).unwrap()).clone()
}

pub fn water_filling(map: &mut HashMap<(i32, i32), Tile>, portals: &HashMap<(i32, i32), (i32, i32)>, start_pt: &(i32, i32), end_pt: &(i32, i32)) -> i64 {

    let (tx, handle) = start_render_thread();

    let mut cloud: HashMap<(i32, i32), Vec<Move>> = HashMap::new();
    cloud.insert(*start_pt, vec![]);

    let mut t = 1;
    loop {
        // Find new_points
        let mut new_points = HashMap::new();
        for (point, path) in &cloud {
            for tup in neighboring_grid_points(point, portals) {
                let new_pt = (tup.0, tup.1);
                let mut new_path = path.clone();
                new_path.push(tup.2);
                if !cloud.contains_key(&new_pt) {
                    new_points.insert(new_pt, new_path);
                }
            }
        }

        // Insert new_points into cloud
        for (point, path) in &new_points {
            if let Some(tile) = &map.get(point) {
                match tile {
                    Tile::Wall => continue,
                    Tile::Portal(_) => continue,
                    _ => (),
                }
            }

            map.insert(*point, Tile::Water);

            if let Some(existing_path) = cloud.get_mut(point) {
                if path.len() < existing_path.len() {
                    *existing_path = (*path).clone();
                }
            } else {
                cloud.insert(point.clone(), path.clone());
            }
        }

        tx.send(Some(map.clone())).unwrap();

        // See if end_pt is in cloud yet
        let mut done = false;
        if cloud.contains_key(end_pt) {
            done = true;
        }

        if done {
            tx.send(None).unwrap();
            handle.join().unwrap();
            break;
        }

        t += 1;
    }
    t
}

pub fn find_portals(input: &HashMap<(i32, i32), Tile>) -> ((i32, i32), (i32, i32), HashMap<(i32, i32), (i32, i32)>) {
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

    let mut start_pt = (0, 0);
    let mut end_pt = (0, 0);
    let mut output = HashMap::new();
    for (name, v) in portals {
        if name == "AA" {
            start_pt = v[0].1;
        } else if name == "ZZ" {
            end_pt = v[0].1;
        } else {
            let pt1 = v[0];
            let pt2 = v[1];
            output.insert(pt1.0, pt2.1);
            output.insert(pt2.0, pt1.1);
        }
    }

    (start_pt, end_pt, output)
}



pub fn part1(input: &HashMap<(i32, i32), Tile>) -> i64 {
    let mut map = input.clone();
    let (start_pt, end_pt, portals) = find_portals(&map);

    // Populate start and stop tiles
    map.insert(start_pt, Tile::Start);
    map.insert(end_pt, Tile::End);

    let out = water_filling(&mut map, &portals, &start_pt, &end_pt);
    out
}

// For this part most likely water filling alone won't be fast enough.
//
// I'm thinking of using a map that is keyed off of the portal entrances, and
// the value would be a set of the portals that can be reached from the keyed
// portal, along with a number of steps to each of them in a tuple.  Like this:
//
//          Keyed Portal        (Other Portal, Dist. to Other)
// HashMap<(String, Char), HashSet<((String, Char), u32)>>
//
// The portals are represented by a tuple of the portal string, and then a char
// designating either that it is the inner or the outer edge portal.
//
// When creating the sets we need to take care that the start and end are only
// counted as portals on the outermost (1st) level of the maze.
//
// We can use water_filling with modified rules (don't traverse portals) to
// distances between connected portals, and which portals are connected at all.
// Then we create our dataset and traverse the graph it creates, keeping track
// of each of the states (portals) and the shortest path from the start to that
// state.  Once we've gotten to all of them we simply look at the path to the
// end portal. (Something sort of like the Viterbi Algorithm?)
pub fn part2(input: &HashMap<(i32, i32), Tile>) -> i64 {
    let mut map = input.clone();
    let (start_pt, end_pt, portals) = find_portals(&map);

    // Populate start and stop tiles
    map.insert(start_pt, Tile::Start);
    map.insert(end_pt, Tile::End);

    let out = water_filling(&mut map, &portals, &start_pt, &end_pt);
    out
}
