use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn get_increases_part_1(filename: &str) -> i32 {
    let reader = get_file(filename);

    let mut increase_count = 0;
    let mut last_depth = std::i32::MAX;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let depth: i32 = line.trim().parse().expect("Not a valid number");

        if depth > last_depth {
            increase_count += 1
        }

        last_depth = depth;
    }

    println!("Depth increases: {}", increase_count);

    increase_count
}

struct Window {
    a: i32,
    b: i32,
    c: i32,
}

impl Window {
    fn new(a: i32, b: i32, c: i32) -> Window {
        Window { a, b, c }
    }

    fn sum(&self) -> i32 {
        if self.a == std::i32::MAX ||
            self.b == std::i32::MAX ||
            self.c == std::i32::MAX {
                std::i32::MAX
        } else {
            self.a + self.b + self.c
        }
    }

    fn increase(&self, other_window: &Window) -> i32 {
        if other_window.sum() > self.sum() {
            1
        } else {
            0
        }
    }

    fn update_window(&self, new_value: i32) -> Window {
        Window::new(self.b, self.c, new_value)
    }
}

fn get_window_increases_part_2(filename: &str) -> i32 {
    let reader = get_file(filename);

    let mut increase_count = 0;
    let mut last_window = Window::new(std::i32::MAX, std::i32::MAX, std::i32::MAX);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let depth: i32 = line.trim().parse().expect("Not a valid number");

        let new_window = last_window.update_window(depth);

        increase_count += last_window.increase(&new_window);

        last_window = new_window;
    }

    println!("Depth increases: {}", increase_count);

    increase_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(get_increases_part_1("src/days/day1/sample.txt"), 7);
    }

    #[test]
    fn my_input_1() {
        assert_eq!(get_increases_part_1("src/days/day1/myinput.txt"), 1557);
    }

    #[test]
    fn window_sum() {
        let window = Window::new(1, 2, 3);
        assert_eq!(6, window.sum());
    }

    #[test]
    fn sample_2() {
        assert_eq!(get_window_increases_part_2("src/days/day1/sample.txt"), 5);
    }

    #[test]
    fn my_input_2() {
        assert_eq!(get_window_increases_part_2("src/days/day1/myinput.txt"), 5);
    }
}
