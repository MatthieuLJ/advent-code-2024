use std::collections::hash_map::Entry::*;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
    process,
};

fn main() {
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();
    let mut map2 = HashMap::new();

    if let Err(e) = read_file_into_heaps(&mut heap1, &mut heap2, &mut map2) {
        println!("Error reading the file {}", e);
    }
    if heap1.len() != heap2.len() || heap1.len() == 0 || map2.len() == 0 {
        println!(
            "I got {} and {} elements in the heaps, something went wrong",
            heap1.len(),
            heap2.len()
        );
        process::exit(1);
    }

    let mut result1: u32 = 0;
    let mut result2: u32 = 0;
    while heap1.len() > 0 {
        let entry = heap1.pop().unwrap();

        result1 += entry.abs_diff(heap2.pop().unwrap());

        let count = match map2.entry(entry) {
            Occupied(o) => *o.get(),
            Vacant(_) => 0
        };
        result2 += entry * count;

    }

    println!("Final result for part1: {}", result1);
    println!("Final result for part2: {}", result2);
}

fn read_file_into_heaps(
    heap1: &mut BinaryHeap<u32>,
    heap2: &mut BinaryHeap<u32>,
    map2: &mut HashMap<u32, u32>,
) -> Result<(), std::io::Error> {
    for line in read_to_string("input.txt")?.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() != 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Did not get 2 parts",
            ));
        }
        heap1.push(u32::from_str_radix(parts.get(0).unwrap(), 10).unwrap());
        heap2.push(u32::from_str_radix(parts.get(1).unwrap(), 10).unwrap());
        map2.entry(u32::from_str_radix(parts.get(1).unwrap(), 10).unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);

    }

    Ok(())
}
