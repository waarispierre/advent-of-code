use std::fs;

fn main() -> std::io::Result<()> {
    let data = read_data()?;

    let answer_one = part_one(data)?;
    println!("Part One: {}", answer_one);

    Ok(())
}

fn read_data() -> std::io::Result<Vec<Vec<i32>>> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;
    let mut data_vector: Vec<Vec<i32>> = Vec::new();
    for line in input_data.lines() {
        let only_values = line.replace(":", "");
        let vector: Vec<i32> = only_values
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect();
        data_vector.push(vector);
    }

    return Ok(data_vector);
}

fn part_one(data: Vec<Vec<i32>>) -> std::io::Result<i32> {
    for i in data.iter() {
        let combination = generate_combinations(i[1..].to_vec())?;
        for i in combination.iter() {
            let mut result = Vec::new();
            let mut last_pos = 0;
        
            for (i, c) in i.char_indices() {
                if c == '+' || c == '*' {
                    result.push(&i[last_pos..i]);
                    result.push(&c.to_string()); 
                    last_pos = i + 1;
                }
            }
        
            result.push(&expression[last_pos..]); 
            println!("{:?}", result);
        }
    }
    return Ok(0);
}

fn generate_combinations(numbers: Vec<i32>) -> std::io::Result<Vec<String>> {
    let mut combinations = Vec::new();
    if numbers.len() == 1 {
        return Ok(vec![numbers[0].to_string()]);
    }

    for i in 1..numbers.len() {
        let (left, right) = numbers.split_at(i);

        for left_comb in generate_combinations(left.to_vec())? {
            for right_comb in generate_combinations(right.to_vec())? {
                combinations.push(format!(
                    "{}+{}",
                    left_comb.to_string(),
                    right_comb.to_string()
                ));
                combinations.push(format!(
                    "{}*{}",
                    left_comb.to_string(),
                    right_comb.to_string()
                ));
            }
        }
    }

    return Ok(combinations);
}
