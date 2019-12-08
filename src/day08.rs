extern crate ncurses;

use std::fs::File;
use std::io::prelude::*;

use ncurses::*;

pub fn load_input() -> Vec<u8> {
    let mut f =
        File::open("inputs/08.txt").expect("Could not open specified file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Failed to read input to string");

    buffer
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Not valid digit") as u8)
        .collect()
}

#[derive(Debug)]
pub struct Layer {
    data: Vec<Vec<u8>>,
}

impl Layer {
    pub fn count_digit(&self, digit: u8) -> usize {
        let mut count = 0;
        for row in &self.data {
            for pixel in row {
                if *pixel == digit {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn to_layers(input: &Vec<u8>, width: usize, height: usize) -> Vec<Layer> {
    let n_elements = height * width;
    let n_layers = input.len() / n_elements;
    let mut output = Vec::with_capacity(n_layers);
    let mut input_copy = input.clone();

    for _ in 0..n_layers {
        let mut layer = Layer {
            data: Vec::with_capacity(height),
        };
        for _ in 0..height {
            let line = input_copy.drain(..width).collect();
            layer.data.push(line);
        }
        output.push(layer);
    }

    output
}

pub fn flatten_layers(layers: &Vec<Layer>) -> Layer {
    let height = layers[0].data.len();
    let width = layers[0].data[0].len();
    let mut output = Layer {
        data: Vec::with_capacity(height),
    };

    // Fill output with transparency for now
    for _ in 0..height {
        let mut line = Vec::with_capacity(width);
        for _ in 0..width {
            line.push(2);
        }
        output.data.push(line);
    }

    for layer in layers {
        for (i, row) in (&layer).data.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                if output.data[i][j] == 2 {
                    output.data[i][j] = *pixel;
                }
            }
        }
    }
    output
}

#[allow(dead_code)]
pub fn print_layer(layer: &Layer) {
    // Start ncurses
    initscr();

    // Print to the back buffer
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_BLACK, COLOR_BLACK);

    for (y, row) in (&layer).data.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            if *pixel == 1 {
                attron(COLOR_PAIR(1));
                mvprintw(y as i32, x as i32, "#");
                attroff(COLOR_PAIR(1));
            } else {
                attron(COLOR_PAIR(2));
                mvprintw(y as i32, x as i32, ".");
                attroff(COLOR_PAIR(2));
            }
        }
    }

    // Update the screen
    refresh();

    // Wait for a keypress
    getch();

    // Terminate ncurses
    endwin();
}

pub fn part1(input: &Vec<u8>) -> usize {
    let layers = to_layers(input, 25, 6);

    // Find layer with minimum number of 0 pixels
    let mut min_count = std::usize::MAX;
    let mut min_idx = 0;
    for (i, layer) in layers.iter().enumerate() {
        let count = layer.count_digit(0);
        if count < min_count {
            min_count = count;
            min_idx = i;
        }
    }

    // Find number of 1 pixels * number of 2 pixels
    let ones = layers[min_idx].count_digit(1);
    let twos = layers[min_idx].count_digit(2);
    ones * twos
}

pub fn part2(input: &Vec<u8>) -> &str {
    let layers = to_layers(input, 25, 6);
    let _out_layer = flatten_layers(&layers);
    //print_layer(&_out_layer);

    "CEKUA"
}
