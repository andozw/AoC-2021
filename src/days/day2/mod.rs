use crate::common::get_file;
use std::io::BufRead;

fn part1(filename: &str) -> i32 {
    let reader = get_file(filename);

    let mut vertical: i32 = 0;
    let mut horizontal: i32 = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut words = line.split_whitespace();
        let command = words.next().unwrap().trim();
        let distance: i32 = words.next().unwrap().parse().expect("Not a valid number");

        match command {
            "forward" => { horizontal += distance; },
            "down" => { vertical += distance; },
            "up" => { vertical -= distance; },
            _ => { panic!("Unknown command: {}", command); }
        }
    }
    
    vertical * horizontal
}

fn part2(filename: &str) -> i32 {
    let reader = get_file(filename);

    let mut vertical: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut words = line.split_whitespace();
        let command = words.next().unwrap().trim();
        let distance: i32 = words.next().unwrap().parse().expect("Not a valid number");

        match command {
            "forward" => { 
                horizontal += distance;
                vertical += aim * distance;
            },
            "down" => { aim += distance; },
            "up" => { aim -= distance; },
            _ => { panic!("Unknown command: {}", command); }
        }
    }
    
    vertical * horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_sample() {
        assert_eq!(part1("src/days/day2/sample.txt"), 150);
    }

    #[test]
    fn day2_input() {
        assert_eq!(part1("src/days/day2/input.txt"), 1648020);
    }

    #[test]
    fn day2_part_2_sample() {
        assert_eq!(part2("src/days/day2/sample.txt"), 900);
    }

    #[test]
    fn day2_part_2_input() {
        assert_eq!(part2("src/days/day2/input.txt"), 1759818555);
    }
}
