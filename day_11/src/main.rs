use std::fs::read_to_string;

fn main() {
    let mut data: Vec<u64> = read_to_string("input.txt")
        .expect("Cannot read input file")
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut index = 0;
        while index < data.len() {
            if data[index] == 0 {
                data[index] = 1;
                index += 1;
            } else if data[index].to_string().len() % 2 == 0 {
                let at_index_str = data[index].to_string();
                let (str1,str2) = at_index_str.split_at(data[index].to_string().len() / 2);
                data[index] = u64::from_str_radix(str1, 10).unwrap();
                data.insert(index+1, u64::from_str_radix(str2, 10).unwrap());
                index += 2;
            } else {
                data[index] *= 2024;
                index += 1;
            }
        }
    }

    println!("Result: {}", data.len());
}
