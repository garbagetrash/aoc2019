extern crate clap;

use clap::{App, Arg};

mod day01;
mod day02;


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
        _ => println!("Day {} not yet implemented", day),
    }
}
