extern crate ncurses;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use std::thread::JoinHandle;
use std::sync::mpsc;

use ncurses::*;

#[derive(Debug, Clone)]
pub enum Move {
    North = 1,
    South,
    West,
    East,
}

#[derive(Debug, Clone)]
pub enum TileType {
    Start,
    End,
    Floor,
    Water,
    Wall,
    Portal(String),
}

pub struct Tile {
    tile_type: TileType,
    point: (i32, i32),
}

impl Tile {
    pub fn new(tile_type: TileType, point: (i32, i32)) -> Tile {
        Tile {
            tile_type,
            point,
        }
    }

    pub fn neighboring_tiles(&self) -> Vec<(i32, i32, Move)> {
        let pt = self.point;
        let npt = (pt.0, pt.1 - 1, Move::North);
        let spt = (pt.0, pt.1 + 1, Move::South);
        let ept = (pt.0 + 1, pt.1, Move::East);
        let wpt = (pt.0 - 1, pt.1, Move::West);

        vec![npt, spt, ept, wpt]
    }
}

pub enum DoorType {
    Inner,
    Outer,
}

pub struct Door {
    name: String,
    level: u32,
    door_type: DoorType,
    point: (i32, i32),
}

impl Door {
    pub fn new(name: String, level: u32, door_type: DoorType, point: (i32, i32)) -> Door {
        Door {
            name,
            level,
            door_type,
            point,
        }
    }
}

pub fn neighboring_grid_points(pt: &(i32, i32)) -> Vec<(i32, i32, Move)> {
    let npt = (pt.0, pt.1 - 1, Move::North);
    let spt = (pt.0, pt.1 + 1, Move::South);
    let ept = (pt.0 + 1, pt.1, Move::East);
    let wpt = (pt.0 - 1, pt.1, Move::West);

    vec![npt, spt, ept, wpt]
}


pub fn neighboring_grid_points_with_portals(pt: &(i32, i32), portals: &HashMap<(i32, i32), (i32, i32)>) -> Vec<(i32, i32, Move)> {
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

pub fn load_input(name: &str) -> HashMap<(i32, i32), TileType> {
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
                output.insert(point, TileType::Floor);
            },
            '#' => {
                output.insert(point, TileType::Wall);
            },
            '\n' => {
                x = -1;
                y += 1;
            },
            ' ' => (),
            _ => {
                output.insert(point, TileType::Portal(c.to_string()));
            },
        }
        x += 1;
    }

    output
}

pub fn start_render_thread() -> (mpsc::Sender<Option<HashMap<(i32, i32), TileType>>>, JoinHandle<()>) {
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
pub fn render(map: &HashMap<(i32, i32), TileType>, offset: &i32) {
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
            TileType::Start => {
                attron(COLOR_PAIR(3));
                let out = mvprintw(y - min_y, x - min_x, "S");
                attroff(COLOR_PAIR(3));
                out
            },
            TileType::End => {
                attron(COLOR_PAIR(4));
                let out = mvprintw(y - min_y, x - min_x, "E");
                attron(COLOR_PAIR(4));
                out
            },
            TileType::Floor => mvprintw(y - min_y, x - min_x, "."),
            TileType::Water => {
                attron(COLOR_PAIR(2));
                let out = mvprintw(y - min_y, x - min_x, "~");
                attroff(COLOR_PAIR(2));
                out
            },
            TileType::Wall => mvprintw(y - min_y, x - min_x, "#"),
            TileType::Portal(c) => mvprintw(y - min_y, x - min_x, &c.to_string()),
        };
        attroff(COLOR_PAIR(1));
    }

    refresh();
    thread::sleep(time::Duration::from_millis(33));
}

