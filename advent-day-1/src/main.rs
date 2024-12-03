use std::fs;

fn main() -> std::io::Result<()> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    let mut left_ids = Vec::new();
    let mut right_ids = Vec::new();

    for line in input_data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Parse the left and right numbers from the parts
        if let [left, right] = parts.as_slice() {
            left_ids.push(left.parse::<i32>().unwrap());
            right_ids.push(right.parse::<i32>().unwrap());
        }
    }

    let mut _count = 0;
    let mut total_distance = 0;

    left_ids.sort();
    right_ids.sort();

    for left_id in left_ids.iter() {
        let right_id = right_ids[_count];
        let difference; 

        if right_id < *left_id {
            difference = left_id - right_id;
        } else {
            difference = right_id - left_id;
        }

        total_distance += difference;
        _count += 1;
    }

    print!("Total distance = {}", total_distance);
    Ok(())
}