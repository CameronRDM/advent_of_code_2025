use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let ranges: Vec<&str> = input.split(",").collect();

    let mut id_total = 0;

    for range in ranges {
        //let start_range: i32 = range.split("-").collect()[0]
        let range_vector: Vec<&str> = range.split("-").collect();
        let min_range: i64 = range_vector[0].parse::<i64>().unwrap();
        let max_range: i64 = range_vector[1].parse::<i64>().unwrap();

        for i in min_range..max_range {
            let i_string = i.to_string();
            let cand: &str = i_string.as_str();
            if is_symmetrical(cand) {
                id_total = id_total + i;
            }
        }
    }

    println!("{id_total}");
}

fn is_symmetrical(id: &str) -> bool {
    if id.len() % 2 == 0 {
        let length = id.len();
        let mid: usize = length / 2;
        let first_half: &str = &id[0..mid];
        let second_half: &str = &id[mid..length];
        if first_half == second_half {
            return true;
        }
    }

    return false;
}