pub fn path(p1: &(i32, i32), p2: &(i32, i32), map: &HashMap<(i32, i32), TileType>) -> Option<u32> {
    let mut cloud: HashMap<(i32, i32), u32> = HashMap::new();
    cloud.insert(*p1, 0);

    loop {
        // Find new_points
        let mut new_points = HashMap::new();
        for (point, path) in &cloud {
            for tup in neighboring_grid_points(point) {
                let new_pt = (tup.0, tup.1);
                let new_path = path + 1;
                if !cloud.contains_key(&new_pt) {
                    if let Some(tile) = map.get(&new_pt) {
                        match tile {
                            TileType::Floor => {
                                new_points.insert(new_pt, new_path);
                            },
                            _ => (),
                        }
                    }
                }
            }
        }

        // Break if no new points
        if new_points.len() == 0 {
            break;
        }

        // Insert new_points into cloud
        for (point, path) in &new_points {
            if let Some(tile) = map.get(point) {
                match tile {
                    TileType::Wall => continue,
                    TileType::Portal(_) => continue,
                    _ => (),
                }
            }

            if let Some(existing_path) = cloud.get_mut(point) {
                if path < existing_path {
                    *existing_path = *path;
                }
            } else {
                cloud.insert(point.clone(), path.clone());
            }
        }

        // Break if we found a path to p2
        if cloud.contains_key(p2) {
            break;
        }
    }

    if let Some(value) = cloud.get(p2) {
        Some(value.clone())
    } else {
        None
    }
}

pub fn water_filling(map: &mut HashMap<(i32, i32), TileType>, portals: &HashMap<(i32, i32), (i32, i32)>, start_pt: &(i32, i32), end_pt: &(i32, i32)) -> i64 {

    //let (tx, handle) = start_render_thread();

    let mut cloud: HashMap<(i32, i32), Vec<Move>> = HashMap::new();
    cloud.insert(*start_pt, vec![]);

    let mut t = 1;
    loop {
        // Find new_points
        let mut new_points = HashMap::new();
        for (point, path) in &cloud {
            for tup in neighboring_grid_points_with_portals(point, portals) {
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
                    TileType::Wall => continue,
                    TileType::Portal(_) => continue,
                    _ => (),
                }
            }

            map.insert(*point, TileType::Water);

            if let Some(existing_path) = cloud.get_mut(point) {
                if path.len() < existing_path.len() {
                    *existing_path = (*path).clone();
                }
            } else {
                cloud.insert(point.clone(), path.clone());
            }
        }

        //tx.send(Some(map.clone())).unwrap();

        // See if end_pt is in cloud yet
        let mut done = false;
        if cloud.contains_key(end_pt) {
            done = true;
        }

        if done {
            //tx.send(None).unwrap();
            //handle.join().unwrap();
            break;
        }

        t += 1;
    }
    t
}

