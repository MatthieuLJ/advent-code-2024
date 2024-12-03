use std::fs;

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Could not read file");
    let mut result1: u32 = 0;
    let mut result2: u32 = 0;
    let mut disabled = false;

    let re = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\))").unwrap();
    let matches = re.captures_iter(&content);
    for m in matches {
        
        //println!("Got capture [{}]", &m[1]);

        if &m[1] == "do()" {
            disabled = false;
        } else if &m[1] == "don't()" {
            disabled = true;
        } else {
            let adder = m[2].parse::<u32>().unwrap() * m[3].parse::<u32>().unwrap();
            result1 += adder;
            if !disabled {
                result2 += adder;
            }
        }
    }

    println!("FInal result: (1) {} and(2) {}", result1, result2);
}
