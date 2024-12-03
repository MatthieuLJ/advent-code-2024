use std::collections::VecDeque;
use std::fs::read_to_string;
//use std::process;

fn main() {
    let mut result = 0;

    for line in read_to_string("input.txt")
        .expect("Cannot read file")
        .lines()
    {
        let mut parts: VecDeque<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();
        if parts.len() < 2 {
            continue;
        }
        println!("This line has {} elements {:?}", parts.len(), parts);
        let increasing = parts.get(1) > parts.get(0);
        
        println!(
            "This sequence is {} increasing",
            if increasing { "" } else { "not" }
        );
        
        let mut num = parts.pop_front().unwrap();
        let mut fail = false;
        while parts.len() > 0 {
            let next = parts.pop_front().unwrap();
            println!("Evaluating {} and {}", num, next);
            if (next < num && increasing)
                || (next > num && !increasing)
                || (next == num)
                || (next.abs_diff(num) > 3)
            {
                println!("Fail!");
                fail = true;
                break;
            }

            num = next;
        }
        if !fail {
            println!("Succeed!");
            result += 1;
        }
        //process::exit(0);
    }

    println!("Final result: {}", result);
}