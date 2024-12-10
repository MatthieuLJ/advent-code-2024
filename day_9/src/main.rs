use std::{collections::VecDeque, fs::read_to_string, usize};

fn main() {
    part1();

    part2();
}

#[derive(Clone, Debug, PartialEq)]
enum Space {
    Free,
    File(usize),
}

fn part1() {
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

#[derive(Clone, Debug, PartialEq)]
enum Space2 {
    Free(usize),        // (size)
    File(usize, usize), // (size, fileID)
}

fn part2() {
    let mut disk: VecDeque<Space2> = VecDeque::new();

    let data = read_to_string("input.txt").expect("Cannot read input file");
    let mut file_state = true; // true=file, false=free space
    let mut file_id: usize = 0;
    for c in data.chars() {
        let num = c.to_digit(10).unwrap() as usize;

        if file_state {
            disk.push_back(Space2::File(num, file_id));
            file_state = false;
            file_id += 1;
        } else {
            disk.push_back(Space2::Free(num));
            file_state = true;
        }
    }

    let mut last_file_id_processed = usize::MAX;
    let mut file_size = 0;
    'outer_loop: loop {
        let mut file_index = disk.len() -1;
        while file_index > 0 {
            if let Space2::File(fs, fi) = disk[file_index] {
                if fi < last_file_id_processed {
                    file_size = fs;
                    file_id = fi;
                    break;
                }
            }
            file_index -= 1;
        }
        if file_index == 0 {
            break 'outer_loop;
        }
        last_file_id_processed = file_id;
        let mut free_index = 0;
        let mut free_size=0;
        while free_index < file_index {
            if let Space2::Free(fs) = disk[free_index] {
                free_size = fs;
                if free_size >= file_size {
                    break;
                }
            }
            free_index += 1;
        }
        if free_index == file_index {
            continue;
        }

        disk.remove(file_index);
        disk.insert(file_index, Space2::Free(file_size));

        disk.remove(free_index);
        if free_size > file_size {
            disk.insert(free_index, Space2::Free(free_size-file_size));
        }
        disk.insert(free_index, Space2::File(file_size, file_id));

        
    }

    let mut result: usize = 0;
    let mut disk_index= 0;
    let mut index = 0;
    while disk_index < disk.len() {
        match disk[disk_index] {
            Space2::Free(size) => { index += size; },
            Space2::File(size, id) => {
                for _ in 0..size {
                    result += index * id;
                    index += 1;
                }

            }
        };
        disk_index += 1;
    }

    println!("Result2: {}", result);
}
