use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read input file");
    let mut warehouse: Vec<Vec<char>> = Vec::new();

    for l in input.lines() {
        if l == "" {
            break;
        }
        let new_line: Vec<char> = l.chars().collect();
        warehouse.push(new_line);
    }

    let (mut robot_col, mut robot_row): (usize, usize) = (usize::MAX, usize::MAX);
    for r in 0..warehouse.len() {
        for c in 0..warehouse[r].len() {
            if warehouse[r][c] == '@' {
                (robot_col, robot_row) = (c, r);
            }
        }
    }
    if robot_col == usize::MAX || robot_row == usize::MAX {
        panic!("Could not find the robot in the map");
    }

    let mut past_map = false;
    for l in input.lines() {
        if l == "" {
            past_map = true;
            continue;
        } else if !past_map {
            continue;
        }
        for c in l.chars() {
            (robot_col, robot_row) = move_robot(&mut warehouse, (robot_col, robot_row), c);
        }
    }

    let mut result: usize = 0;
    for r in 0..warehouse.len() {
        for c in 0..warehouse[r].len() {
            if warehouse[r][c] == 'O' {
                result += 100 * r + c;
            }
        }
    }

    println!("Result: {}", result);
}

fn move_robot(
    warehouse: &mut Vec<Vec<char>>,
    piece: (usize, usize),
    direction: char,
) -> (usize, usize) {
    let next_position: (usize, usize) = match direction {
        '>' => (piece.0 + 1, piece.1),
        '<' => (piece.0 - 1, piece.1),
        '^' => (piece.0, piece.1 - 1),
        'v' => (piece.0, piece.1 + 1),
        _ => unreachable!(),
    };
    let move_piece: bool;
    match warehouse[next_position.1][next_position.0] {
        '.' => move_piece = true,
        'O' => move_piece = move_robot(warehouse, next_position, direction) != next_position,
        '#' => return (piece.0, piece.1),
        _ => unreachable!(),
    };
    if move_piece {
        warehouse[next_position.1][next_position.0] = warehouse[piece.1][piece.0];
        warehouse[piece.1][piece.0] = '.';

        next_position
    } else {
        (piece.0, piece.1)
    }
}
