use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    let corrupted_data = read_data()?;

    let part_one_answer = part_one(corrupted_data.clone())?;
    let part_two_answer = part_two(corrupted_data.clone())?;

    println!("Part One answer: {:?}", part_one_answer);
    println!("Part Two answer: {:?}", part_two_answer);

    Ok(())
}

fn read_data() -> std::io::Result<String> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    return Ok(input_data);
}

fn part_one(data: String) -> std::io::Result<i32> {
    let cleaned_data: Vec<String> = clean_data(data)?;

    let sum_of_multiplication = calculate_sum_of_multiplication(cleaned_data)?;

    return Ok(sum_of_multiplication);
}

fn part_two(input: String) -> std::io::Result<i32> {
    let parts: Vec<String> = input
        .split("do()")
        .map(|s| s.parse::<String>().unwrap())
        .collect();

    let mut parts_without_dont: Vec<String> = Vec::new();

    for part in parts.iter() {
        if let Some(index) = part.find("don't()") {
            parts_without_dont.push((part[..index]).to_string());
        } else {
            parts_without_dont.push(part.to_string());
        }
    }

    let mut cleaned_data: Vec<String> = Vec::new();

    for part in parts_without_dont.iter() {
        let temp_clean_data: Vec<String> = clean_data(part.to_string())?;
        for temp_data in temp_clean_data.iter() {
            cleaned_data.push(temp_data.to_string());
        }
    }

    let sum_of_multiplication = calculate_sum_of_multiplication(cleaned_data)?;
    return Ok(sum_of_multiplication);
}

fn clean_data(data: String) -> std::io::Result<Vec<String>> {
    let regex_pattern = Regex::new(r"mul\(\d+,\d+\)").expect("Invalid regex pattern");
    let mut cleaned_data: Vec<String> = Vec::new();
    for mat in regex_pattern.find_iter(&data) {
        cleaned_data.push(mat.as_str().replace("mul(", "").replace(")", ""));
    }

    return Ok(cleaned_data);
}

fn calculate_sum_of_multiplication(data: Vec<String>) -> std::io::Result<i32> {
    let mut sum_of_multiplication = 0;
    for line in data.iter() {
        let values: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

        let answer = values[0] * values[1];
        sum_of_multiplication += answer;
    }

    return Ok(sum_of_multiplication);
}
