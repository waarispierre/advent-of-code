use std::fs;

fn main() -> std::io::Result<()> {
    let (rules, orders) = read_data()?;

    let (part_one_answer, unsafe_orders) = part_one(rules.clone(), orders.clone())?;
    let part_two_answer = part_two(rules.clone(), unsafe_orders.clone())?;

    println!("Part one answer: {}", part_one_answer);
    println!("Part two answer: {}", part_two_answer);
    Ok(())
}

fn read_data() -> std::io::Result<(Vec<Vec<i32>>, Vec<Vec<i32>>)> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    let split_rules_from_orders: Vec<String> =
        input_data.split("\n\n").map(|s| s.to_string()).collect();

    let rules: Vec<Vec<i32>> = split_rules_from_orders[0]
        .lines()
        .map(|line| {
            line.split('|')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let orders: Vec<Vec<i32>> = split_rules_from_orders[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    return Ok((rules, orders));
}

fn part_one(rules: Vec<Vec<i32>>, orders: Vec<Vec<i32>>) -> std::io::Result<(i32, Vec<Vec<i32>>)> {
    let mut sum_of_middle_number = 0;
    let mut unsafe_orders: Vec<Vec<i32>> = Vec::new();

    for order in orders.iter() {
        let safe_order = is_safe_order(order.to_vec(), rules.clone())?;

        if safe_order {
            let middle_index = (order.len() - 1) / 2;
            sum_of_middle_number += order[middle_index];
        } else {
            unsafe_orders.push(order.to_vec());
        }
    }

    return Ok((sum_of_middle_number, unsafe_orders));
}

fn part_two(rules: Vec<Vec<i32>>, orders: Vec<Vec<i32>>) -> std::io::Result<i32> {
    let mut sum_of_middle_number = 0;
    let mut counter = 0;

    for order in orders.iter() {
        let mut reordered_order = order.clone();
        let mut retry = true;
        counter += 1;

        while retry {
            let mut count = 0;
            let iteration_order = reordered_order.clone();
            println!(
                "{} out of {} => {:?}",
                counter,
                orders.len(),
                iteration_order
            );
            for page in iteration_order.iter() {
                for next_page_count in 0..order.len() {
                    if next_page_count + count < order.len() {
                        let condition = vec![
                            iteration_order[next_page_count + count] as i32,
                            *page as i32,
                        ];
                        let rule_is_violated = rules.iter().any(|entry| *entry == condition);

                        if rule_is_violated {
                            reordered_order.swap(count, next_page_count + count);
                        }

                        //check if the reordered list is safe or not
                        retry = !is_safe_order(reordered_order.to_vec(), rules.clone())?;
                        if !retry {
                            println!("now safe");
                            break;
                        }
                    }
                }
                if !retry {
                    break;
                }
                count += 1;
            }
        }
        let middle_index = (reordered_order.len() - 1) / 2;
        sum_of_middle_number += reordered_order[middle_index];
    }

    return Ok(sum_of_middle_number);
}

fn is_safe_order(order: Vec<i32>, rules: Vec<Vec<i32>>) -> std::io::Result<bool> {
    let mut count = 0;
    let mut safe_order = true; //assumee order is true

    for page in order.iter() {
        //step throuugh each order to see if any page combination violates any rule
        for next_page_count in 0..order.len() {
            if next_page_count + count < order.len() {
                //create a violating rule
                let condition = vec![order[next_page_count + count] as i32, *page as i32];

                //check if the violating rule exists
                let rule_is_violated = rules.iter().any(|entry| *entry == condition);

                if safe_order && rule_is_violated {
                    //mark order as unsafe
                    safe_order = !rule_is_violated;
                    break;
                }
            }
        }
        count += 1;
    }

    return Ok(safe_order);
}
