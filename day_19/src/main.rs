use std::fs::read_to_string;

fn main() {
    let input_data = read_to_string("input.txt").expect("Cannot read input files");
    let mut data_lines_iter = input_data.lines();

    let patterns_line = data_lines_iter.next().unwrap();
    let patterns: Vec<&str> = patterns_line.split(",").map(|s| s.trim()).collect();

    data_lines_iter.next(); // skip the empty line

    let mut result: u32 = 0;
    while let Some(towel) = data_lines_iter.next() {
        result += match_towel_patterns(&patterns, towel);
        println!("\r{}", result);
    }

    println!("Result: {}", result);
}

fn match_towel_patterns(patterns: &Vec<&str>, towel: &str) -> u32 {
    if towel == "" {
        return 1;
    }
    let mut result: u32 = 0;
    for p in patterns {
        if let Some(short_towel) = towel.strip_prefix(p) {
            result += match_towel_patterns(patterns, short_towel);
        }
    }
    return result;
}
