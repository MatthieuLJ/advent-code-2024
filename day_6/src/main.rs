use std::fs::read_to_string;

fn main() {
    part1();

    part2();
}

fn part1() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut position = read_file(&mut grid);
    let mut direction: (isize, isize) = (0, -1);

    loop {
        grid[position.1][position.0] = 'X';

        if (position.0 == 0 && direction.0 < 0)
            || (position.1 == 0 && direction.1 < 0)
            || (position.0 == grid[0].len() - 1 && direction.0 > 0)
            || (position.1 == grid.len() - 1 && direction.1 > 0)
        {
            // we are leaving the map
            break;
        }

        let next_position: (usize, usize) = (
            (position.0 as isize + direction.0) as usize,
            (position.1 as isize + direction.1) as usize,
        );

        if grid[next_position.1][next_position.0] == '#' {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => {
                    panic!("I'm lost");
                }
            };
            continue;
        }
        position = next_position;
    }

    let mut result: usize = 0;

    for row in &grid {
        result += row.iter().filter(|&v| *v == 'X').count();
        //println!("{:?}", row);
    }

    println!("Result: {}", result);
}

// returns the starting position
fn read_file(grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    let mut result: (usize, usize) = (0, 0);
    for (row, line) in read_to_string("input.txt")
        .expect("Cannot read file")
        .lines()
        .enumerate()
    {
        grid.push(Vec::new());
        for (col, c) in line.chars().enumerate() {
            grid[row].push(c);
            if c == '^' {
                result = (col, row);
            }
        }
    }
    result
}

fn part2() {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut position = read_file2(&mut grid);
    let mut direction: (isize, isize) = (0, -1);
    let mut direction_value: usize = 1; // what we will write on the map. 1=up, 2=right, 4=down, 8=left
    let mut change_direction = true; // to trace a line behind
    let mut result: usize = 0;

    update_for_traps(&mut grid);
    update_for_traps_missing_top_left(&mut grid);
    update_for_traps_missing_top_right(&mut grid);
    update_for_traps_missing_bottom_right(&mut grid);
    update_for_traps_missing_bottom_left(&mut grid);

    loop {
        grid[position.1][position.0] |= direction_value;

        if change_direction {
            // backtrack to pretend there is also a trace behind us in the same
            // direction. The loop could be caught from before
            let mut trace_position = position;
            loop {
                if (trace_position.0 == 0 && direction.0 > 0)
                    || (trace_position.1 == 0 && direction.1 > 0)
                    || (trace_position.0 == grid[0].len() - 1 && direction.0 < 0)
                    || (trace_position.1 == grid.len() - 1 && direction.1 < 0)
                {
                    // we are leaving the map
                    break;
                }

                trace_position = (
                    (trace_position.0 as isize - direction.0) as usize,
                    (trace_position.1 as isize - direction.1) as usize,
                );

                if grid[trace_position.1][trace_position.0] == 255 {
                    break;
                }

                grid[trace_position.1][trace_position.0] |= direction_value;
            }
            change_direction = false;
        }

        let next_direction_value = match direction_value {
            1 => 2,
            2 => 4,
            4 => 8,
            8 => 1,
            _ => {
                panic!("That can't be");
            }
        };

        if grid[position.1][position.0] & next_direction_value != 0 {
            result += 1;
        }

        if (position.0 == 0 && direction.0 < 0)
            || (position.1 == 0 && direction.1 < 0)
            || (position.0 == grid[0].len() - 1 && direction.0 > 0)
            || (position.1 == grid.len() - 1 && direction.1 > 0)
        {
            // we are leaving the map
            break;
        }

        let next_position: (usize, usize) = (
            (position.0 as isize + direction.0) as usize,
            (position.1 as isize + direction.1) as usize,
        );

        if grid[next_position.1][next_position.0] == 255 {
            (direction, direction_value) = match direction {
                (0, -1) => ((1, 0), 2),
                (1, 0) => ((0, 1), 4),
                (0, 1) => ((-1, 0), 8),
                (-1, 0) => ((0, -1), 1),
                _ => {
                    panic!("I'm lost");
                }
            };
            change_direction = true;
            continue;
        }
        position = next_position;
    }

    /*
    for row in &grid {
        println!("{:?}", row);
    }
     */

    println!("Result2: {}", result);
}

// returns the starting position
fn read_file2(grid: &mut Vec<Vec<usize>>) -> (usize, usize) {
    let mut result: (usize, usize) = (0, 0);
    for (row, line) in read_to_string("input.txt")
        .expect("Cannot read file")
        .lines()
        .enumerate()
    {
        grid.push(Vec::new());
        for (col, c) in line.chars().enumerate() {
            let new_value = match c {
                '.' => 0,
                '^' => 1,
                '#' => 255,
                _ => {
                    panic!("found some junk in the map");
                }
            };
            grid[row].push(new_value);
            if c == '^' {
                result = (col, row);
            }
        }
    }
    result
}

