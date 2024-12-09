use std::{collections::VecDeque, fs::read_to_string};

#[derive(Clone, Debug, PartialEq)]
enum Space {
    Free,
    File(usize),
}

fn main() {
    let mut disk: VecDeque<Space> = VecDeque::new();

    let data = read_to_string("input.txt").expect("Cannot read input file");

    let mut file_state = true;
    let mut file_id: usize = 0;
    for c in data.chars() {
        let num = c.to_digit(10).unwrap();
        for _ in 0..num {
            if file_state {
                disk.push_back(Space::File(file_id));
            } else {
                disk.push_back(Space::Free);
            }
        }
        if file_state {
            file_state = false;
            file_id += 1;
        } else {
            file_state = true;
        }
    }


    let mut filling_index = 0;
    let mut file_space = Space::Free;
    'outer_loop: loop {
        loop {
            if filling_index >= disk.len() {
                break 'outer_loop;
            }
            if let Space::File(_) = disk[filling_index] {
                filling_index += 1;
                continue;
            }
            break;
        }
        loop {
            if filling_index >= disk.len() - 1 {
                break 'outer_loop;
            }
            let last_space = disk.pop_back();
            if last_space == None {
                break 'outer_loop;
            }
            file_space = last_space.unwrap();
            if let Space::Free = file_space {
                continue;
            }
            break;
        }
        disk[filling_index] = file_space.clone();
    }

    let mut result: usize = 0;
    for i in 0..disk.len() {
        if let Space::File(file_id) = disk[i] {
            result += i * file_id;
        }
    }

    println!("Result: {}", result);
}
