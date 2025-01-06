use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let mut sequences: Vec<HashMap<(i8, i8, i8, i8), i8>> = Vec::new();

    for secret_str in input_data.lines() {
        let secret = secret_str.parse::<u128>().unwrap();
        sequences.push(build_trail(secret));
    }

    let mut result : u64 = 0;
    for i in -9..=9 {
        for j in -9..=9 {
            for k in -9..=9 {
                for l in -9..=9 {
                    let mut sum: u64 = 0;
                    for seq in &sequences {
                        if let Some(val) = seq.get(&(i,j,k,l)) {
                            sum += *val as u64;
                        }
                    }
                    if sum > result {
                        println!("Best sequence so far: ({i},{j},{k},{l}) for a sum of {sum}");
                        result = sum;
                    }
                }
            }
        }
    }
}

fn build_trail(mut secret: u128) -> HashMap<(i8, i8, i8, i8), i8> {
    let mut latest_price: VecDeque<i8> = VecDeque::new();
    let mut result: HashMap<(i8, i8, i8, i8), i8> = HashMap::new();

    for _ in 0..2000 {
        if latest_price.len() == 5 {
            latest_price.pop_front();
        }
        secret = get_next_secret(secret);
        latest_price.push_back((secret % 10) as i8);
        if latest_price.len() == 5 {
            result
                .entry((
                    latest_price[1] - latest_price[0],
                    latest_price[2] - latest_price[1],
                    latest_price[3] - latest_price[2],
                    latest_price[4] - latest_price[3],
                ))
                .or_insert(latest_price[4]);
        }
    }

    result
}

/*
In particular, each buyer's secret number evolves into the next secret number in the sequence via the following process:

Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
Each step of the above process involves mixing and pruning:

To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
 */

fn get_next_secret(input: u128) -> u128 {
    let temp1 = input << 6;
    let temp2 = input ^ temp1;
    let temp3 = temp2 & (0x1000000 - 1);
    let temp4 = temp3 >> 5;
    let temp5 = temp3 ^ temp4;
    let temp6 = temp5 & (0x1000000 - 1);
    let temp7 = temp6 << 11;
    let temp8 = temp6 ^ temp7;
    let temp9 = temp8 & (0x1000000 - 1);

    temp9
}
