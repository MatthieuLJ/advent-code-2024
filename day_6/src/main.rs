use std::fs::read_to_string;

fn main() {
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
