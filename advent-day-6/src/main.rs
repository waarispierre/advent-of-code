use std::fs;
use std::io::{stdout, Write};

fn main() -> std::io::Result<()> {
    let map = read_data_into_matrix()?;

    let number_of_unique_guard_positions = part_one(map.clone())?;
    println!("Part one answer: {:?}", number_of_unique_guard_positions.0);

    let number_of_possibilities = part_two(map.clone())?;
    println!("Part two answer: {:?}", number_of_possibilities);

    Ok(())
}

fn find_current_location(map: Vec<Vec<char>>) -> std::io::Result<((usize, usize), char)> {
    let mut location: (usize, usize) = (0, 0);
    for (row_id, row) in map.iter().enumerate() {
        for (col_id, character) in row.iter().enumerate() {
            if *character != '.' && *character != '#' {
                location = (row_id, col_id);
                break;
            }
        }
        if location != (0, 0) {
            break;
        }
    }

    return Ok((location, map[location.0][location.1]));
}

fn read_data_into_matrix() -> std::io::Result<Vec<Vec<char>>> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    let mut data_vector: Vec<String> = Vec::new();

    for line in input_data.lines() {
        data_vector.push(line.to_string());
    }

    let rows: usize = data_vector.len();
    let cols: usize = data_vector[0].len();
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(rows);

    for _row in 0..rows {
        let mut row: Vec<char> = Vec::with_capacity(cols);
        for column in 0..cols {
            let letter = data_vector[_row].chars().nth(column).unwrap();
            row.push(letter);
        }
        matrix.push(row);
    }

    return Ok(matrix);
}

