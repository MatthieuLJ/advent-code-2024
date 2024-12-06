use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut antecedants: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut print_list: Vec<Vec<usize>> = Vec::new();

    read_file(&mut antecedants, &mut print_list);

    //println!("Got antecedants: {:?}", antecedants);
    //println!("Got print_list: {:?}", print_list);

    let mut result: usize = 0;
    let mut result2: usize = 0;

    for mut update in print_list {
        if let Ok(r) = verify_update(&antecedants, &update) {
            result += r;
        } else {
            fix_update(&antecedants, &mut update);
            result2 += update.get((update.len() - 1) / 2).unwrap();
        }
    }
    println!("Result: {}", result);
    println!("Result2: {}", result2);
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

fn verify_update<'a>(
    antecedants: &HashMap<usize, Vec<usize>>,
    update: &'a Vec<usize>,
) -> Result<&'a usize, (usize, usize)> {
    for (index, p) in update.iter().enumerate() {
        for next_index in index + 1..update.len() {
            match antecedants.get(p) {
                None => continue,
                Some(v) => {
                    let next = update.get(next_index).unwrap();
                    if v.contains(next) {
                        return Err((index, next_index));
                    }
                }
            }
        }
    }
    Ok(update.get((update.len() - 1) / 2).unwrap())
}

fn fix_update(antecedants: &HashMap<usize, Vec<usize>>, update: &mut Vec<usize>) {
    let mut index: usize = 0;

    //println!("Fixing {:?}", update);

    // if there is a loop in the antecedants, we're toast!

    'outer: loop {
        let p:&usize = update.get(index).unwrap();

        for next_index in index + 1..update.len() {
            match antecedants.get(p) {
                None => continue,
                Some(v) => {
                    let next = update.get(next_index).unwrap();
                    if v.contains(next) {
                        drop(p);
                        // swap index and next_index and redo the verification
                        let next_value = update.remove(next_index);
                        let previous_value = update.remove(index);
                        update.insert(index, next_value);
                        update.insert(next_index, previous_value);

                        index = 0;
                        continue 'outer;
                    }
                }
            }
        }

        index += 1;
        if index == update.len()-1 {
            //println!("Got {:?}", update);
            return;
        }
    }
}
