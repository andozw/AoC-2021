use crate::common::get_file;
use std::io::BufRead;

// Power consumption = gamma_rate * epsilon_rate.
fn part1(filename: &str) -> u32 {
    let reader = get_file(filename);

    let mut count_map: Vec<u32> = Vec::new();
    let mut line_count = 0;

    for (index, line) in reader.lines().enumerate() {
        line_count += 1;
        let line = line.unwrap();

        let mut index = 0;

        // This is gross. Initialize the count map on the first iteration.
        if count_map.len() == 0 {
            count_map = line.chars().map(|c| 0).collect();
        }

        // This won't work for all string type input.
        for digit in line.chars() {
            match digit {
                '0' => { }, // Do nothing.
                '1' => { count_map[index] += 1; },
                _ => { panic!("Unknown binary digit: {}", digit); }
            }

            index += 1;
        }
    }

    println!("count_map: {:?}", count_map);

    let bitmap: Vec<_> = count_map.iter().map(|&one_count| -> bool {
        one_count > (line_count as u32 / 2)
    }).collect();

    println!("bitmap: {:?}", bitmap);

    let epsilon_bitmap = bitmap.iter().map(|&bit| !bit).collect(); 
    let gamma_rate = convert_to_decimal(bitmap); 
    let epsilon_rate = convert_to_decimal(epsilon_bitmap); 

    println!("gamma: {}. epsilon {}", gamma_rate, epsilon_rate);
    gamma_rate * epsilon_rate
}

fn most_common_is_1(number: u32, line_count: u32) -> bool {
    number > (line_count / 2)
}

fn convert_to_decimal(bitmap: Vec<bool> ) -> u32 {
    // This is so gross. I look forward to writing not clunky rust.
    // TODO: can this be done with a reduce?
    let mut total = 0;
    let mut pow2 = 1;
    for &item in bitmap.iter().rev() {
        if item {
            total += pow2
        }

        pow2 = pow2 << 1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_sample() {
        assert_eq!(part1("src/days/day3/sample.txt"), 198);
    }

    #[test]
    fn day3_input() {
        assert_eq!(part1("src/days/day3/input.txt"),3549854);
    }
}
