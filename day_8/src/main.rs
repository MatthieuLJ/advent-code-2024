use std::cmp;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    read_file(&mut grid);

    let mut processed_satellites: Vec<char> = Vec::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for row in &grid {
        for c in row {
            if *c != '.' && !processed_satellites.contains(c) {
                find_antinodes_for(*c, &mut antinodes, &grid);
                processed_satellites.push(*c);
            }
        }
    }

    println!("Result: {}", antinodes.len());
}

fn find_antinodes_for(c: char, antinodes: &mut HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == c {
                find_antinodes_from(c, (col, row), antinodes, grid);
            }
        }
    }
}

fn find_antinodes_from(
    c: char,
    position: (usize, usize),
    antinodes: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) {
    //finish the current row first
    for col in position.0 + 1..grid[position.0].len() {
        if grid[position.1][col] == c {
            add_antinode(position, (col, position.1), antinodes, grid);
        }
    }
    // then go with the rest of the grid
    for row in position.1 + 1..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == c {
                add_antinode(position, (col, row), antinodes, grid);
            }
        }
    }
}

fn add_antinode(
    position1: (usize, usize),
    position2: (usize, usize),
    antinodes: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) {
    let diff_col = position1.0.abs_diff(position2.0);
    let diff_row = position1.1.abs_diff(position2.1);
    let min_col = cmp::min(position1.0, position2.0);
    let min_row = cmp::min(position1.1, position2.1);
    let max_col = cmp::max(position1.0, position2.0);
    let max_row = cmp::max(position1.1, position2.1);

    let left_antinode_col: isize = min_col as isize - diff_col as isize;
    let right_antinode_col = max_col as isize + diff_col as isize;
    let top_antinode_row: isize = min_row as isize - diff_row as isize;
    let bottom_antinode_row = max_row as isize + diff_row as isize;

    let antinode1_col = if left_antinode_col >= 0 && position1.0 <= position2.0 {
        left_antinode_col
    } else if right_antinode_col <= grid[0].len() as isize - 1 && position1.0 > position2.0 {
        right_antinode_col
    } else {
        -1
    };
    if antinode1_col != -1 {
        let antinode1_row = if top_antinode_row >= 0 && position1.1 <= position2.1 {
            top_antinode_row
        } else if bottom_antinode_row <= grid.len() as isize - 1 && position1.1 > position2.1 {
            bottom_antinode_row
        } else {
            -1
        };

        if antinode1_row != -1 {
            let new_antinode = (antinode1_col as usize, antinode1_row as usize);
            antinodes.insert(new_antinode);
        }
    }

    let antinode2_col = if left_antinode_col >= 0 && position2.0 <= position1.0 {
        left_antinode_col
    } else if right_antinode_col <= grid[0].len() as isize - 1 && position2.0 > position1.0 {
        right_antinode_col
    } else {
        -1
    };
    if antinode2_col != -1 {
        let antinode2_row = if top_antinode_row >= 0 && position2.1 <= position1.1 {
            top_antinode_row
        } else if bottom_antinode_row <= grid.len() as isize - 1 && position2.1 > position1.1 {
            bottom_antinode_row
        } else {
            -1
        };

        if antinode2_row != -1 {
            let new_antinode = (antinode2_col as usize, antinode2_row as usize);
            antinodes.insert(new_antinode);
        }
    }
}

fn read_file(grid: &mut Vec<Vec<char>>) {
    for (row, line) in read_to_string("input.txt")
        .expect("Cannot read file")
        .lines()
        .enumerate()
    {
        grid.push(Vec::new());
        for c in line.chars() {
            grid[row].push(c);
        }
    }
}
