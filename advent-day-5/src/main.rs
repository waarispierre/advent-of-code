use std::fs;

fn main() -> std::io::Result<()> {
    let (rules, orders) = read_data()?;

    let mut sum_of_middle_number = 0;

    for order in orders.iter() {
        let mut decreasing_order = order.clone();
        let mut safe_order = true; //assumee order is true
        for page in order.iter() {
            //step throuugh each order to see if any page combination violates any rule
            for next_page_count in 0..decreasing_order.len() {
                if next_page_count + 1 < decreasing_order.len() {
                    //create a violating rule
                    let condition =
                        vec![decreasing_order[next_page_count + 1] as i32, *page as i32];

                    //check if the violating rule exists
                    let rule_is_violated = rules.iter().any(|entry| *entry == condition);

                    if safe_order && rule_is_violated {
                        //mark order as unsafe
                        safe_order = !rule_is_violated;
                        break;
                    }
                }
            }
            if !safe_order {
                break;
            }
            decreasing_order.remove(0);
        }
        if safe_order {
            let middle_index = (order.len() - 1) / 2;
            sum_of_middle_number += order[middle_index];
        }
    }

    println!("Part one answer: {}", sum_of_middle_number);
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
