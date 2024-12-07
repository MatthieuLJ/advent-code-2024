use std::fs::read_to_string;

fn main() {
    let mut input: Vec<(usize, Vec<usize>)> = Vec::new();
    let mut result: usize = 0;
    let mut result2: usize = 0;
    read_file(&mut input);

    for e in input.iter() {
        if test_possible_equation(&e.0, &e.1) {
            result += e.0;
        }
        if test_possible_equation_with_concat(&e.0, &e.1) {
            result2 += e.0;
        }
    }
    println!("Result: {}", result);
    println!("Result2: {}", result2);
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
            return true;
        }
    }
    false
}

fn test_possible_equation_with_concat(calbration: &usize, components: &Vec<usize>) -> bool {
    let ops = OperatorCombinations::new(components.len() - 1);
    //println!("Testing {} with {:?}", calbration, components);
    for operators in ops {
        //println!("With operators {:?}", operators);
        let mut comp_iter = components.iter();
        let mut total = *comp_iter.next().unwrap();
        for (index, compo) in comp_iter.enumerate() {
            total = match operators[index] {
                Operators::PLUS => total + compo,
                Operators::MULT => total * compo,
                Operators::CONC => {
                    total * 10_usize.pow(1 + compo.checked_ilog10().unwrap_or(0)) + compo
                }
            };
            //println!("At {}", total);
        }
        if total == *calbration {
            return true;
        }
    }
    false
}

#[derive(Clone, PartialEq, Debug)]
enum Operators {
    PLUS,
    MULT,
    CONC,
}

struct OperatorCombinations {
    size: usize,
    current: Vec<Operators>,
    overflow: bool,
}

impl OperatorCombinations {
    fn new(size: usize) -> OperatorCombinations {
        let mut result = OperatorCombinations {
            size: size,
            current: Vec::new(),
            overflow: false,
        };

        for _i in 0..size {
            result.current.push(Operators::PLUS);
        }
        result
    }
}

impl Iterator for OperatorCombinations {
    type Item = Vec<Operators>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflow {
            return None;
        }
        let result = self.current.clone();
        let mut index = 0;
        while index < self.size {
            self.current[index] = match self.current[index] {
                Operators::PLUS => Operators::MULT,
                Operators::MULT => Operators::CONC,
                Operators::CONC => Operators::PLUS,
            };
            if self.current[index] == Operators::PLUS {
                index += 1;
            } else {
                break;
            }
        }
        if index == self.size {
            self.overflow = true;
        }
        Some(result)
    }
}
