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

    antinodes.clear();
    processed_satellites.clear();

    for row in &grid {
        for c in row {
            if *c != '.' && !processed_satellites.contains(c) {
                find_antinodes_for_harmonics(*c, &mut antinodes, &grid);
                processed_satellites.push(*c);
            }
        }
    }

    println!("Result2: {}", antinodes.len());
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
    let diff: (isize, isize) = (
        position2.0 as isize - position1.0 as isize,
        position2.1 as isize - position1.1 as isize,
    );
    if position1.0 as isize - diff.0 >= 0
        && position1.0 as isize - diff.0 < grid[0].len() as isize
        && position1.1 as isize - diff.1 >= 0
        && position1.1 as isize - diff.1 < grid[0].len() as isize
    {
        let new_antinode = (
            (position1.0 as isize - diff.0) as usize,
            (position1.1 as isize - diff.1) as usize,
        );
        antinodes.insert(new_antinode);
    }
    if position2.0 as isize + diff.0 >= 0
        && position2.0 as isize + diff.0 < grid[0].len() as isize
        && position2.1 as isize + diff.1 >= 0
        && position2.1 as isize + diff.1 < grid[0].len() as isize
    {
        let new_antinode = (
            (position2.0 as isize + diff.0) as usize,
            (position2.1 as isize + diff.1) as usize,
        );
        antinodes.insert(new_antinode);
    }
}

fn find_antinodes_for_harmonics(
    c: char,
    antinodes: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == c {
                find_antinodes_from_harmonics(c, (col, row), antinodes, grid);
            }
        }
    }
}

fn find_antinodes_from_harmonics(
    c: char,
    position: (usize, usize),
    antinodes: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) {
    //finish the current row first
    for col in position.0 + 1..grid[position.0].len() {
        if grid[position.1][col] == c {
            add_antinode_harmonics(position, (col, position.1), antinodes, grid);
        }
    }
    // then go with the rest of the grid
    for row in position.1 + 1..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == c {
                add_antinode_harmonics(position, (col, row), antinodes, grid);
            }
        }
    }
}

fn add_antinode_harmonics(
    position1: (usize, usize),
    position2: (usize, usize),
    antinodes: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) {
    let diff: (isize, isize) = (
        position2.0 as isize - position1.0 as isize,
        position2.1 as isize - position1.1 as isize,
    );
    let mut i = 0;
    loop {
        if position1.0 as isize - i * diff.0 >= 0
            && position1.0 as isize - i * diff.0 < grid[0].len() as isize
            && position1.1 as isize - i * diff.1 >= 0
            && position1.1 as isize - i * diff.1 < grid[0].len() as isize
        {
            let new_antinode = (
                (position1.0 as isize - i * diff.0) as usize,
                (position1.1 as isize - i * diff.1) as usize,
            );
            antinodes.insert(new_antinode);
        } else {
            break;
        }
        i += 1;
    }

    i = 0;
    loop {
        if position2.0 as isize + i * diff.0 >= 0
            && position2.0 as isize + i * diff.0 < grid[0].len() as isize
            && position2.1 as isize + i * diff.1 >= 0
            && position2.1 as isize + i * diff.1 < grid[0].len() as isize
        {
            let new_antinode = (
                (position2.0 as isize + i * diff.0) as usize,
                (position2.1 as isize + i * diff.1) as usize,
            );
            antinodes.insert(new_antinode);
        } else {
            break;
        }
        i += 1;
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
