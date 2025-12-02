use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut num = 50;
    let mut password = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("Starting at {num}");
            let turn: i32 = line[1..line.len()].parse::<i32>().unwrap() % 100;
            if &line[0..1] == "R" {
                num = (num + turn) % 100;
                println!("Turning right {turn} to {num}");
            } else {
                num = (num + 100 - turn) % 100;
                println!("Turning left {turn} to {num}");
            }
            if num == 0 {
                password = password + 1;
            }
        }
    }

    println!("{password}");
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}