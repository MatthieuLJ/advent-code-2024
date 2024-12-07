use std::fs::read_to_string;

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let starting_position = read_file(&mut grid);
    let mut position = (starting_position.0, starting_position.1);
    let mut direction: (isize, isize) = (0, -1);

    loop {
        grid[position.1][position.0] = 'X';

        if !move_to_next_position(&grid, &mut position, &mut direction) {
            break;
        }
    }

    let mut result: usize = 0;

    for row in &grid {
        result += row.iter().filter(|&v| *v == 'X').count();
        //println!("{:?}", row);
    }

    let mut result2: usize = 0;

    for row_index in 0..grid.len()  {
        for col_index in 0..grid[row_index].len()  {
            if grid[row_index][col_index] == 'X' {
                grid[row_index][col_index] = '#';
                if check_for_loop(&mut grid, &starting_position) {
                    grid[row_index][col_index] = 'O';
                    result2 += 1;
                } else {
                    grid[row_index][col_index] = 'X';
                }
            }
        }
    }

    for row in &grid {
        println!("{:?}", row);
    }

    println!("Result: {}", result);
    println!("Result2: {}", result2);
}

fn check_for_loop(grid: &mut Vec<Vec<char>>, starting_position: &(usize, usize)) -> bool {
    let mut position1 = starting_position.clone();
    let mut direction1: (isize, isize) = (0, -1);
    let mut position2 = starting_position.clone();
    let mut direction2: (isize, isize) = (0, -1);

    loop {
        if !move_to_next_position(&grid, &mut position1, &mut direction1) {
            return false;
        }
        if !move_to_next_position(&grid, &mut position2, &mut direction2) {
            return false;
        }
        if !move_to_next_position(&grid, &mut position2, &mut direction2) {
            return false;
        }
        if position1 == position2 && direction1 == direction2 {
            return true;
        }
    }
}

fn move_to_next_position(
    grid: &Vec<Vec<char>>,
    position: &mut (usize, usize),
    direction: &mut (isize, isize),
) -> bool {
    if (position.0 == 0 && direction.0 < 0)
        || (position.1 == 0 && direction.1 < 0)
        || (position.0 == grid[0].len() - 1 && direction.0 > 0)
        || (position.1 == grid.len() - 1 && direction.1 > 0)
    {
        // we are leaving the map
        return false;
    }

    let next_position: (usize, usize) = (
        (position.0 as isize + direction.0) as usize,
        (position.1 as isize + direction.1) as usize,
    );

    if grid[next_position.1][next_position.0] == '#' {
        *direction = match direction {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => {
                panic!("I'm lost");
            }
        };
        if !move_to_next_position(grid, position, direction) {
            return false;
        }
    } else {
        *position = next_position;
    }
    return true;
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
