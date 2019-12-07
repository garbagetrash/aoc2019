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

            let input = day06::load_input();
            println!("Day 6:");
            println!("Part 1 Solution {:?}", day06::part1(&input));
            println!("Part 2 Solution {:?}", day06::part2(&input));

            let input = day07::load_input();
            println!("Day 7:");
            println!("Part 1 Solution {:?}", day07::part1(&input));
            println!("Part 2 Solution {:?}", day07::part2(&input));
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
            let input = day06::load_input();
            println!("Part 1 Solution {:?}", day06::part1(&input));
            println!("Part 2 Solution {:?}", day06::part2(&input));
        }
        7 => {
            let input = day07::load_input();
            println!("Part 1 Solution {:?}", day07::part1(&input));
            println!("Part 2 Solution {:?}", day07::part2(&input));
        }
        _ => println!("Day {} not yet implemented", day),
    }
}
