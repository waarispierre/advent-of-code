use std::fs;

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;

    Ok(())
}

fn part_one() -> std::io::Result<()> {
    let (left_ids, right_ids) = read_list()?;

    let mut _count = 0;
    let mut total_distance = 0;

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

    println!("Total distance = {}", total_distance);
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let (left_ids, right_ids) = read_list()?;

    let mut similarity_score = 0;
    for left_id in left_ids.iter() {
        let occurence = right_ids
            .iter()
            .filter(|&&right_id| right_id == *left_id)
            .count() as i32;
        similarity_score += occurence * left_id;
    }

    println!("Similarity Score = {}", similarity_score);
    Ok(())
}

fn read_list() -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    let mut left_ids = Vec::new();
    let mut right_ids = Vec::new();

    for line in input_data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let [left, right] = parts.as_slice() {
            left_ids.push(left.parse::<i32>().unwrap());
            right_ids.push(right.parse::<i32>().unwrap());
        }
    }

    left_ids.sort();
    right_ids.sort();

    Ok((left_ids, right_ids))
}
