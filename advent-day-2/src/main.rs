use std::fs;

fn main() -> std::io::Result<()> {
    println!("Starting checks");
    read_list()?;
    Ok(())
}

fn read_list() -> std::io::Result<()> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    let mut _safe_reports = 0;
    for line in input_data.lines() {
        let columns: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect();

        let rule_one: bool = satisfies_rule_one(columns.clone())?;
        let rule_two: bool = satisfies_rule_two(columns.clone())?;
        if rule_one && rule_two {
            _safe_reports += 1;
        }
    }

    println!("Number of safe reports: {}", _safe_reports);
    Ok(())
}

fn satisfies_rule_one(data: Vec<i32>) -> std::io::Result<bool> {
    let is_increasing = data[0] < data[1];

    for i in 0..data.len() - 1 {
        if is_increasing {
            if data[i] > data[i + 1] {
                return Ok(false);
            }
        } else {
            if data[i] < data[i + 1] {
                return Ok(false);
            }
        }
    }

    return Ok(true);
}

fn satisfies_rule_two(data: Vec<i32>) -> std::io::Result<bool> {
    let mut _count = 0;
    let mut rule_satisfied: bool = false;
    for i in 0..data.len() - 1 {
        let difference = data[i] - data[i + 1];

        if difference.abs() < 1 || difference.abs() > 3 {
            rule_satisfied = false;
            break;
        } else {
            rule_satisfied = true
        }
    }

    return Ok(rule_satisfied);
}