fn part_one(mut map: Vec<Vec<char>>) -> std::io::Result<(usize, Vec<(usize, usize)>)> {
    let (mut current_location, mut current_direction) = find_current_location(map.clone())?;
    let mut guard_positions: Vec<(usize, usize)> = Vec::new();
    guard_positions.push(current_location);
    let col_limit = map[0].len() - 1;
    let row_limit = map.len() - 1;
    let mut loop_count = 0;
    let possibilities = row_limit * col_limit;

    while current_location.0 < row_limit
        && current_location.0 > 0
        && current_location.1 > 0
        && current_location.1 < col_limit
    {
        loop_count += 1;
        if current_direction == '^' {
            let next_location_char = map[current_location.0 - 1][current_location.1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0, current_location.1 + 1);
                current_direction = '>';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '|';
                current_location = (current_location.0 - 1, current_location.1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == 'v' {
            let next_location_char = map[current_location.0 + 1][current_location.1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0, current_location.1 - 1);
                current_direction = '<';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '|';
                current_location = (current_location.0 + 1, current_location.1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == '>' {
            let next_location_char = map[current_location.0][current_location.1 + 1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0 + 1, current_location.1);
                current_direction = 'v';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '-';
                current_location = (current_location.0, current_location.1 + 1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == '<' {
            let next_location_char = map[current_location.0][current_location.1 - 1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0 - 1, current_location.1);
                current_direction = '^';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '-';
                current_location = (current_location.0, current_location.1 - 1);
                map[current_location.0][current_location.1] = current_direction;
            }
        }

        if !guard_positions.contains(&current_location) {
            guard_positions.push(current_location);
        }

        if (current_location.0 == 0 && current_direction != '^')
            || (current_location.1 == 0 && current_direction != '<')
            || (current_location.1 == map[0].len() - 1 && current_direction != '>')
            || (current_location.0 == map.len() - 1 && current_direction != 'v')
        {
            break;
        }
        if ((current_location.0 == 0 && current_direction != '^')
            || (current_location.1 == 0 && current_direction != '<')
            || (current_location.1 == map[0].len() - 1 && current_direction != '>')
            || (current_location.0 == map.len() - 1 && current_direction != 'v'))
            || loop_count >= possibilities
        {
            return Ok((0, guard_positions));
        }
    }

    return Ok((guard_positions.len(), guard_positions));
}

fn part_two(map: Vec<Vec<char>>) -> std::io::Result<i32> {
    let mut new_maps: Vec<(usize, (usize, usize), Vec<Vec<char>>)> = Vec::new();

    let number_of_locations_reached = find_locations_to_test(map.clone())?;

    let (current_location, _) = find_current_location(map.clone())?;
    let positions = number_of_locations_reached.clone();
    let locations_to_check = positions.len();

    let mut cnt: usize = 1;
    for position in positions.iter() {
        let mut new_map = map.clone();
        new_map[current_location.0][current_location.1] = '.';

        if position.1.0 == current_location.0 && position.1.1 == current_location.1 && position.0 == '^' {
            println!("====={:?}", position);
        } else {
            if position.0 == '^' {
                new_map[position.1 .0][position.1 .1] = '^';
                new_map[position.1 .0 - 1][position.1 .1] = '#';
            } else if position.0 == 'v' {
                new_map[position.1 .0][position.1 .1] = 'v';
                new_map[position.1 .0 + 1][position.1 .1] = '#';
            } else if position.0 == '>' {
                new_map[position.1 .0][position.1 .1] = '>';
                new_map[position.1 .0][position.1 .1 + 1] = '#';
            } else if position.0 == '<' {
                new_map[position.1 .0][position.1 .1] = '<';
                new_map[position.1 .0][position.1 .1 - 1] = '#';
            }
        }
        new_maps.push((cnt, position.1, new_map));
        cnt += 1;
        if cnt == locations_to_check {
            break;
        }
    }

    let mut new_obstruction_count = 0;
    for i in new_maps.iter() {
        let x = part_one(i.2.clone())?;
        if x.0 == 0 {
            new_obstruction_count += 1;

            print!("\r");
            stdout().flush().unwrap();
            print!("{}", new_obstruction_count);
            stdout().flush().unwrap();
        }
    }

    return Ok(new_obstruction_count.try_into().unwrap());
}

fn find_locations_to_test(mut map: Vec<Vec<char>>) -> std::io::Result<Vec<(char, (usize, usize))>> {
    let (mut current_location, mut current_direction) = find_current_location(map.clone())?;
    let mut guard_positions: Vec<(char, (usize, usize))> = Vec::new();
    let col_limit = map[0].len() - 1;
    let row_limit = map.len() - 1;

    while current_location.0 < row_limit
        && current_location.0 > 0
        && current_location.1 > 0
        && current_location.1 < col_limit
    {
        if current_direction == '^' {
            let next_location_char = map[current_location.0 - 1][current_location.1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0, current_location.1 + 1);
                current_direction = '>';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '|';
                current_location = (current_location.0 - 1, current_location.1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == 'v' {
            let next_location_char = map[current_location.0 + 1][current_location.1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0, current_location.1 - 1);
                current_direction = '<';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '|';
                current_location = (current_location.0 + 1, current_location.1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == '>' {
            let next_location_char = map[current_location.0][current_location.1 + 1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0 + 1, current_location.1);
                current_direction = 'v';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '-';
                current_location = (current_location.0, current_location.1 + 1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == '<' {
            let next_location_char = map[current_location.0][current_location.1 - 1];

            if next_location_char == '#' {
                map[current_location.0][current_location.1] = '+';
                current_location = (current_location.0 - 1, current_location.1);
                current_direction = '^';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                map[current_location.0][current_location.1] = '-';
                current_location = (current_location.0, current_location.1 - 1);
                map[current_location.0][current_location.1] = current_direction;
            }
        }

        guard_positions.push((current_direction, current_location));

        if (current_location.0 == 0 && current_direction != '^')
            || (current_location.1 == 0 && current_direction != '<')
            || (current_location.1 == map[0].len() - 1 && current_direction != '>')
            || (current_location.0 == map.len() - 1 && current_direction != 'v')
        {
            break;
        }
    }

    return Ok(guard_positions);
}