/*
    We need to find "traps", those will look either like
    . . . . . . .
    . . # . . . .
    . . . . . # .
    . . . . . . .
    . . . . . . .
    . # . . . . .
    . . . . # . .
    . . . . . . .

    in this case, we need to prep the grid with those traces as if the guard
    had gone through there already:
    . . . . ↓ . .
    . . # . ↓ . .
    → → → → ↓ # .
    . . ↑ . ↓ . .
    . . ↑ . ↓ . .
    . # ↑ ← ← ← ←
    . . ↑ . # . .
    . . ↑ . . . .
*/
fn update_for_traps(grid: &mut Vec<Vec<usize>>) {
    for top_left_row in 0..grid.len() - 2 {
        'cursor: for top_left_col in 1..grid[top_left_row].len() - 1 {
            if grid[top_left_row][top_left_col] == 255 {
                let mut cursor: (usize, usize) = (top_left_col, top_left_row + 1);
                loop {
                    if cursor.0 == grid[cursor.1].len() - 1 {
                        continue 'cursor;
                    }
                    if grid[cursor.1][cursor.0 + 1] == 255 {
                        break;
                    }
                    cursor.0 += 1;
                }
                let right_edge = cursor.0;
                loop {
                    if cursor.1 == grid.len() - 1 {
                        continue 'cursor;
                    }
                    if grid[cursor.1 + 1][cursor.0] == 255 {
                        break;
                    }
                    cursor.1 += 1;
                }
                let bottom_edge = cursor.1;
                loop {
                    if cursor.0 == 0 {
                        continue 'cursor;
                    }
                    if grid[cursor.1][cursor.0 - 1] == 255 {
                        break;
                    }
                    cursor.0 -= 1;
                }
                loop {
                    if cursor.1 == 0 {
                        continue 'cursor;
                    }
                    if grid[cursor.1 - 1][cursor.0] == 255 {
                        break;
                    }
                    cursor.1 -= 1;
                }
                if cursor == (top_left_col, top_left_row + 1) {
                    // we made it and found a loop
                    cursor = (right_edge - 1, top_left_row + 1);
                    loop {
                        grid[cursor.1][cursor.0] |= 2;
                        if cursor.0 == 0 || grid[cursor.1][cursor.0 - 1] == 255 {
                            break;
                        }
                        cursor.0 -= 1;
                    }
                    cursor = (right_edge, bottom_edge - 1);
                    loop {
                        grid[cursor.1][cursor.0] |= 4;
                        if cursor.1 == 0 || grid[cursor.1 - 1][cursor.0] == 255 {
                            break;
                        }
                        cursor.1 -= 1;
                    }
                    cursor = (top_left_col + 1, bottom_edge);
                    loop {
                        grid[cursor.1][cursor.0] |= 8;
                        if cursor.0 == grid[cursor.1].len() - 1
                            || grid[cursor.1][cursor.0 + 1] == 255
                        {
                            break;
                        }
                        cursor.0 += 1;
                    }
                    cursor = (top_left_col, top_left_row + 2);
                    loop {
                        grid[cursor.1][cursor.0] |= 1;
                        if cursor.1 == grid.len() - 1 || grid[cursor.1 + 1][cursor.0] == 255 {
                            break;
                        }
                        cursor.1 += 1;
                    }
                }
            }
        }
    }
}

/*
    it's also possible one of those is missing
    . . . . . . .
    . . . . . . .
    . . . . . # .
    . . . . . . .
    . . . . . . .
    . # . . . . .
    . . . . # . .
    . . . . . . .

    then we need to update like this:
    . . . . . . .
    . . . . . . .
    . . → . . # .
    . . . . . . .
    . . . . . . .
    . # . . . . .
    . . . . # . .
    . . . . . . .

*/
fn update_for_traps_missing_top_left(grid: &mut Vec<Vec<usize>>) {
    for top_right_row in 1..grid.len() - 2 {
        'cursor: for top_right_col in 2..grid[top_right_row].len() - 1 {
            if grid[top_right_row][top_right_col] == 255 {
                let mut cursor: (usize, usize) = (top_right_col - 1, top_right_row);
                if grid[cursor.1][cursor.0] & 4 != 0 {
                    continue 'cursor; // this is already part of a loop
                }
                loop {
                    if cursor.1 == grid.len() - 1 {
                        continue 'cursor;
                    }
                    if grid[cursor.1 + 1][cursor.0] == 255 {
                        break;
                    }
                    cursor.1 += 1;
                }
                loop {
                    if cursor.0 == 0 {
                        continue 'cursor;
                    }
                    if grid[cursor.1][cursor.0 - 1] == 255 {
                        break;
                    }
                    cursor.0 -= 1;
                }
                loop {
                    if cursor.1 == top_right_row {
                        break;
                    }
                    if grid[cursor.1 - 1][cursor.0] == 255 {
                        continue 'cursor;
                    }
                    cursor.1 -= 1;
                }
                grid[cursor.1][cursor.0] |= 2;
            }
        }
    }
}

