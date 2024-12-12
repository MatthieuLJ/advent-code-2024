use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let mut data: Vec<Vec<u8>> = Vec::new();

    for l in read_to_string("input.txt")
        .expect("Cannot read input file")
        .lines()
    {
        let mut new_line: Vec<u8> = Vec::new();
        for c in l.chars() {
            new_line.push(c.to_digit(10).unwrap() as u8);
        }
        data.push(new_line);
    }

    let mut result: usize = 0;
    let mut result2: usize = 0;

    // go through the data first to find all the 0s
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            if data[row][col] == 0 {
                result += get_score(&mut data, (col, row));
                result2 += get_rating(&mut data, (col, row));
            }
        }
    }

    println!("Result: {}", result);
    println!("Result2: {}", result2);
}

fn get_score(data: &mut Vec<Vec<u8>>, start: (usize, usize)) -> usize {
    let mut paths: [HashSet<(usize, usize)>; 10] = core::array::from_fn(|_| HashSet::new());

    paths[0].insert(start);

    for i in 1..10 as u8 {
        let (previous_level_coords, next_level_coords) = paths.split_at_mut(i as usize);
        for seed in &previous_level_coords[i as usize - 1] {
            let (col, row) = *seed;
            if col > 0 && data[row][col - 1] == i {
                next_level_coords[0].insert((col - 1, row));
            }
            if col < data[row].len() - 1 && data[row][col + 1] == i {
                next_level_coords[0].insert((col + 1, row));
            }
            if row > 0 && data[row - 1][col] == i {
                next_level_coords[0].insert((col, row - 1));
            }
            if row < data.len() - 1 && data[row + 1][col] == i {
                next_level_coords[0].insert((col, row + 1));
            }
        }
    }

    paths[9].len()
}

fn get_rating(data: &mut Vec<Vec<u8>>, start: (usize, usize)) -> usize {
    let mut paths: [Vec<(usize, usize)>; 10] = core::array::from_fn(|_| Vec::new());

    paths[0].push(start);

    for i in 1..10 as u8 {
        let (previous_level_coords, next_level_coords) = paths.split_at_mut(i as usize);
        for seed in &previous_level_coords[i as usize - 1] {
            let (col, row) = *seed;
            if col > 0 && data[row][col - 1] == i {
                next_level_coords[0].push((col - 1, row));
            }
            if col < data[row].len() - 1 && data[row][col + 1] == i {
                next_level_coords[0].push((col + 1, row));
            }
            if row > 0 && data[row - 1][col] == i {
                next_level_coords[0].push((col, row - 1));
            }
            if row < data.len() - 1 && data[row + 1][col] == i {
                next_level_coords[0].push((col, row + 1));
            }
        }
    }

    paths[9].len()
}
