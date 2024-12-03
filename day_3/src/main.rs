use std::fs;

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Could not read file");
    let mut result1: u32 = 0;
    let mut result2: u32 = 0;
    let mut disabled = false;

    // https://regexper.com/#%28mul%5C%28%28%5B0-9%5D%7B1%2C3%7D%29%2C%28%5B0-9%5D%7B1%2C3%7D%29%5C%29%7Cdo%5C%28%5C%29%7Cdon't%5C%28%5C%29%29
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
