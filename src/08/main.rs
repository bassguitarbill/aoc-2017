use std::fs;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("src/08/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", registers_phase_1(contents));
    println!("Phase 2: {}", registers_phase_2(contents));
}

fn registers_phase_1(input: &String) -> i32 {
    let instructions = input.lines().map(parse_instruction);
    let mut registers: HashMap<String, i32> = HashMap::new();
    for instruction in instructions.into_iter() {
        apply_instruction(&mut registers, &instruction);
    }
    let mut max = i32::MIN;
    for register in registers.into_iter() {
        max = cmp::max(max, register.1);
    }
    max
}
fn registers_phase_2(input: &String) -> i32 {
    let instructions = input.lines().map(parse_instruction);
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max = i32::MIN;
    for instruction in instructions.into_iter() {
        max = cmp::max(max, apply_instruction(&mut registers, &instruction));
    }
    max
}

fn apply_instruction<'a, 'b: 'a>(registers: &mut HashMap<String, i32>, instruction: &'a Instruction) -> i32 {
    if evaluate_condition(registers, &instruction.condition) {
        let value = match instruction.operation {
            Operation::Inc =>  instruction.amount,
            Operation::Dec => -instruction.amount,
        };
        let reg_value = get_or_init(registers, &instruction.register.0);
        registers.insert(instruction.register.0.clone(), reg_value + value);
        reg_value + value
    } else { 0 }
}

fn evaluate_condition<'a, 'b: 'a>(registers: &mut HashMap<String, i32>, condition: &'a Condition) -> bool {
    let left = match &condition.left {
        RegisterOrConstant::Register(Register(left_name)) => get_or_init(registers, &left_name),
        RegisterOrConstant::Constant(value) => *value,
    };
    let right = match &condition.right {
        RegisterOrConstant::Register(Register(right_name)) => get_or_init(registers, &right_name),
        RegisterOrConstant::Constant(value) => *value,
    };
    match condition.comparator {
        Comparator::LessThan => left < right,
        Comparator::GreaterThan => left > right,
        Comparator::LessOrEqual => left <= right,
        Comparator::GreaterOrEqual => left >= right,
        Comparator::Equal => left == right,
        Comparator::NotEqual => left != right,
    }
}

fn get_or_init<'a, 'b: 'a>(registers: &mut HashMap<String, i32>, key: &'a str) -> i32 {
    match registers.get(key) {
        Some(val) => *val,
        None => {
            registers.insert(String::from(key), 0);
            0
        },
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut split_iter = line.split_whitespace();

    let register = Register(String::from(split_iter.next().unwrap()));

    let operation = match split_iter.next().unwrap() {
        "inc" => Operation::Inc,
        "dec" => Operation::Dec,
        op => panic!("Invalid operator {}", op)
    };

    let amount: i32 = split_iter.next().unwrap().parse().unwrap();

    split_iter.next(); // The useless word "if"

    let left = parse_register_or_constant(split_iter.next().unwrap());
    let comparator = parse_comparator(split_iter.next().unwrap());
    let right = parse_register_or_constant(split_iter.next().unwrap());

    let condition = Condition{ left, comparator, right };

    Instruction { register, operation, amount, condition }
}

fn parse_comparator(input: &str) -> Comparator {
    match input {
        "<" => Comparator::LessThan,
        ">" => Comparator::GreaterThan,
        "<=" => Comparator::LessOrEqual,
        ">=" => Comparator::GreaterOrEqual,
        "==" => Comparator::Equal,
        "!=" => Comparator::NotEqual,
        _ => panic!("Invalid comparator {}", input),
    }
}

fn parse_register_or_constant(input: &str) -> RegisterOrConstant {
    match input.parse::<i32>() {
        Ok(n) => RegisterOrConstant::Constant(n),
        Err(_) => RegisterOrConstant::Register( Register(String::from(input)) ),
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    register: Register,
    operation: Operation,
    amount: i32,
    condition: Condition,
}

#[derive(Debug, PartialEq)]
struct Register(String);

#[derive(Debug, PartialEq)]
enum Operation {
    Inc,
    Dec,
}

#[derive(Debug, PartialEq)]
enum RegisterOrConstant {
    Register(Register),
    Constant(i32),
}

#[derive(Debug, PartialEq)]
struct Condition {
    left: RegisterOrConstant,
    comparator: Comparator,
    right: RegisterOrConstant,
}

#[derive(Debug, PartialEq)]
enum Comparator {
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

#[test]
fn test_parse_instructions() {
    assert_eq!(
        parse_instruction("b inc 5 if a > 1"),
        Instruction {
            register: Register(String::from("b")),
            operation: Operation::Inc,
            amount: 5,
            condition: Condition {
                left: RegisterOrConstant::Register(Register(String::from("a"))),
                comparator: Comparator::GreaterThan,
                right: RegisterOrConstant::Constant(1),
            }
        },
    );
    assert_eq!(
        parse_instruction("a inc 1 if b < 5"),
        Instruction {
            register: Register(String::from("a")),
            operation: Operation::Inc,
            amount: 1,
            condition: Condition {
                left: RegisterOrConstant::Register(Register(String::from("b"))),
                comparator: Comparator::LessThan,
                right: RegisterOrConstant::Constant(5),
            }
        },
    );
    assert_eq!(
        parse_instruction("c dec -10 if a >= 1"),
        Instruction {
            register: Register(String::from("c")),
            operation: Operation::Dec,
            amount: -10,
            condition: Condition {
                left: RegisterOrConstant::Register(Register(String::from("a"))),
                comparator: Comparator::GreaterOrEqual,
                right: RegisterOrConstant::Constant(1),
            }
        },
    );
    assert_eq!(
        parse_instruction("c inc -20 if c == 10"),
        Instruction {
            register: Register(String::from("c")),
            operation: Operation::Inc,
            amount: -20,
            condition: Condition {
                left: RegisterOrConstant::Register(Register(String::from("c"))),
                comparator: Comparator::Equal,
                right: RegisterOrConstant::Constant(10),
            }
        },
    );
}

#[test]
fn test_evaluate_conditions() {
    let mut registers: HashMap<String, i32> = HashMap::new();
    registers.insert(String::from("a"), 0);
    registers.insert(String::from("b"), 2);
    registers.insert(String::from("c"), 5);
    let condition = Condition{
        left: RegisterOrConstant::Register(Register(String::from("a"))),
        comparator: Comparator::Equal,
        right: RegisterOrConstant::Constant(0),
    };
    assert!(evaluate_condition(&mut registers, &condition));
    let condition = &Condition{
        left: RegisterOrConstant::Register(Register(String::from("c"))),
        comparator: Comparator::GreaterThan,
        right: RegisterOrConstant::Register(Register(String::from("b"))),
    };
    assert!(evaluate_condition(&mut registers, &condition));
    let condition = &Condition{
        left: RegisterOrConstant::Constant(2),
        comparator: Comparator::GreaterOrEqual,
        right: RegisterOrConstant::Constant(2),
    };
    assert!(evaluate_condition(&mut registers, &condition));
    let condition = &Condition{
        left: RegisterOrConstant::Constant(2),
        comparator: Comparator::NotEqual,
        right: RegisterOrConstant::Register(Register(String::from("a"))),
    };
    assert!(evaluate_condition(&mut registers, &condition));
}
