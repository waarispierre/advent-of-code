use std::fs;
use std::thread;

fn main() -> std::io::Result<()> {
    let map = read_data_into_matrix()?;
    let number_of_unique_guard_positions = part_one(map.clone())?;
    println!("Part one answer: {:?}", number_of_unique_guard_positions);

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

fn part_one(mut map: Vec<Vec<char>>) -> std::io::Result<usize> {
    let (mut current_location, mut current_direction) = find_current_location(map.clone())?;
    let mut guard_positions: Vec<(usize, usize)> = Vec::new();
    guard_positions.push(current_location);
    let mut guard_loop_detection_positions: Vec<(usize, usize)> = guard_positions.clone();

    let col_limit = map[0].len() - 1;
    let row_limit = map.len() - 1;
    while current_location.0 < row_limit
        && current_location.0 > 0
        && current_location.1 > 0
        && current_location.1 < col_limit
    {
        if current_direction == '^' {
            let next_location_char = map[current_location.0 - 1][current_location.1];
            map[current_location.0][current_location.1] = '.';

            if next_location_char == '#' {
                current_location = (current_location.0, current_location.1 + 1);
                current_direction = '>';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                current_location = (current_location.0 - 1, current_location.1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == 'v' {
            let next_location_char = map[current_location.0 + 1][current_location.1];
            map[current_location.0][current_location.1] = '.';

            if next_location_char == '#' {
                current_location = (current_location.0, current_location.1 - 1);
                current_direction = '<';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                current_location = (current_location.0 + 1, current_location.1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == '>' {
            let next_location_char = map[current_location.0][current_location.1 + 1];
            map[current_location.0][current_location.1] = '.';

            if next_location_char == '#' {
                current_location = (current_location.0 + 1, current_location.1);
                current_direction = 'v';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                current_location = (current_location.0, current_location.1 + 1);
                map[current_location.0][current_location.1] = current_direction;
            }
        } else if current_direction == '<' {
            let next_location_char = map[current_location.0][current_location.1 - 1];
            map[current_location.0][current_location.1] = '.';

            if next_location_char == '#' {
                current_location = (current_location.0 - 1, current_location.1);
                current_direction = '^';
                map[current_location.0][current_location.1] = current_direction;
            } else {
                current_location = (current_location.0, current_location.1 - 1);
                map[current_location.0][current_location.1] = current_direction;
            }
        }

        if !guard_positions.contains(&current_location) {
            guard_positions.push(current_location);
        } else {
            guard_loop_detection_positions.push(current_location);

            let occurence = guard_loop_detection_positions
                .iter()
                .filter(|&&item| item == current_location)
                .count();
            let prev_element =
                guard_loop_detection_positions[guard_loop_detection_positions.len() - 2];
            let prev_element_occurence = guard_loop_detection_positions
                .iter()
                .filter(|&&item| item == prev_element)
                .count();
            if occurence > 10 && occurence == prev_element_occurence {
                return Ok(0);
            }
        }
    }

    return Ok(guard_positions.len());
}

fn part_two(map: Vec<Vec<char>>) -> std::io::Result<i32> {
    let mut scenarios: Vec<Vec<Vec<char>>> = Vec::new();

    let row_limit = map.len();
    let col_limit = map[0].len();

    for row_count in 0..row_limit {
        for col_count in 0..col_limit {
            let mut new_map = map.clone();
            new_map[row_count][col_count] = '#';
            scenarios.push(new_map);
        }
    }

    let thread_chunk_size = scenarios.len() / 13;

    let mut handles = vec![];
    for chunk in scenarios.chunks(thread_chunk_size) {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || chunk.into_iter().map(part_one).collect::<Vec<_>>());
        handles.push(handle);
    }
    let mut results = vec![];
    for handle in handles {
        results.extend(handle.join().unwrap());
    }

    let new_obstruction_count = results
        .iter()
        .filter(|item| match item {
            Ok(0) => true,
            _ => false,
        })
        .count();

    return Ok(new_obstruction_count.try_into().unwrap());
}
