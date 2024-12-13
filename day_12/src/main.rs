use std::{fs::read_to_string, iter};

fn main() {
    let mut input_data: Vec<Vec<char>> = Vec::new();

    for l in read_to_string("input.txt")
        .expect("Cannot read input file")
        .lines()
    {
        let mut new_line: Vec<char> = Vec::new();
        for c in l.chars() {
            new_line.push(c);
        }
        input_data.push(new_line);
    }

    let mut processed_data: Vec<Vec<bool>> = iter::repeat_with(|| vec![false; input_data[0].len()])
        .take(input_data.len())
        .collect();

    let mut result = 0 as usize;
    let (mut process_col, mut process_row) = (0 as usize, 0 as usize);
    while process_row != input_data.len() {
        if processed_data[process_row][process_col] {
            process_col += 1;
            if process_col == input_data[process_row].len() {
                process_col = 0;
                process_row += 1;
            }
            continue;
        }

        //println!("New starting point {},{}", process_col, process_row);

        let mut area = 0;
        let mut perimeter = 0;
        let area_value = input_data[process_row][process_col];
        let mut next_spots: Vec<(usize, usize)> = Vec::new();
        next_spots.push((process_col, process_row));

        while next_spots.len() > 0 {
            let (new_col, new_row) = next_spots.pop().unwrap();

            if processed_data[new_row][new_col] {
                continue;
            }

            if new_col > 0 {
                if input_data[new_row][new_col - 1] == area_value {
                    if !processed_data[new_row][new_col - 1] {
                        next_spots.push((new_col - 1, new_row));
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
            if new_row > 0 {
                if input_data[new_row - 1][new_col] == area_value {
                    if !processed_data[new_row - 1][new_col] {
                        next_spots.push((new_col, new_row - 1));
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
            if new_col < input_data[new_row].len() - 1 {
                if input_data[new_row][new_col + 1] == area_value {
                    if !processed_data[new_row][new_col + 1] {
                        next_spots.push((new_col + 1, new_row));
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
            if new_row < input_data.len() - 1 {
                if input_data[new_row + 1][new_col] == area_value {
                    if !processed_data[new_row + 1][new_col] {
                        next_spots.push((new_col, new_row + 1));
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }

            area += 1;
            processed_data[new_row][new_col] = true;
        }

        result += area * perimeter;

        process_col += 1;
        if process_col == input_data[process_row].len() {
            process_col = 0;
            process_row += 1;
        }
    }

    println!("Result: {}", result);
}