pub fn find_portals(input: &HashMap<(i32, i32), TileType>) -> ((i32, i32), (i32, i32), HashMap<(i32, i32), (i32, i32)>, HashMap<String, (i32, i32)>) {
    let mut portals: HashMap<String, Vec<((i32, i32), (i32, i32))>> = HashMap::new();
    for (pt, tile) in input {
        match tile {
            TileType::Portal(c) => {
                let right_pt = (pt.0 + 1, pt.1);
                let lower_pt = (pt.0, pt.1 + 1);
                if let Some(tile) = input.get(&right_pt) {
                    match tile {
                        TileType::Portal(c2) => {
                            let rrpt = (pt.0 + 2, pt.1);
                            let lrpt = (pt.0 - 1, pt.1);

                            // This case is when we are X_.
                            if let Some(tile2) = input.get(&rrpt) {
                                match tile2 {
                                    TileType::Floor => {
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
                                    TileType::Floor => {
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
                        TileType::Portal(c2) => {
                            let llpt = (pt.0, pt.1 + 2);
                            let ulpt = (pt.0, pt.1 - 1);

                            // This case is when we are X
                            //                          _
                            //                          .
                            if let Some(tile2) = input.get(&llpt) {
                                match tile2 {
                                    TileType::Floor => {
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
                                    TileType::Floor => {
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
    let mut portal_names = HashMap::new();
    let mut output = HashMap::new();
    for (name, v) in portals {
        if name == "AA" {
            start_pt = v[0].1;
            portal_names.insert(name.clone(), v[0].1);
        } else if name == "ZZ" {
            end_pt = v[0].1;
            portal_names.insert(name.clone(), v[0].1);
        } else {
            let pt1 = v[0];
            let pt2 = v[1];
            let name_a = format!("{}{}", name, "A");
            let name_b = format!("{}{}", name, "B");
            portal_names.insert(name_a, pt1.1);
            portal_names.insert(name_b, pt2.1);
            output.insert(pt1.0, pt2.1);
            output.insert(pt2.0, pt1.1);
        }
    }

    (start_pt, end_pt, output, portal_names)
}

pub fn explore_segments(map: &HashMap<(i32, i32), TileType>, name_map: &HashMap<String, (i32, i32)>, start_pt: &(i32, i32), end_pt: &(i32, i32)) -> HashMap<String, HashMap<String, u32>> {

    let mut output: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for name1 in name_map.keys() {
        let point1 = name_map.get(name1).unwrap();

        for name2 in name_map.keys() {
            let point2 = name_map.get(name2).unwrap();

            if let Some(dist) = path(&point1, &point2, map) {

                // If some path found from point1 to point2 sans portals
                if let Some(valuemap) = output.get_mut(name1) {
                    valuemap.insert((*name2).clone(), dist);
                } else {
                    let mut new_map = HashMap::new();
                    new_map.insert((*name2).clone(), dist);
                    output.insert((*name1).clone(), new_map);
                }
            }
        }
    }

    // Output a HashMap where the key is the starting points String name, and
    // the value is a HashMap of the portals that can be reached from that
    // starting point.  The value HashMaps key is the String name of the
    // destination, and the value HashMaps value is the distance between the
    // two points.
    output
}

pub fn part1(input: &HashMap<(i32, i32), TileType>) -> i64 {
    let mut map = input.clone();
    let (start_pt, end_pt, portals, _names) = find_portals(&map);

    // Populate start and stop tiles
    map.insert(start_pt, TileType::Start);
    map.insert(end_pt, TileType::End);

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
pub fn part2(input: &HashMap<(i32, i32), TileType>) -> u32 {
    let mut map = input.clone();
    let (start_pt, end_pt, _portals, names) = find_portals(&map);

    // Populate start and stop tiles
    map.insert(start_pt, TileType::Start);
    map.insert(end_pt, TileType::End);

    let seg_map = explore_segments(&mut map, &names, &start_pt, &end_pt);
    println!("Finished exploring");

    // Viterbi(-like) algorithm here
    let mut dist_map = HashMap::new();
    let dests = seg_map.get("AA").unwrap();
    for (dpt, dist) in dests {
        dist_map.insert(dpt.clone(), *dist);
    }

    let mut jumps = 0;
    loop {
        println!("jumps: {}", jumps);
        let old_dist_map = dist_map.clone();
        for (dest, dest_dist) in dist_map.clone() {
            println!("dest: {}", dest);

            // Get the other door
            let preamble = dest.chars().take(2).collect::<String>();

            let mut other = "A";
            if preamble != "AA" && preamble != "ZZ" {
                let current_letter = dest.as_bytes()[2] as char;
                match current_letter {
                    'A' => other = "B",
                    'B' => other = "A",
                    _ => continue,
                }
            } else {
                continue;
            }

            let other = format!("{}{}", preamble, other.to_string());
            println!("other: {}", other);
            let new_dests_map = seg_map.get(&other).expect("seg_map should have other");

            for (new_dest, new_dist) in new_dests_map {
                println!("new_dests_map: {:?}", new_dests_map);
                if !dist_map.contains_key(new_dest) {
                    dist_map.insert(new_dest.clone(), dest_dist + new_dist);
                }
            }
        }

        if old_dist_map.len() == dist_map.len() {
            break;
        }
        jumps += 1;
    }

    println!("{:?}", dist_map);
    *dist_map.get("ZZ").expect("Final result should be in dist_map")
}
