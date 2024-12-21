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

    let mut _result: Vec<u64> = Vec::new();

    /* after a few tries, we find that all the values of a that reach at least 5 items in the program cycle through an addition of
         66824
    256
    63992
    1504520
    256
    261888
    256
    199160
    66824
    256
    63992
    1966080

    and we can start at 1079733

    then we repeat the process to find another start and increases sequence
     */

    let mut init_a = 596353314493;
    let adds = [
        256,
        67108608,
        256,
        68652367616,
        256,
        67108608,
        256,
        885904113408,
        256,
        2952789760,
        256,
        67108608,
        256,
        67108608,
        256,
        67108608,
        256,
        67108608,
        256,
        141599702784,
        256,
        67108608,
        256,
        68652367616,
        256,
        67108608,
        256,
        1030725041920,
        256,
        67108608,
        256,
        68652367616,
        256,
        67108608,
        256,
        637056057088,
        256,
        8388352,
        256,
        8388352,
        256,
        83885824,
        256,
        248747392768,
        256,
        2952789760,
        256,
        67108608,
        256,
        67108608,
        256,
        67108608,
        256,
        67108608,
        256,
        22808624896,
        256,
        8388352,
        256,
        8388352,
        256,
        83885824,
        256,
        2592079616,
        256,
        67108608,
        256,
        67108608,
        256,
        67108608,
        256,
        325388792,
        1766664,
        256,
        8388352,
        256,
        8388352,
        256,
        100663040,
        256,
        184171888384,
        256,
        67108608,
        256,
        1030725041920,
    ];
    let mut adds_index = 0;
    let mut pc_match;

    'outer: loop {
        init_a += adds[adds_index];
        adds_index += 1;
        if adds_index == adds.len() {
            adds_index = 0;
        }
        pc_match = 0;
        current_state.reg_a = init_a;
        current_state.pc = 0;
        current_state.reg_b = 0;
        current_state.reg_c = 0;
        print!("\r{} ", current_state.reg_a);

        loop {
            match current_state.instructions[current_state.pc] {
                0 => {
                    current_state.reg_a =
                        current_state.reg_a / (1 << get_combo_operand(&current_state))
                }

                1 => {
                    current_state.reg_b = current_state.reg_b ^ get_literal_operand(&current_state)
                }
                2 => current_state.reg_b = get_combo_operand(&current_state) % 8,
                3 => {
                    if current_state.reg_a != 0 {
                        current_state.pc = get_literal_operand(&current_state) as usize;
                        continue;
                    }
                }
                4 => current_state.reg_b ^= current_state.reg_c,
                5 => {
                    /* result.push(get_combo_operand(&current_state) % 8) */
                    let x = (get_combo_operand(&current_state) % 8) as u8;
                    if current_state.instructions[pc_match] == x {
                        print!("{}, ", x);
                        pc_match += 1;
                    } else {
                        if pc_match >= 12 {
                            println!("");
                        }
                        continue 'outer;
                    }
                }
                6 => {
                    current_state.reg_b =
                        current_state.reg_a / (1 << get_combo_operand(&current_state))
                }
                7 => {
                    current_state.reg_c =
                        current_state.reg_a / (1 << get_combo_operand(&current_state))
                }
                _ => panic!(),
            }
            current_state.pc += 2;
            if current_state.pc >= current_state.instructions.len() {
                //println!("pc is {}", current_state.pc);

                if pc_match == current_state.instructions.len() {
                    break 'outer;
                } else {
                    break;
                }
            }
        }
    }

    /*println!("{:?}", result);*/
    println!("{}", init_a);
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
