use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut dial_pos = 50;
    let mut end_zero = 0;
    let mut pass_zero = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            // Parse direction and distance correctly
            let direction = &line[0..1];
            let distance: i32 = line[1..].parse::<i32>().unwrap();

            // Count zeros during rotation
            let zeros_during_rotation = count_zeros_during_rotation(dial_pos, direction, distance);
            pass_zero += zeros_during_rotation;

            // Update dial position
            if direction == "R" {
                dial_pos = (dial_pos + distance) % 100;
            } else {
                dial_pos = (dial_pos - distance).rem_euclid(100);
            }
            
            // Check if final position is zero
            if dial_pos == 0 {
                end_zero += 1;
            }
        }
    }

    let password1 = end_zero;
    let password2 = pass_zero + end_zero;

    println!("Password 1: {password1}");
    println!("Password 2: {password2}");
}

fn count_zeros_during_rotation(start_pos: i32, direction: &str, distance: i32) -> i32 {
    let mut count = 0;
    
    if direction == "R" {
        // Right rotation: check positions (start_pos + 1) to (start_pos + distance)
        for step in 1..distance {
            if (start_pos + step) % 100 == 0 {
                count += 1;
            }
        }
    } else {
        // Left rotation: check positions (start_pos - 1) to (start_pos - distance)  
        for step in 1..distance {
            if (start_pos - step).rem_euclid(100) == 0 {
                count += 1;
            }
        }
    }
    
    count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
//pass1 = 1036

//pass2 = 6228