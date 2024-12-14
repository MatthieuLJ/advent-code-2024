extern crate termion;

use k_board::{keyboard::Keyboard, keys::Keys};
use regex::Regex;
use std::fs::read_to_string;
use std::time::SystemTime;

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
    for r in &robots {
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

    let mut k: isize = 88;
    let mut pause: bool = false;
    let mut last_refresh = SystemTime::now();

    for key in Keyboard::new() {
        match key {
            Keys::Left => {
                k -= 103;
                show_robots(&robots, k, field_width, field_height);
            }
            Keys::Right => {
                k += 103;
                show_robots(&robots, k, field_width, field_height);
            }
            Keys::Home => {
                k = 0;
                show_robots(&robots, k, field_width, field_height);
            }
            Keys::Enter => {
                show_robots(&robots, k, field_width, field_height);
                println!("GO!");
                pause = !pause;
                last_refresh = SystemTime::now();
            }
            Keys::Char('q') => break,
            _ => {}
        }
        if !pause {
            let duration = last_refresh.elapsed().unwrap().as_millis();
            if duration >= 250 {
                k += 103;
                show_robots(&robots, k, field_width, field_height);
                last_refresh = SystemTime::now();
            }
        }
    } // Eventually at iteration 7916, there will be a tree...

    print!("{}", termion::clear::All);
    println!(
        "Result: {}",
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    );
}

fn show_robots(
    robots: &Vec<(isize, isize, isize, isize)>,
    iteration: isize,
    width: isize,
    height: isize,
) {
    print!("{}", termion::clear::All);
    print!("{}k={}", termion::cursor::Goto(1, 1), iteration);
    for r in robots {
        print!(
            "{}X",
            termion::cursor::Goto(
                (r.0 + iteration * r.2).rem_euclid(width) as u16 + 1,
                (r.1 + iteration * r.3).rem_euclid(height) as u16 + 2
            )
        );
    }
}