/*
    it's also possible one of those is missing
    . . . . . . .
    . . # . . . .
    . . . . . . .
    . . . . . . .
    . . . . . . .
    . # . . . . .
    . . . . # . .
    . . . . . . .

    then we need to update like this:
    . . . . . . .
    . . # . . . .
    . . . . ↓ . .
    . . . . . . .
    . . . . . . .
    . # . . . . .
    . . . . # . .
    . . . . . . .

*/
fn update_for_traps_missing_top_right(grid: &mut Vec<Vec<usize>>) {
    for bottom_right_row in 2..grid.len() - 1 {
        'cursor: for bottom_right_col in 1..grid[bottom_right_row].len() - 2 {
            if grid[bottom_right_row][bottom_right_col] == 255 {
                let mut cursor: (usize, usize) = (bottom_right_col, bottom_right_row - 1);
                if grid[cursor.1][cursor.0] & 8 != 0 {
                    continue 'cursor; // this is already part of a loop
                }
                loop {
                    if cursor.0 == 0 {
                        continue 'cursor;
                    }
                    if grid[cursor.1][cursor.0 - 1] == 255 {
                        break;
                    }
                    cursor.0 -= 1;
                }
                loop {
                    if cursor.1 == 0 {
                        continue 'cursor;
                    }
                    if grid[cursor.1 - 1][cursor.0] == 255 {
                        break;
                    }
                    cursor.1 -= 1;
                }
                loop {
                    if cursor.0 == bottom_right_col {
                        break;
                    }
                    if grid[cursor.1][cursor.0 + 1] == 255 {
                        continue 'cursor;
                    }
                    cursor.0 += 1;
                }
                grid[cursor.1][cursor.0] |= 4;
            }
        }
    }
}

/*
    it's also possible one of those is missing
    . . . . . . .
    . . # . . . .
    . . . . . # .
    . . . . . . .
    . . . . . . .
    . # . . . . .
    . . . . . . .
    . . . . . . .

    then we need to update like this:
    . . . . . . .
    . . # . . . .
    . . . . . # .
    . . . . . . .
    . . . . . . .
    . # . . ← . .
    . . . . . . .
    . . . . . . .

*/
fn update_for_traps_missing_bottom_right(grid: &mut Vec<Vec<usize>>) {
    for bottom_left_row in 1..grid.len() - 2 {
        'cursor: for bottom_left_col in 0..grid[bottom_left_row].len() - 3 {
            if grid[bottom_left_row][bottom_left_col] == 255 {
                let mut cursor: (usize, usize) = (bottom_left_col + 1, bottom_left_row);
                if grid[cursor.1][cursor.0] & 1 != 0 {
                    continue 'cursor; // this is already part of a loop
                }
                loop {
                    if cursor.1 == 0 {
                        continue 'cursor;
                    }
                    if grid[cursor.1 - 1][cursor.0] == 255 {
                        break;
                    }
                    cursor.1 -= 1;
                }
                loop {
                    if cursor.0 == grid[cursor.1].len() - 1 {
                        continue 'cursor;
                    }
                    if grid[cursor.1][cursor.0 + 1] == 255 {
                        break;
                    }
                    cursor.0 += 1;
                }
                loop {
                    if cursor.1 == bottom_left_row {
                        break;
                    }
                    if grid[cursor.1 + 1][cursor.0] == 255 {
                        continue 'cursor;
                    }
                    cursor.1 += 1;
                }
                grid[cursor.1][cursor.0] |= 8;
            }
        }
    }
}

/*
    it's also possible one of those is missing
    . . . . . . .
    . . # . . . .
    . . . . . # .
    . . . . . . .
    . . . . . . .
    . . . . . . .
    . . . . # . .
    . . . . . . .

    then we need to update like this:
    . . . . . . .
    . . # . . . .
    . . . . . # .
    . . . . . . .
    . . . . . . .
    . . ↑ . . . .
    . . . . # . .
    . . . . . . .

*/
fn update_for_traps_missing_bottom_left(grid: &mut Vec<Vec<usize>>) {
    for top_left_row in 0..grid.len() - 3 {
        'cursor: for top_left_col in 1..grid[top_left_row].len() - 2 {
            if grid[top_left_row][top_left_col] == 255 {
                let mut cursor: (usize, usize) = (top_left_col, top_left_row + 1);
                if grid[cursor.1][cursor.0] & 2 != 0 {
                    continue 'cursor; // this is already part of a loop
                }
                loop {
                    if cursor.0 == grid[cursor.1].len() - 1 {
                        continue 'cursor;
                    }
                    if grid[cursor.1][cursor.0 + 1] == 255 {
                        break;
                    }
                    cursor.0 += 1;
                }
                loop {
                    if cursor.1 == grid.len() - 1 {
                        continue 'cursor;
                    }
                    if grid[cursor.1 + 1][cursor.0] == 255 {
                        break;
                    }
                    cursor.1 += 1;
                }
                loop {
                    if cursor.0 == top_left_col {
                        break;
                    }
                    if grid[cursor.1][cursor.0 - 1] == 255 {
                        continue 'cursor;
                    }
                    cursor.0 -= 1;
                }

                grid[cursor.1][cursor.0] |= 1;
            }
        }
    }
}
