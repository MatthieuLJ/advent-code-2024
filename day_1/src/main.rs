use std::{collections::BinaryHeap, fs::read_to_string, process};

fn main() {
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();

    if let Err(e) = read_file_into_heaps(&mut heap1, &mut heap2) {
        println!("Error reading the file {}", e);
    }
    if heap1.len() != heap2.len() || heap1.len() == 0 {
        println!(
            "I got {} and {} elements in the heaps, something went wrong",
            heap1.len(),
            heap2.len()
        );
        process::exit(1);
    }

    let mut result: u32 = 0;
    while heap1.len() > 0 {
        result += heap1.pop().unwrap().abs_diff(heap2.pop().unwrap());
    }

    println!("Final result: {}", result);
}

fn read_file_into_heaps(
    heap1: &mut BinaryHeap<u32>,
    heap2: &mut BinaryHeap<u32>,
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
    }

    Ok(())
}
