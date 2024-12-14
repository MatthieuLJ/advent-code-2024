use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let mut robots: Vec<(isize, isize, isize, isize)> = Vec::new();
    /*
    let input_data = read_to_string("input_test.txt").expect("Cannot read input file");
    let field_width = 11;
    let field_height = 7;
    */
    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let field_width = 101;
    let field_height = 103;
    

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for r in input_data.lines() {
        let matches = re.captures(&r).unwrap();
        robots.push((
            matches.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            matches.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            matches.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            matches.get(4).unwrap().as_str().parse::<isize>().unwrap(),
        ));
    }

    let mut new_positions: Vec<(isize, isize)> = Vec::new();
    for r in robots {
        new_positions.push((
            (r.0 + 100 * r.2).rem_euclid(field_width),
            (r.1 + 100 * r.3).rem_euclid(field_height),
        ));
    }

    let mut quadrants: [isize; 4] = [0; 4];
    for p in new_positions {
        if p.0 < (field_width - 1) / 2 && p.1 < (field_height - 1) / 2 {
            quadrants[0] += 1;
        } else if p.0 > (field_width - 1) / 2 && p.1 < (field_height - 1) / 2 {
            quadrants[1] += 1;
        } else if p.0 < (field_width - 1) / 2 && p.1 > (field_height - 1) / 2 {
            quadrants[2] += 1;
        } else if p.0 > (field_width - 1) / 2 && p.1 > (field_height - 1) / 2 {
            quadrants[3] += 1;
        }
    }

    println!(
        "Result: {}",
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    );
}

//p=0,4 v=3,-3
