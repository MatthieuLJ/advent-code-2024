use std::fs::read_to_string;

fn main() {
    let mut input: Vec<(usize, Vec<usize>)> = Vec::new();
    let mut result: usize = 0;
    read_file(&mut input);

    for e in input.iter() {
        if test_possible_equation(&e.0, &e.1) {
            result += e.0;
        }
    }
    println!("Result: {}", result);
}

fn read_file(input: &mut Vec<(usize, Vec<usize>)>) {
    for line in read_to_string("input.txt")
        .expect("Cannot read file")
        .lines()
    {
        let values: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        let calibration_value = values[0].parse::<usize>().unwrap();
        let components: Vec<usize> = values[1]
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        input.push((calibration_value, components));
    }
}

fn test_possible_equation(calbration: &usize, components: &Vec<usize>) -> bool {
    for operators in 0..2_usize.pow(components.len() as u32 - 1) {
        // using the binary representation of the number to decide between
        // '+' (0) and '*' (1) operators. Least significant bit is first operator
        let mut comp_iter = components.iter();
        let mut total = *comp_iter.next().unwrap();
        for (index, compo) in comp_iter.enumerate() {
            if operators & (1 << index) == 0 {
                total += *compo;
            } else {
                total *= *compo;
            }
        }
        if total == *calbration {
            return true
        }
    }
    false
}
