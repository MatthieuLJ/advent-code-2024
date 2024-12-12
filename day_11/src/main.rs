use std::{collections::HashMap, fs::read_to_string};

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
                let (str1, str2) = at_index_str.split_at(at_index_str.len() / 2);
                data[index] = u64::from_str_radix(str1, 10).unwrap();
                data.insert(index + 1, u64::from_str_radix(str2, 10).unwrap());
                index += 2;
            } else {
                data[index] *= 2024;
                index += 1;
            }
        }
    }

    println!("Result: {}", data.len());

    let data: Vec<u64> = read_to_string("input.txt")
        .expect("Cannot read input file")
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut stone_cache = StoneCache::new();

    let mut result2 = 0;
    for s in data {
        result2 += stone_cache.get_stone(s, 75);
        println!("For the stone {}, now result is {}", s, result2);
    }

    println!("Result2: {}", result2);
}

struct StoneCache {
    cache: HashMap<(u64, usize), usize>,
}

impl StoneCache {
    pub fn new() -> StoneCache {
        let result = StoneCache  {
            cache : HashMap::new(),
        };
        return result;
    }
    pub fn get_stone(&mut self, stone: u64, generations: usize) -> usize {
        if generations == 0 {
            return 1;
        } else if self.cache.contains_key(&(stone, generations)) {
            return *self.cache.get(&(stone, generations)).unwrap();
        } else if stone == 0 {
            let result = self.get_stone(1, generations - 1);
            self.cache.insert((stone, generations), result);
            return result;
        } else if stone.to_string().len() % 2 == 0 {
            let at_index_str = stone.to_string();
            let (str1, str2) = at_index_str.split_at(at_index_str.len() / 2);
            let result1 = self.get_stone(u64::from_str_radix(str1, 10).unwrap(), generations - 1);
            let result2 = self.get_stone(u64::from_str_radix(str2, 10).unwrap(), generations - 1);
            self.cache.insert((stone, generations), result1 + result2);
            return result1 + result2;
        } else {
            let result = self.get_stone(stone * 2024, generations-1);
            self.cache.insert((stone, generations), result);
            return result;
        }
    }
}
