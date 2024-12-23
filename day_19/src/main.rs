use std::collections::HashMap;
use std::fs::read_to_string;
use std::str;
use trie_rs::Trie;

fn main() {
    let input_data = read_to_string("input.txt").expect("Cannot read input files");
    let mut data_lines_iter = input_data.lines();

    let patterns_line = data_lines_iter.next().unwrap();
    let patterns_iter = patterns_line.split(",").map(|s| s.trim());
    let trie = Trie::from_iter(patterns_iter);
    let patterns: Vec<&str> = patterns_line.split(",").map(|s| s.trim()).collect();

    let mut known_strings : HashMap<String, u64> = HashMap::new();

    data_lines_iter.next(); // skip the empty line

    let mut result1: u32 = 0;
    let mut result2: u64 = 0;
    while let Some(towel) = data_lines_iter.next() {
        println!("{}", towel);
        if match_towel_patterns_bool(&patterns, towel) {
            result1 += 1;
            result2 += match_towel_patterns(&trie, &mut known_strings, towel);
        }
    }

    println!("Result1: {}", result1);
    println!("Result2: {}", result2);
}

fn match_towel_patterns(trie: &trie_rs::Trie<u8>, known_strings: &mut HashMap<String, u64>, towel: &str) -> u64 {
    if towel == "" {
        return 1;
    }
    if known_strings.contains_key(towel) {
        return *known_strings.get(towel).unwrap();
    }

    let matching_prefixes: Vec<String> = trie.common_prefix_search(towel).collect();
    let mut result: u64 = 0;
    for prefix in matching_prefixes {
        if towel == prefix {
            result += 1;
        } else {
            result += match_towel_patterns(trie, known_strings,towel.strip_prefix(&prefix).unwrap());
        }
    }
    known_strings.insert(towel.to_string(), result);
    result
}


fn match_towel_patterns_bool(patterns: &Vec<&str>, towel: &str) -> bool {
    if towel == "" {
        return true;
    }
    for p in patterns {
        if let Some(short_towel) = towel.strip_prefix(p) {
            if match_towel_patterns_bool(patterns, short_towel) {
                return true;
            }
        }
    }
    return false;
}
