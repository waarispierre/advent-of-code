use std::fs;

fn main() -> std::io::Result<()> {
    let map = read_data_into_matrix()?;
    let number_of_unique_guard_positions = part_one(map.clone())?;
    println!("Part one answer: {:?}", number_of_unique_guard_positions);
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
        if location != (0,0) {
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
    let (mut current_location,mut current_direction) = find_current_location(map.clone())?;
    println!("{} {:?}", current_direction, current_location);
    let mut guard_positions: Vec<(usize, usize)> = Vec::new();
    guard_positions.push(current_location);

    let col_limit = map[0].len() - 1;
    let row_limit = map.len() - 1;

    while current_location.0 < row_limit && current_location.0 > 0 && current_location.1 > 0 && current_location.1 < col_limit {
        if current_direction == '^' {
            let next_location_char = map[current_location.0 - 1][current_location.1];
            map[current_location.0][current_location.1] = '.';

            if next_location_char == '#' {
                current_location = (current_location.0, current_location.1 + 1);
                current_direction = '>';
                map[current_location.0][current_location.1] = current_direction;
            }else{
                current_location = (current_location.0 -1, current_location.1);
                map[current_location.0][current_location.1] = current_direction;
            }    
        } else if current_direction == 'v' {
            let next_location_char = map[current_location.0 + 1][current_location.1];
            map[current_location.0][current_location.1] = '.';

            if next_location_char == '#' {
                current_location = (current_location.0, current_location.1 - 1);
                current_direction = '<';
                map[current_location.0][current_location.1] = current_direction;

            }else{
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

            }else{
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

            }else{
                current_location = (current_location.0, current_location.1 - 1);
                map[current_location.0][current_location.1] = current_direction;

            }    
        }
        guard_positions.push(current_location);
    }
    
    guard_positions.sort_unstable();
    guard_positions.dedup();

    return Ok(guard_positions.len());
}