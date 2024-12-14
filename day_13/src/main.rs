use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("input.txt").expect("Cannot read from input file");
    let mut input_data = binding.lines();
    let mut result: isize = 0;

    loop {
        let Some(first_line) = input_data.next() else {
            break;
        };
        let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let matches = re.captures(&first_line).unwrap();
        let xa = matches.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let ya = matches.get(2).unwrap().as_str().parse::<isize>().unwrap();

        let Some(second_line) = input_data.next() else {
            break;
        };
        let re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let matches = re.captures(&second_line).unwrap();
        let xb = matches.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let yb = matches.get(2).unwrap().as_str().parse::<isize>().unwrap();

        let Some(third_line) = input_data.next() else {
            break;
        };
        let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let matches = re.captures(&third_line).unwrap();
        let xt = matches.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let yt = matches.get(2).unwrap().as_str().parse::<isize>().unwrap();

        let _ = input_data.next();

        // we want to minimize the number of presses on button A
        // first try the naive way to solve with the minimum number of presses on A
        // this may not be true if A was getting us closer to the goal more than
        // 3x faster than B....
        let presses: Option<(isize, isize)> = get_minimum_press_a(xt, xa, xb, yt, ya, yb);
        match presses {
            None => continue,
            Some((press_a, press_b)) => {
                dbg!((press_a, press_b));
                result += 3 * press_a + press_b;
            }
        };
    }
    println!("Result: {}", result);
}

fn get_minimum_press_a(
    xt: isize,
    xa: isize,
    xb: isize,
    yt: isize,
    ya: isize,
    yb: isize,
) -> Option<(isize, isize)> {
    println!("Solving for: ");
    dbg!((xt,xa,xb,yt,ya,yb));
    for press_a in 0..100 as isize {
        if (xt - (press_a as isize * xa)) % xb == 0
            && (yt - (press_a as isize * ya)) % yb == 0
            && (xt - (press_a as isize * xa)) / xb == (yt - (press_a as isize * ya)) / yb
        {
            return Some((press_a, (xt - (press_a as isize * xa)) / xb));
        }
    }
    None
}
