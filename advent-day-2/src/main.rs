use std::fs;

fn main() -> std::io::Result<()> {
    let data = read_list()?;

    let (_safe_reports_without_dampening, updated_report_data) = part_one(data.clone())?;
    let mut _safe_reports_with_dampening = part_two(updated_report_data.clone())?;

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

fn part_one(data: Vec<Vec<i32>>) -> std::io::Result<(i32, Vec<Vec<i32>>)> {
    let mut _safe_reports = 0;
    let mut indexes_to_remove: Vec<usize> = Vec::new();
    let mut updated_report_data = data.clone();

    let mut _count = 0;
    for report in data.iter() {
        if satisfies_all_rules(report.clone())? {
            _safe_reports += 1;
            indexes_to_remove.push(_count);
        }
        _count += 1;
    }

    indexes_to_remove.sort_by(|a, b| b.cmp(a));

    for remove_index in indexes_to_remove.iter() {
        updated_report_data.remove(*remove_index);
    }
    return Ok((_safe_reports, updated_report_data));
}

fn part_two(data: Vec<Vec<i32>>) -> std::io::Result<i32> {
    let mut _safe_reports = 0;

    for report in data.iter() {
        match report_dampening(report.clone()) {
            Ok(updated_report) => {
                if satisfies_rule_one(updated_report.clone())?
                    && satisfies_rule_two(updated_report.clone())?
                {
                    _safe_reports += 1;
                }
            }
            Err(_) => {}
        }
    }

    return Ok(_safe_reports);
}

fn read_list() -> std::io::Result<Vec<Vec<i32>>> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;
    let mut all_data: Vec<Vec<i32>> = Vec::new();

    for line in input_data.lines() {
        let columns: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect();

        all_data.push(columns);
    }
    return Ok(all_data);
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
