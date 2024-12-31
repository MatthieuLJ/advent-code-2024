use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let mut result = 0;

    for l in input_data.lines() {
        let robot1 = get_program_for_code(l);
        let robot2 = get_program_for_directions(&robot1);
        let final_sequence = get_program_for_directions(&robot2);

        let numeric_value = l[0..l.len()-1].parse::<u32>().unwrap();

        println!("For code {}, got value {} and length {}", &l, numeric_value, final_sequence.len());
        println!("robot1: [{}]\nrobot2: [{}]\n final: [{}]", robot1, robot2, final_sequence);
        
        result += numeric_value * final_sequence.len() as u32;
    }

    println!("Final result {}", result);
    
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
fn get_program_for_code(codes: &str) -> String {
    let sequence = HashMap::from([
        // first character in the sequence, going from "A"
        ("A", vec![""]),
        ("0", vec!["<"]),
        ("1", vec!["^<<"]),
        ("2", vec!["<^", "^<"]),
        ("3", vec!["^"]),
        ("4", vec!["^^<<"]),
        ("5", vec!["<^^", "^^<"]),
        ("6", vec!["^^"]),
        ("7", vec!["^^^<<"]),
        ("8", vec!["<^^^", "^^^<"]),
        ("9", vec!["^^^"]),
        // transitions from one key to another
        ("AA", vec![""]),
        ("A0", vec!["<"]),
        ("A1", vec!["^<<"]),
        ("A2", vec!["<^", "^<"]),
        ("A3", vec!["^"]),
        ("A4", vec!["^^<<"]),
        ("A5", vec!["<^^", "^^<"]),
        ("A6", vec!["^^"]),
        ("A7", vec!["^^^<<"]),
        ("A8", vec!["<^^^", "^^^<"]),
        ("A9", vec!["^^^"]),
        ("0A", vec![">"]),
        ("00", vec![""]),
        ("01", vec!["^<"]),
        ("02", vec!["^"]),
        ("03", vec!["^>", ">^"]),
        ("04", vec!["^^<"]),
        ("05", vec!["^^"]),
        ("06", vec!["^^>", ">^^"]),
        ("07", vec!["^^^<"]),
        ("08", vec!["^^^"]),
        ("09", vec!["^^^>", ">^^^"]),
        ("1A", vec![">>v", "v>>"]),
        ("10", vec![">v"]),
        ("11", vec![""]),
        ("12", vec![">"]),
        ("13", vec![">>"]),
        ("14", vec!["^"]),
        ("15", vec!["^>", ">^"]),
        ("16", vec!["^>>", ">>^"]),
        ("17", vec!["^^"]),
        ("18", vec![">^^", ">^^"]),
        ("19", vec![">>^^", "^^>>"]),
        ("2A", vec![">v", "v>"]),
        ("20", vec!["v"]),
        ("21", vec!["<"]),
        ("22", vec![""]),
        ("23", vec![">"]),
        ("24", vec!["<^", "^<"]),
        ("25", vec!["^"]),
        ("26", vec![">^", "^>"]),
        ("27", vec!["<^^", "^^<"]),
        ("28", vec!["^^"]),
        ("29", vec!["^^>", ">^^"]),
        ("3A", vec!["v"]),
        ("30", vec!["<v", "v<"]),
        ("31", vec!["<<"]),
        ("32", vec!["<"]),
        ("33", vec![""]),
        ("34", vec!["<<^", "^<<"]),
        ("35", vec!["<^", "^<"]),
        ("36", vec!["^"]),
        ("37", vec!["<<^^", "^^<<"]),
        ("38", vec!["<^^", "^^<"]),
        ("39", vec!["^^"]),
        ("4A", vec![">>vv"]),
        ("40", vec![">vv"]),
        ("41", vec!["v"]),
        ("42", vec!["v>", ">v"]),
        ("43", vec!["v>>", ">>v"]),
        ("44", vec![""]),
        ("45", vec![">"]),
        ("46", vec![">>"]),
        ("47", vec!["^"]),
        ("48", vec!["^>", ">^"]),
        ("49", vec!["^>>", ">>^"]),
        ("5A", vec!["vv>", ">vv"]),
        ("50", vec!["vv"]),
        ("51", vec!["<v", "v<"]),
        ("52", vec!["v"]),
        ("53", vec!["v>", ">v"]),
        ("54", vec!["<"]),
        ("55", vec![""]),
        ("56", vec![">"]),
        ("57", vec!["<^", "^<"]),
        ("58", vec!["^"]),
        ("59", vec!["^>", ">^"]),
        ("6A", vec!["vv"]),
        ("60", vec!["<vv", "vv<"]),
        ("61", vec!["<<v", "v<<"]),
        ("62", vec!["<v", "v<"]),
        ("63", vec!["v"]),
        ("64", vec!["<<"]),
        ("65", vec!["<"]),
        ("66", vec![""]),
        ("67", vec!["<<^", "^<<"]),
        ("68", vec!["<^", "^<"]),
        ("69", vec!["^"]),
        ("7A", vec![">>vvv"]),
        ("70", vec![">vvv", "vvv>"]),
        ("71", vec!["vv"]),
        ("72", vec!["vv>", ">vv"]),
        ("73", vec!["vv>>", ">>vv"]),
        ("74", vec!["v"]),
        ("75", vec!["v>", ">v"]),
        ("76", vec!["v>>", ">>v"]),
        ("77", vec![""]),
        ("78", vec![">"]),
        ("79", vec![">>"]),
        ("8A", vec!["vvv>", ">vvv"]),
        ("80", vec!["vvv"]),
        ("81", vec!["vv<", "<vv"]),
        ("82", vec!["vv"]),
        ("83", vec!["vv>", ">vv"]),
        ("84", vec!["<v", "v<"]),
        ("85", vec!["v"]),
        ("86", vec!["v>", ">v"]),
        ("87", vec!["<"]),
        ("88", vec![""]),
        ("89", vec![">"]),
        ("9A", vec!["vvv"]),
        ("90", vec!["<vvv", "vvv<"]),
        ("91", vec!["<<vv", "vv<<"]),
        ("92", vec!["<vv", "vv<"]),
        ("93", vec!["vv"]),
        ("94", vec!["<<v", "v<<"]),
        ("95", vec!["<v", "v<"]),
        ("96", vec!["v"]),
        ("97", vec!["<<"]),
        ("98", vec!["<"]),
        ("99", vec![""]),
    ]);

    
    let mut result = String::new();
    result.push_str(sequence[&codes[0..1]][0]);
    result.push('A');
    for n in 0..codes.len()-1 {
        println!("Looking for code {}", &codes[n..n+2]);
        result.push_str(sequence[&codes[n..n+2]][0]);
        result.push('A');
    }

    result
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn get_program_for_directions(codes: &str) -> String {
    let sequence = HashMap::from([
        // first character in the sequence, going from "A"
        ("A", vec![""]),
        ("^", vec!["<"]),
        ("<", vec!["v<<"]),
        ("v", vec!["<v", "v<"]),
        (">", vec!["v"]),
        // transitions from one key to another
        ("AA", vec![""]),
        ("A^", vec!["<"]),
        ("A<", vec!["v<<"]),
        ("Av", vec!["<v", "v<"]),
        ("A>", vec!["v"]),
        ("^A", vec![">"]),
        ("^^", vec![""]),
        ("^<", vec!["v<"]),
        ("^v", vec!["v"]),
        ("^>", vec!["v>", ">v"]),
        ("<A", vec![">>^"]),
        ("<^", vec![">^"]),
        ("<<", vec![""]),
        ("<v", vec![">"]),
        ("<>", vec![">>"]),
        ("vA", vec![">^", "^>"]),
        ("v<", vec!["<"]),
        ("vv", vec![""]),
        ("v>", vec![">"]),
        (">A", vec!["^"]),
        (">^", vec!["^<", "<^"]),
        ("><", vec!["<<"]),
        (">v", vec!["<"]),
        (">>", vec![""]),
    ]);

    let mut result = String::new();
    result.push_str(sequence[&codes[0..1]][0]);
    result.push('A');
    for n in 0..codes.len()-1 {
        result.push_str(sequence[&codes[n..n+2]][0]);
        result.push('A');
    }

    result
}