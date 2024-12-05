use std::fs;

fn main() -> std::io::Result<()> {
    let input_data = read_data()?;
    let word_matrix = create_matrix_from_data(input_data)?;

    let word_to_search: String = ("XMAS").to_string();
    let xmas_occurence_count = calculate_occurence_of_word(word_to_search, word_matrix.clone())?;
    println!("Part one answer: {}", xmas_occurence_count);

    Ok(())
}

fn read_data() -> std::io::Result<Vec<String>> {
    let input_data = fs::read_to_string("src/input_data/input.txt")?;

    let mut data_vector: Vec<String> = Vec::new();
    for line in input_data.lines() {
        data_vector.push(line.to_string());
    }

    return Ok(data_vector);
}

fn create_matrix_from_data(data: Vec<String>) -> std::io::Result<Vec<Vec<String>>> {
    let rows: usize = data.len();
    let cols: usize = data[0].len();
    let mut matrix: Vec<Vec<String>> = Vec::with_capacity(rows);

    for _row in 0..rows {
        let mut row: Vec<String> = Vec::with_capacity(cols);
        for column in 0..cols {
            let letter = data[_row].chars().nth(column).unwrap().to_string();
            row.push(letter);
        }
        matrix.push(row);
    }

    return Ok(matrix);
}

fn calculate_occurence_of_word(
    word: String,
    word_matrix: Vec<Vec<String>>,
) -> std::io::Result<i32> {
    let rows = word_matrix.len().try_into().unwrap();
    let columns = word_matrix[0].len().try_into().unwrap();
    let mut words_found = 0;
    let word_length = word.len();

    for row in 0..rows {
        for col in 0..columns {
            let first_letter: char = word_matrix[row][col].clone().chars().next().unwrap();

            if first_letter == word.chars().next().unwrap() {
                let can_go_north: bool = calculate_if_in_bounds(
                    Direction::North,
                    word_length.try_into().unwrap(),
                    (rows, columns),
                    (row, col),
                )?;
                let can_go_east: bool = calculate_if_in_bounds(
                    Direction::East,
                    word_length.try_into().unwrap(),
                    (rows, columns),
                    (row, col),
                )?;
                let can_go_south: bool = calculate_if_in_bounds(
                    Direction::South,
                    word_length.try_into().unwrap(),
                    (rows, columns),
                    (row, col),
                )?;
                let can_go_west: bool = calculate_if_in_bounds(
                    Direction::West,
                    word_length.try_into().unwrap(),
                    (rows, columns),
                    (row, col),
                )?;

                let can_go_north_west: bool = can_go_north && can_go_west;
                let can_go_north_east: bool = can_go_north && can_go_east;
                let can_go_south_east: bool = can_go_south && can_go_east;
                let can_go_south_west: bool = can_go_south && can_go_west;

                let search_letters = word.clone().split_off(1);
                if can_go_north {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row - count][col]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
                if can_go_east {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row][col + count]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
                if can_go_south {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row + count][col]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
                if can_go_west {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row][col - count]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
                if can_go_north_east {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row - count][col + count]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
                if can_go_south_east {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row + count][col + count]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
                if can_go_south_west {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row + count][col - count]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
                if can_go_north_west {
                    let mut count = 1;
                    for letter in search_letters.chars() {
                        let position_letter = word_matrix[row - count][col - count]
                            .clone()
                            .chars()
                            .next()
                            .unwrap();

                        if letter != position_letter {
                            break;
                        }
                        if count == 3 {
                            words_found += 1;
                        }
                        count += 1;
                    }
                }
            }
        }
    }
    return Ok(words_found);
}

fn calculate_if_in_bounds(
    direction: Direction,
    word_length: i32,
    bounds: (usize, usize),
    location: (usize, usize),
) -> std::io::Result<bool> {
    let location_x_int = location.1 as i32;
    let location_y_int = location.0 as i32;

    let bounds_x_axis = bounds.1 as i32;
    let bounds_y_axis = bounds.0 as i32;

    match direction {
        Direction::North => {
            return Ok(location_y_int >= word_length - 1);
        }
        Direction::East => {
            return Ok(location_x_int <= bounds_x_axis - word_length);
        }
        Direction::South => {
            return Ok(location_y_int <= bounds_y_axis - word_length);
        }
        Direction::West => {
            return Ok(location_x_int >= word_length - 1);
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}
