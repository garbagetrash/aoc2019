extern crate clap;

use clap::{App, Arg};

mod computer;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let matches = App::new("AOC2019")
        .arg(
            Arg::with_name("DAY")
                .required(true)
                .index(1)
                .help("Day number to run"),
        )
        .get_matches();

    let day = matches.value_of("DAY").unwrap().parse().unwrap();

    match day {
        0 => {
            let input = day01::load_input();
            println!("Day 1:");
            println!("Part 1 Solution {:?}", day01::part1(&input));
            println!("Part 2 Solution {:?}", day01::part2(&input));

            let input = day02::load_input();
            println!("Day 2:");
            println!("Part 1 Solution {:?}", day02::part1(&input));
            println!("Part 2 Solution {:?}", day02::part2(&input));

            let input = day03::load_input();
            println!("Day 3:");
            println!("Part 1 Solution {:?}", day03::part1(&input));
            println!("Part 2 Solution {:?}", day03::part2(&input));

            let input = day04::load_input();
            println!("Day 4:");
            println!("Part 1 Solution {:?}", day04::part1(&input));
            println!("Part 2 Solution {:?}", day04::part2(&input));

            let input = day05::load_input();
            println!("Day 5:");
            println!("Part 1 Solution {:?}", day05::part1(&input));
            println!("Part 2 Solution {:?}", day05::part2(&input));

            let input = day06::load_input("inputs/06.txt");
            println!("Day 6:");
            println!("Part 1 Solution {:?}", day06::part1(&input));
            println!("Part 2 Solution {:?}", day06::part2(&input));

            let input = day07::load_input();
            println!("Day 7:");
            println!("Part 1 Solution {:?}", day07::part1(&input));
            println!("Part 2 Solution {:?}", day07::part2(&input));

            let input = day08::load_input();
            println!("Day 8:");
            println!("Part 1 Solution {:?}", day08::part1(&input));
            println!("Part 2 Solution {}", day08::part2(&input));

            let input = day09::load_input();
            println!("Day 9:");
            println!("Part 1 Solution {:?}", day09::part1(&input));
            println!("Part 2 Solution {:?}", day09::part2(&input));

            let input = day10::load_input("inputs/10.txt");
            println!("Day 10:");
            println!("Part 1 Solution {:?}", day10::part1(&input));
            println!("Part 2 Solution {:?}", day10::part2(&input));
        }
        1 => {
            let input = day01::load_input();
            println!("Part 1 Solution {:?}", day01::part1(&input));
            println!("Part 2 Solution {:?}", day01::part2(&input));
        }
        2 => {
            let input = day02::load_input();
            println!("Part 1 Solution {:?}", day02::part1(&input));
            println!("Part 2 Solution {:?}", day02::part2(&input));
        }
        3 => {
            let input = day03::load_input();
            println!("Part 1 Solution {:?}", day03::part1(&input));
            println!("Part 2 Solution {:?}", day03::part2(&input));
        }
        4 => {
            let input = day04::load_input();
            println!("Part 1 Solution {:?}", day04::part1(&input));
            println!("Part 2 Solution {:?}", day04::part2(&input));
        }
        5 => {
            let input = day05::load_input();
            println!("Part 1 Solution {:?}", day05::part1(&input));
            println!("Part 2 Solution {:?}", day05::part2(&input));
        }
        6 => {
            let input = day06::load_input("inputs/06.txt");
            println!("Part 1 Solution {:?}", day06::part1(&input));
            println!("Part 2 Solution {:?}", day06::part2(&input));
        }
        7 => {
            let input = day07::load_input();
            println!("Part 1 Solution {:?}", day07::part1(&input));
            println!("Part 2 Solution {:?}", day07::part2(&input));
        }
        8 => {
            let input = day08::load_input();
            println!("Part 1 Solution {:?}", day08::part1(&input));
            println!("Part 2 Solution {}", day08::part2(&input));
        }
        9 => {
            let input = day09::load_input();
            println!("Part 1 Solution {:?}", day09::part1(&input));
            println!("Part 2 Solution {:?}", day09::part2(&input));
        }
        10 => {
            let input = day10::load_input("inputs/10.txt");
            println!("Part 1 Solution {:?}", day10::part1(&input));
            println!("Part 2 Solution {:?}", day10::part2(&input));
        }
        11 => {
            let input = day11::load_input();
            println!("Part 1 Solution {:?}", day11::part1(&input));
            println!("Part 2 Solution {:?}", day11::part2(&input));
        }
        _ => println!("Day {} not yet implemented", day),
    }
}
