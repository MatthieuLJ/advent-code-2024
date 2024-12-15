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

    /////////////////////////////// part 2
    warehouse.clear();
    for l in input.lines() {
        if l == "" {
            break;
        }
        let mut new_line: Vec<char> = Vec::new();
        for c in l.chars() {
            match c {
                '#' => new_line.extend_from_slice(&['#', '#']),
                '.' => new_line.extend_from_slice(&['.', '.']),
                'O' => new_line.extend_from_slice(&['[', ']']),
                '@' => new_line.extend_from_slice(&['@', '.']),
                _ => unreachable!(),
            }
        }
        warehouse.push(new_line);
    }

    //dbg!(&warehouse);

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
            (robot_col, robot_row) = move_robot2(&mut warehouse, (robot_col, robot_row), c);
        }
    }

    let mut result2: usize = 0;
    for r in 0..warehouse.len() {
        for c in 0..warehouse[r].len() {
            if warehouse[r][c] == '[' {
                result2 += 100 * r + c;
            }
        }
    }

    println!("Result2: {}", result2);
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

fn can_move(warehouse: &mut Vec<Vec<char>>, piece: (usize, usize), direction: char) -> bool {
    if warehouse[piece.1][piece.0] == '.' {
        return true;
    }
    let next_position: (usize, usize) = match direction {
        '>' => (piece.0 + 1, piece.1),
        '<' => (piece.0 - 1, piece.1),
        '^' => (piece.0, piece.1 - 1),
        'v' => (piece.0, piece.1 + 1),
        _ => unreachable!(),
    };
    match warehouse[next_position.1][next_position.0] {
        '.' => true,
        '[' => match direction {
            '>' => can_move(warehouse, (piece.0 + 2, piece.1), direction),
            '^' => {
                can_move(warehouse, (piece.0, piece.1 - 1), direction)
                    && can_move(warehouse, (piece.0 + 1, piece.1 - 1), direction)
            }
            'v' => {
                can_move(warehouse, (piece.0, piece.1 + 1), direction)
                    && can_move(warehouse, (piece.0 + 1, piece.1 + 1), direction)
            }
            '<' => can_move(warehouse, (piece.0 - 1, piece.1), direction),
            _ => unreachable!(),
        },
        ']' => match direction {
            '<' => can_move(warehouse, (piece.0 - 2, piece.1), direction),
            '^' => {
                can_move(warehouse, (piece.0, piece.1 - 1), direction)
                    && can_move(warehouse, (piece.0 - 1, piece.1 - 1), direction)
            }
            'v' => {
                can_move(warehouse, (piece.0, piece.1 + 1), direction)
                    && can_move(warehouse, (piece.0 - 1, piece.1 + 1), direction)
            }
            '>' => can_move(warehouse, (piece.0 + 1, piece.1), direction),
            _ => unreachable!(),
        },
        '#' => false,
        _ => unreachable!(),
    }
}

fn do_move(warehouse: &mut Vec<Vec<char>>, piece: (usize, usize), direction: char) {
    if warehouse[piece.1][piece.0] == '.' {
        return;
    }

    let next_position: (usize, usize) = match direction {
        '>' => (piece.0 + 1, piece.1),
        '<' => (piece.0 - 1, piece.1),
        '^' => (piece.0, piece.1 - 1),
        'v' => (piece.0, piece.1 + 1),
        _ => unreachable!(),
    };

    match warehouse[piece.1][piece.0] {
        '@' => {
            do_move(warehouse, next_position, direction);
            warehouse[next_position.1][next_position.0] = '@';
            warehouse[piece.1][piece.0] = '.';
        }
        '[' => match direction {
            '^' => {
                do_move(warehouse, next_position, direction);
                do_move(warehouse, (next_position.0 + 1, next_position.1), direction);
                warehouse[next_position.1][next_position.0] = '[';
                warehouse[next_position.1][next_position.0 + 1] = ']';
                warehouse[piece.1][piece.0] = '.';
                warehouse[piece.1][piece.0 + 1] = '.';
            }
            'v' => {
                do_move(warehouse, next_position, direction);
                do_move(warehouse, (next_position.0 + 1, next_position.1), direction);
                warehouse[next_position.1][next_position.0] = '[';
                warehouse[next_position.1][next_position.0 + 1] = ']';
                warehouse[piece.1][piece.0] = '.';
                warehouse[piece.1][piece.0 + 1] = '.';
            }
            '<' => panic!(),
            '>' => {
                do_move(warehouse, (next_position.0 + 1, next_position.1), direction);
                warehouse[next_position.1][next_position.0 + 1] = ']';
                warehouse[next_position.1][next_position.0] = '[';
                warehouse[piece.1][piece.0] = '.';
            }
            _ => unreachable!(),
        },
        ']' => match direction {
            '^' => {
                do_move(warehouse, next_position, direction);
                do_move(warehouse, (next_position.0 - 1, next_position.1), direction);
                warehouse[next_position.1][next_position.0] = ']';
                warehouse[next_position.1][next_position.0 - 1] = '[';
                warehouse[piece.1][piece.0] = '.';
                warehouse[piece.1][piece.0 - 1] = '.';
            }
            'v' => {
                do_move(warehouse, next_position, direction);
                do_move(warehouse, (next_position.0 - 1, next_position.1), direction);
                warehouse[next_position.1][next_position.0] = ']';
                warehouse[next_position.1][next_position.0 - 1] = '[';
                warehouse[piece.1][piece.0] = '.';
                warehouse[piece.1][piece.0 - 1] = '.';
            }
            '<' => {
                do_move(warehouse, (next_position.0 - 1, next_position.1), direction);
                warehouse[next_position.1][next_position.0 - 1] = '[';
                warehouse[next_position.1][next_position.0] = ']';
                warehouse[piece.1][piece.0] = '.';
            }
            '>' => panic!(),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn move_robot2(
    warehouse: &mut Vec<Vec<char>>,
    piece: (usize, usize),
    direction: char,
) -> (usize, usize) {
    show_warehouse(&warehouse);
    println!("Moving {}", direction);

    if !can_move(warehouse, piece, direction) {
        println!("Can't move");
        return piece;
    }

    do_move(warehouse, piece, direction);

    let next_position: (usize, usize) = match direction {
        '>' => (piece.0 + 1, piece.1),
        '<' => (piece.0 - 1, piece.1),
        '^' => (piece.0, piece.1 - 1),
        'v' => (piece.0, piece.1 + 1),
        _ => unreachable!(),
    };
    next_position
}

fn show_warehouse(warehouse: &Vec<Vec<char>>) {
    for r in warehouse {
        let l: String = r.into_iter().collect();
        println!("{}", l);
    }
}
