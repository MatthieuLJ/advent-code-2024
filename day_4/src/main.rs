use std::fs::read_to_string;

fn main() {
    let mut data: Vec<Vec<char>> = Vec::new();

    read_file(&mut data);

    let mut result1: u32 = 0;
    let mut result2: u32 = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if y >= 3 {
                result1 += if search_for_xmas(&data, (x, y), (0, -1)) {
                    1
                } else {
                    0
                };
            }
            if y >= 3 && x < data[y].len() - 3 {
                result1 += if search_for_xmas(&data, (x, y), (1, -1)) {
                    1
                } else {
                    0
                };
            }
            if x < data[y].len() - 3 {
                result1 += if search_for_xmas(&data, (x, y), (1, 0)) {
                    1
                } else {
                    0
                };
            }
            if y < data.len() - 3 && x < data[y].len() - 3 {
                result1 += if search_for_xmas(&data, (x, y), (1, 1)) {
                    1
                } else {
                    0
                };
            }
            if y < data.len() - 3 {
                result1 += if search_for_xmas(&data, (x, y), (0, 1)) {
                    1
                } else {
                    0
                };
            }
            if y < data.len() - 3 && x >= 3 {
                result1 += if search_for_xmas(&data, (x, y), (-1, 1)) {
                    1
                } else {
                    0
                };
            }
            if x >= 3 {
                result1 += if search_for_xmas(&data, (x, y), (-1, 0)) {
                    1
                } else {
                    0
                };
            }
            if y >= 3 && x >= 3 {
                result1 += if search_for_xmas(&data, (x, y), (-1, -1)) {
                    1
                } else {
                    0
                };
            }

            if y >= 1 && y < data.len() - 1 && x >= 1 && x < data[y].len() - 1 {
                result2 += if search_for_x_mas(&data, (x, y)) {
                    1
                } else {
                    0
                }
            }
        }
    }

    println!("Found {} XMAS", result1);
    println!("Found {} X-MAS", result2);
}

fn read_file(data: &mut Vec<Vec<char>>) {
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut new_line: Vec<char> = Vec::new();
        for c in line.chars() {
            new_line.push(c);
        }
        data.push(new_line);
    }
}

fn search_for_xmas(data: &Vec<Vec<char>>, starting: (usize, usize), direction: (i32, i32)) -> bool {
    let (x, y) = starting;
    let (dx, dy) = direction;

    if data[y][x] == 'X'
        && data[(y as i32 + dy) as usize][(x as i32 + dx) as usize] == 'M'
        && data[(y as i32 + 2 * dy) as usize][(x as i32 + 2 * dx) as usize] == 'A'
        && data[(y as i32 + 3 * dy) as usize][(x as i32 + 3 * dx) as usize] == 'S'
    {
        true
    } else {
        false
    }
}

fn search_for_x_mas(data: &Vec<Vec<char>>, starting: (usize, usize)) -> bool {
    let (x, y) = starting;

    if data[y][x] == 'A'
        && ((data[y - 1][x - 1] == 'M' && data[y + 1][x + 1] == 'S')
            || (data[y - 1][x - 1] == 'S' && data[y + 1][x + 1] == 'M'))
        && ((data[y - 1][x + 1] == 'M' && data[y + 1][x - 1] == 'S')
            || (data[y - 1][x + 1] == 'S' && data[y + 1][x - 1] == 'M'))
    {
        true
    } else {
        false
    }
}
