use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut antecedants: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut print_list: Vec<Vec<usize>> = Vec::new();

    read_file(&mut antecedants, &mut print_list);

    //println!("Got antecedants: {:?}", antecedants);
    //println!("Got print_list: {:?}", print_list);

    let mut result: usize = 0;

    for update in print_list {
        if verify_update(&antecedants, &update) {
            result += update.get((update.len()-1)/2).unwrap();
        }
    }
    println!("Result: {}", result);
}

fn read_file(antecedants: &mut HashMap<usize, Vec<usize>>, print_list: &mut Vec<Vec<usize>>) {
    let file = read_to_string("input.txt").unwrap();
    let mut file_lines = file.lines();
    for line in &mut file_lines {
        if line == "" {
            break;
        }
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() != 2 {
            panic!("Can't read antecedants, got {}", line);
        }
        let before: usize = usize::from_str_radix(parts.get(0).unwrap(), 10).unwrap();
        let after: usize = usize::from_str_radix(parts.get(1).unwrap(), 10).unwrap();
        antecedants
            .entry(after)
            .and_modify(|e| e.push(before))
            .or_insert(vec![before]);
    }

    for line in &mut file_lines {
        let parts: Vec<&str> = line.split(",").collect();
        let mut new_print: Vec<usize> = Vec::new();
        for p in parts {
            new_print.push(usize::from_str_radix(p, 10).unwrap());
        }
        print_list.push(new_print);
    }
}

fn verify_update(antecedants: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    for (index, p) in update.iter().enumerate() {
        for next_index in index + 1..update.len() {
            let next = update.get(next_index).unwrap();
            match antecedants.get(p) {
                None => continue,
                Some(v) => {
                    if v.contains(next) {
                        return false;
                    }
                }
            }
        }
    }
    true
}
