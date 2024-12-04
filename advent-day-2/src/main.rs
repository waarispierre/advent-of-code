use std::fs;

fn main() -> std::io::Result<()> {
    println!("Starting checks");
    read_list()?;
    Ok(())
}

fn read_list() -> std::io::Result<()> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    let mut _safe_reports_without_dampening = 0;
    let mut _safe_reports_with_dampening = 0;

    for line in input_data.lines() {
        let columns: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect();

        if satisfies_all_rules(columns.clone())? {
            _safe_reports_without_dampening += 1;
        } else {
            match report_dampening(columns.clone()) {
                Ok(updated_report) => {
                    if satisfies_rule_one(updated_report.clone())?
                        && satisfies_rule_two(updated_report.clone())?
                    {
                        _safe_reports_with_dampening += 1;
                    }
                }
                Err(_) => {}
            }
        }
    }

    let safe_reports = _safe_reports_with_dampening + _safe_reports_without_dampening;

    println!(
        "Number of safe reports without dampening: {}",
        _safe_reports_without_dampening
    );
    println!(
        "Number of safe reports with dampening: {}",
        _safe_reports_with_dampening
    );
    println!("Number of safe reports: {}", safe_reports);
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

fn report_dampening(data: Vec<i32>) -> std::io::Result<Vec<i32>> {
    for i in 0..data.len() {
        let mut test_data = data.clone();

        test_data.remove(i);

        if satisfies_all_rules(test_data.clone())? {
            return Ok(test_data);
        }
    }

    return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "could not dampen report",
    ));
}

fn satisfies_all_rules(data: Vec<i32>) -> std::io::Result<bool> {
    return Ok(satisfies_rule_one(data.clone())? && satisfies_rule_two(data.clone())?);
}
