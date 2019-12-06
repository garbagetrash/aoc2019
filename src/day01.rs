use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn fuel_required(mass: u64) -> u64 {
    ((mass as f64) / 3.0).floor() as u64 - 2
}

fn fuel_required_recursive(mass: u64) -> u64 {
    let mut sum_total = 0;
    let mut new_mass = mass;
    while new_mass >= 6 {
        new_mass = fuel_required(new_mass);
        sum_total += new_mass
    }

    sum_total
}

pub fn load_input() -> Vec<u64> {
    let f = BufReader::new(File::open("inputs/01.txt").unwrap());
    f.lines()
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .collect()
}

pub fn part1(input: &Vec<u64>) -> u64 {
    input.iter().map(|x| fuel_required(*x)).sum()
}

pub fn part2(input: &Vec<u64>) -> u64 {
    input.iter().map(|x| fuel_required_recursive(*x)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(fuel_required_recursive(14), 2);
        assert_eq!(fuel_required_recursive(1969), 966);
        assert_eq!(fuel_required_recursive(100756), 50346);
    }
}
