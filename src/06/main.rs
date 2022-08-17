use std::{fs, collections::HashSet, collections::HashMap};

fn main() {
    let contents = fs::read_to_string("src/06/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", memory_reallocation_phase_1(contents));
    println!("Phase 2: {}", memory_reallocation_phase_2(contents));
}

fn memory_step(mut mem: Vec<usize>) -> Vec<usize> {
    let mut max = *mem.iter().max().unwrap();
    let mut i = mem.iter().position(|&m| m == max).unwrap() + 1;
    let mem_length = mem.len();
    mem[i - 1] = 0;
    while max > 0 {
        mem[i % mem_length] += 1;
        i += 1;
        max = max - 1;
    }
    mem
}

fn memory_snapshot(mem: &Vec<usize>) -> String {
    let mut s = String::new();
    for x in mem {
        s = s + &x.to_string()  + "|";
    }
    s
}

fn memory_reallocation_phase_1(input: &String) -> usize {
    let mut memory: Vec<usize> = input.split_whitespace().map(|l| l.parse::<usize>().unwrap()).collect();
    let mut previous_memory: HashSet<String> = HashSet::new();
    let mut count = 0;
    while previous_memory.get(&memory_snapshot(&memory)).is_none() {
        previous_memory.insert(memory_snapshot(&memory));
        memory = memory_step(memory);
        count += 1;
    }
    count
}

fn memory_reallocation_phase_2(input: &String) -> usize {
    let mut memory: Vec<usize> = input.split_whitespace().map(|l| l.parse::<usize>().unwrap()).collect();
    let mut previous_memory: HashMap<String, usize> = HashMap::new();
    let mut count = 0;

    while previous_memory.get(&memory_snapshot(&memory)).is_none() {
        previous_memory.insert(memory_snapshot(&memory), count);
        memory = memory_step(memory);
        count += 1;
    }
    count - *previous_memory.get(&memory_snapshot(&memory)).unwrap()
}

#[test]
fn test_memory_step() {
    let mut mem = vec![0, 2, 7, 0];
    mem = memory_step(mem);
    assert_eq!(memory_snapshot(&mem), "2|4|1|2|");
    mem = memory_step(mem);
    assert_eq!(memory_snapshot(&mem), "3|1|2|3|");
    mem = memory_step(mem);
    assert_eq!(memory_snapshot(&mem), "0|2|3|4|");
    mem = memory_step(mem);
    assert_eq!(memory_snapshot(&mem), "1|3|4|1|");
    mem = memory_step(mem);
    assert_eq!(memory_snapshot(&mem), "2|4|1|2|");
}
