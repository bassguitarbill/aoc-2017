use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/12/data.txt").expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", digital_plumber_phase_1(contents));
    println!("Phase 2: {}", digital_plumber_phase_2(contents));
}

fn digital_plumber_phase_1(input: &str) -> i32 {
    let programs: Vec<Program> = input.lines().map(parse_program).collect();
    let mut checked_addresses: Vec<i32> = Vec::new();
    let mut addresses_to_check: Vec<i32> = vec![0];
    while !addresses_to_check.is_empty() {
        checked_addresses.push(addresses_to_check[0]);
        let address: usize = addresses_to_check[0] as usize;
        let prg = &programs[address];
        for pipe in &prg.pipes {
            if !addresses_to_check.contains(pipe) && !checked_addresses.contains(pipe) {
                addresses_to_check.push(*pipe);
            }
        }
        addresses_to_check.remove(0);
    }
    checked_addresses.len() as i32
}

fn digital_plumber_phase_2(input: &str) -> i32 {
    let mut programs: Vec<Program> = input.lines().map(parse_program).collect();
    let mut checked_addresses: Vec<i32> = Vec::new();
    let mut num_groups = 0;
    for x in 0..programs.len() {
        if programs[x].checked {
            continue;
        }
        num_groups += 1;
        let mut addresses_to_check: Vec<i32> = vec![x as i32];
        while !addresses_to_check.is_empty() {
            checked_addresses.push(addresses_to_check[0]);
            let address: usize = addresses_to_check[0] as usize;
            let mut prg = &mut programs[address];
            prg.checked = true;
            for pipe in &prg.pipes {
                if !addresses_to_check.contains(pipe) && !checked_addresses.contains(pipe) {
                    addresses_to_check.push(*pipe);
                }
            }
            addresses_to_check.remove(0);
        }
    }
    num_groups
}

fn parse_program(line: &str) -> Program {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d+) <-> ((?:\\d+, )*(?:\\d+))").unwrap();
    }
    let pipes_str = RE.captures(line).unwrap().get(2).unwrap().as_str();
    let pipes = pipes_str
        .split(", ")
        .map(|p| p.parse::<i32>().unwrap())
        .collect();
    Program {
        pipes,
        checked: false,
    }
}

#[derive(Debug)]
struct Program {
    pipes: Vec<i32>,
    checked: bool,
}

#[test]
fn test_parse_program() {
    let prg_str = "0 <-> 1, 2, 3";
    let prg = parse_program(prg_str);
    assert_eq!(prg.pipes, vec!(1, 2, 3));

    let prg_str = "120 <-> 9";
    let prg = parse_program(prg_str);
    assert_eq!(prg.pipes, vec!(9));
}
