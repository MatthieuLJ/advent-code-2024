use regex::Regex;
use std::fs::read_to_string;

struct Chronospatial {
    instructions: Vec<u8>,
    pc: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

fn main() {
    let mut current_state = Chronospatial {
        instructions: Vec::new(),
        pc: 0,
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
    };

    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let mut input_line_iter = input_data.lines();

    let re_reg_a = Regex::new(r"Register A: (\d+)").unwrap();
    let matches = re_reg_a.captures(&input_line_iter.next().unwrap()).unwrap();
    current_state.reg_a = matches[1].parse::<u64>().unwrap();

    let re_reg_b = Regex::new(r"Register B: (\d+)").unwrap();
    let matches = re_reg_b.captures(&input_line_iter.next().unwrap()).unwrap();
    current_state.reg_b = matches[1].parse::<u64>().unwrap();

    let re_reg_c = Regex::new(r"Register C: (\d+)").unwrap();
    let matches = re_reg_c.captures(&input_line_iter.next().unwrap()).unwrap();
    current_state.reg_c = matches[1].parse::<u64>().unwrap();

    let _ = input_line_iter.next();

    let re_prog = Regex::new(r"Program: ((?:\d+,)+\d+)").unwrap();
    let program = re_prog.captures(&input_line_iter.next().unwrap()).unwrap()[1].to_string();
    for c in program.chars() {
        if c == ',' {
            continue;
        }
        current_state
            .instructions
            .push(c.to_digit(10).unwrap() as u8);
    }
    println!("{:?}", current_state.instructions);

    let mut result: Vec<u64> = Vec::new();
    loop {
        match current_state.instructions[current_state.pc] {
            0 => 
                current_state.reg_a = current_state.reg_a / (1 << get_combo_operand(&current_state)),
            
            1 => current_state.reg_b = current_state.reg_b ^ get_literal_operand(&current_state),
            2 => current_state.reg_b = get_combo_operand(&current_state) % 8,
            3 => {
                if current_state.reg_a != 0 {
                    current_state.pc = get_literal_operand(&current_state) as usize;
                    continue;
                }
            },
            4 => current_state.reg_b ^= current_state.reg_c,
            5 => result.push(get_combo_operand(&current_state) % 8),
            6 => current_state.reg_b = current_state.reg_a / (1 << get_combo_operand(&current_state)),
            7 => current_state.reg_c = current_state.reg_a / (1 << get_combo_operand(&current_state)),
            _ => panic!(),
        }
        current_state.pc += 2;
        if current_state.pc >= current_state.instructions.len() {
            break;
        }
    }

    println!("{:?}", result);
}

fn get_literal_operand(state: &Chronospatial) -> u64 {
    state.instructions[state.pc + 1] as u64
}

fn get_combo_operand(state: &Chronospatial) -> u64 {
    match state.instructions[state.pc + 1] {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => state.reg_a,
        5 => state.reg_b,
        6 => state.reg_c,
        _ => panic!(),
    }
}
