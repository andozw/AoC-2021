use crate::common::get_file;
use std::io::BufRead;

// Power consumption = gamma_rate * epsilon_rate.
fn part1(filename: &str) -> i32 {
    let reader = get_file(filename);

    let mut count_map = [0; 12];
    let mut entry_count = 0;

    for (index, line) in reader.lines().enumerate() {
        entry_count +=1;

        let line = line.unwrap();
        
        let mut index = 0;
        
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

    let mut bitmap = [false; 12];
    // Assume no ties.
    // Figure out more idiomatic way to do this.
    for (index, entry) in count_map.clone().iter().enumerate() {
        if most_common_is_1(entry.clone(), entry_count) {
            bitmap[index] = true;
        } else {
            bitmap[index] = false;
        }
    }
    println!("bitmap: {:?}", bitmap);

    let epsilon_bitmap = invert(bitmap.clone());
    let gamma_rate = convert_to_decimal(bitmap); 
    let epsilon_rate = convert_to_decimal(epsilon_bitmap); 

    println!("gamma: {}. epsilon {}", gamma_rate, epsilon_rate);
    gamma_rate * epsilon_rate
}

fn most_common_is_1(number: i32, entry_count: i32) -> bool {
    number > (entry_count / 2)
}

fn convert_to_decimal(bitmap: [bool; 12]) -> i32 {
    // This is so gross. I look forward to writing not clunky rust.
    let mut total = 0;
    let mut pow2 = 2048;
    for item in bitmap {
        if item == true {
            total += pow2
        }
        
        pow2 = pow2 >> 1;
    }

    total
}

fn invert(bitmap: [bool; 12]) -> [bool; 12] {
    let mut inverted_bitmap = [false; 12];

    for (index, entry) in bitmap.iter().enumerate() {
        inverted_bitmap[index] = !entry;
    }

    inverted_bitmap
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
