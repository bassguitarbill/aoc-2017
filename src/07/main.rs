use std::{fs, collections::HashMap, collections::HashSet};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    holding: Vec<String>,
    holding_programs: Vec<Program>,
}

impl Program {
    fn parse(line: &str) -> Self {
        lazy_static! {
            static ref NODE_RE: Regex = Regex::new(r"^(\w+) \((\d+)\) -> (.+)").unwrap();
            static ref LEAF_RE: Regex = Regex::new(r"^(\w+) \((\d+)\)").unwrap();
        }
        if NODE_RE.is_match(line) {
            let cap = NODE_RE.captures(line).unwrap();
            let mut holding = Vec::new();
            cap[3].trim().split(", ").for_each(|n| holding.push(String::from(n.trim())));
            Program {
                name: String::from(&cap[1]),
                weight: cap[2].parse::<u32>().unwrap(),
                holding,
                holding_programs: vec![],
            }
        } else {
            let cap = LEAF_RE.captures(line).unwrap();
            Program {
                name: String::from(&cap[1]),
                weight: cap[2].parse::<u32>().unwrap(),
                holding: vec![],
                holding_programs: vec![],
            }
        }
    }

    fn calculate_weight(&mut self, programs: &mut HashMap<String, Program>) {
        for p in &self.holding {
            let program = programs.get_mut(p).unwrap();
            program.calculate_weight(programs);
        }
    }

    fn populate(&mut self, programs: HashMap<String, Program>) {
        for p in &self.holding {
            self.holding_programs.push(*programs.get(p).unwrap());
        }
    }
}


fn main() {
    let contents = fs::read_to_string("src/07/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    let solution = recursive_circus(contents);
    println!("Phase 1: {}", solution.0);
    //println!("Phase 2: {}", twisty_trampolines_phase_2(contents));
}

fn recursive_circus(input: &String) -> (String, u32) {
    let program_list: Vec<Program> = input.lines().map(|l| Program::parse(l)).collect();
    let mut program_names: HashSet<String> = HashSet::new();
    let mut programs: HashMap<String, &Program> = HashMap::new();
    for p in program_list.iter() {
        program_names.insert(String::from(&p.name));
        programs.insert(String::from(&p.name), p);
    }
    for p in program_list.iter() {
        for n in &p.holding {
            program_names.remove(n);
        }
    }
    let root_name = String::from(program_names.iter().nth(0).unwrap());
    let root = programs.get(&root_name).unwrap();

    println!("{:?}", root);
    (root_name, 1)
}

#[test]
fn test_parse_leaf_program() {
    let p = Program::parse("xsddbi (61)");
    assert_eq!(p.name, String::from("xsddbi"));
    assert_eq!(p.weight, 61);
}

#[test]
fn test_parse_node_program() {
    let p = Program::parse("txszqu (687) -> mvjqmad, lwqlyjq, jlgnsu");
    assert_eq!(p.name, String::from("txszqu"));
    assert_eq!(p.weight, 687);
    assert_eq!(p.holding, vec!["mvjqmad", "lwqlyjq", "jlgnsu"]);
}
