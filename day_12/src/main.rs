use std::{collections::HashSet, fs::read_to_string, iter};

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

    let mut processed_data: Vec<Vec<u8>> = iter::repeat_with(|| vec![0; input_data[0].len()])
        .take(input_data.len())
        .collect();

    let mut result = 0 as usize;
    let mut result2 = 0 as usize;
    let (mut process_col, mut process_row) = (0 as usize, 0 as usize);
    while process_row != input_data.len() {
        if processed_data[process_row][process_col] != 0 {
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

            if processed_data[new_row][new_col] != 0 {
                continue;
            }

            if new_col > 0 && input_data[new_row][new_col - 1] == area_value {
                if processed_data[new_row][new_col - 1] == 0 {
                    next_spots.push((new_col - 1, new_row));
                }
            } else {
                perimeter += 1;
            }
            if new_row > 0 && input_data[new_row - 1][new_col] == area_value {
                if processed_data[new_row - 1][new_col] == 0 {
                    next_spots.push((new_col, new_row - 1));
                }
            } else {
                perimeter += 1;
            }
            if new_col < input_data[new_row].len() - 1
                && input_data[new_row][new_col + 1] == area_value
            {
                if processed_data[new_row][new_col + 1] == 0 {
                    next_spots.push((new_col + 1, new_row));
                }
            } else {
                perimeter += 1;
            }
            if new_row < input_data.len() - 1 && input_data[new_row + 1][new_col] == area_value {
                if processed_data[new_row + 1][new_col] == 0 {
                    next_spots.push((new_col, new_row + 1));
                }
            } else {
                perimeter += 1;
            }

            area += 1;
            processed_data[new_row][new_col] = 2;
        }

        //dbg!(area_value);
        let num_sides = get_number_sides(&processed_data);

        result += area * perimeter;
        result2 += area * num_sides;

        process_col += 1;
        if process_col == input_data[process_row].len() {
            process_col = 0;
            process_row += 1;
        }

        for row in 0..processed_data.len() {
            for col in 0..processed_data[row].len() {
                if processed_data[row][col] == 2 {
                    processed_data[row][col] = 1;
                }
            }
        }
    }

    println!("Result: {}", result);
    println!("Result2: {}", result2);
}

// the array will have '2's for only the area we care about
fn get_number_sides(area: &Vec<Vec<u8>>) -> usize {
    // first get the "vertical" sides

    // the tuple is (coord, entering_the_area), coord goes from 0 for the left edge of the grid to the right edge...
    let mut edges: Vec<HashSet<(usize, bool)>> = Vec::new();
    let mut in_area: bool;
    for row in 0..area.len() {
        in_area = false;
        let mut edges_row: HashSet<(usize, bool)> = HashSet::new();
        for col in 0..area[row].len() {
            if (in_area && area[row][col] != 2) || (!in_area && area[row][col] == 2) {
                in_area = !in_area;
                edges_row.insert((col, in_area));
                if row > 0 {
                    edges[row - 1].remove(&(col, in_area));
                }
            }
        }
        if area[row][area[row].len() - 1] == 2 {
            edges_row.insert((area[row].len(), false));
            if row > 0 {
                edges[row - 1].remove(&(area[row].len(), false));
            }
        }
        edges.push(edges_row);
    }

    let mut result = 0;
    for r in &edges {
        result += r.len();
    }

    edges.clear();

    for col in 0..area[0].len() {
        in_area = false;
        let mut edges_col: HashSet<(usize, bool)> = HashSet::new();
        for row in 0..area.len() {
            if (in_area && area[row][col] != 2) || (!in_area && area[row][col] == 2) {
                in_area = !in_area;
                edges_col.insert((row, in_area));
                if col > 0 {
                    edges[col - 1].remove(&(row, in_area));
                }
            }
        }
        if area[area.len() - 1][col] == 2 {
            edges_col.insert((area[col].len(), false));
            if col > 0 {
                edges[col - 1].remove(&(area[col].len(), false));
            }
        }
        edges.push(edges_col);
    }

    for c in &edges {
        result += c.len();
    }

    result
}
