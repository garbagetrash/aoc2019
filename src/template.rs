pub mod template {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    pub fn load_input() -> Vec<String> {
        let f = BufReader::new(File::open("inputs/XX.txt").unwrap());
        f.lines().map(|x| x.unwrap()).collect()
    }

    pub fn part1() -> i32 {
        println!("Not yet implemented!");
        0
    }

    pub fn part2() -> i32 {
        println!("Not yet implemented!");
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn part1examples() {
            assert_eq!(part1(), 0);
        }

        #[test]
        fn part2examples() {
            assert_eq!(part2(), 0);
        }
    }
}
