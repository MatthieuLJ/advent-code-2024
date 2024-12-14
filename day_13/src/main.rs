use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("input.txt").expect("Cannot read from input file");
    let mut input_data = binding.lines();
    let mut result: isize = 0;
    let mut result2: i128 = 0;

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
            None => {},
            Some((press_a, press_b)) => {
                result += 3 * press_a + press_b;
            }
        };

        let presses_more: Option<(i128, i128)> = solve_lot_presses(
            xt as i128 + 10000000000000,
            xa as i128,
            xb as i128,
            yt as i128 + 10000000000000,
            ya as i128,
            yb as i128,
        );
        match presses_more {
            None => {},
            Some((press_a, press_b)) => {
                result2 += 3 * press_a + press_b;
            }
        };
    }
    println!("Result: {}", result);
    println!("Result2: {}", result2);
}

fn get_minimum_press_a(
    xt: isize,
    xa: isize,
    xb: isize,
    yt: isize,
    ya: isize,
    yb: isize,
) -> Option<(isize, isize)> {
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

fn solve_lot_presses(
    xt: i128,
    xa: i128,
    xb: i128,
    yt: i128,
    ya: i128,
    yb: i128,
) -> Option<(i128, i128)> {
    // Solve the diophantine equation xa . press_a + xb . press_b = xt
    let (gcd, mut press_a, mut press_b) = extended_euclidean(xa, xb);
    if xt % gcd != 0 {
        return None;
    }
    press_a *= xt / gcd;
    press_b *= xt / gcd;

    // all solutions are (press_a + k.u, press_b - kv) for any integer k
    // with u = xb / gcd and v = xa / gcd
    // first make press_a positive but minimal
    let u = xb/gcd;
    let v = -xa/gcd;

    if press_a < 0 {
        let k = (-press_a) / u;
        press_a += (k + 1) * u;
        press_b += (k + 1) * v;
    }
    if press_a - u >= 0 {
        let k = press_a / u;
        press_a -= k * u;
        press_b -= k * v;
    }

    if press_b < 0 {
        return None;
    }

    // each iteration moves us toward u * ya + v * yb for Y
    let change = u * ya + v * yb;
    if (yt - ((ya * press_a) + (yb * press_b))) % change != 0 {
        return None;
    }
    let k = (yt -((ya * press_a) + (yb * press_b))) / change;
    press_a += k * u;
    press_b += k * v;

    return Some((press_a, press_b));
}

fn extended_euclidean(a: i128, b: i128) -> (i128, i128, i128) {
    // returns (gcd, m, n) such that a*m + b*n = gcd
    if a == 0 {
        (b, 0, 1)
    } else if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, m0, n0) = extended_euclidean(b, a % b);
        // we have gcd = m0 . b + n0 . (a % b)
        //             = m0 . b + n0 . (a - [a/b].b)
        //             = n0 . a + (m0 - n0 . [a/b]) . b
        (gcd, n0, m0 - n0 * (a / b))
    }
}
