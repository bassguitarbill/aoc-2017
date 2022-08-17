use std::fs;

fn main() {
    let contents = fs::read_to_string("src/02/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", corruption_checksum_phase_1(contents));
    println!("Phase 2: {}", corruption_checksum_phase_2(contents));
}

fn row_checksum_phase_1(row: &str) -> u32 {
    let entries = row.split_whitespace();
    let entries = entries.map(|e| e.parse().unwrap());
    let (min, max) = entries.fold((u32::MAX, u32::MIN), |acc, e| (u32::min(acc.0, e), u32::max(acc.1, e)));
    max - min
}

fn evenly_divides(a: u32, b: u32) -> u32 {
    assert_ne!(a, b);
    match a % b  {
        0 => a / b,
        _ => 0,
    }
}

fn row_checksum_phase_2(row: &str) -> u32 {
    let entries: Vec<u32> = row.split_whitespace().map(|e| e.parse().unwrap()).collect();
    let mut sum = 0;
    for i in 0..entries.len() {
        for j in i+1..entries.len() {
            sum += evenly_divides(entries[i], entries[j]);
            sum += evenly_divides(entries[j], entries[i]);
        }
    }
    sum
}

fn corruption_checksum_phase_1(input: &String) -> u32 {
    corruption_checksum(input, row_checksum_phase_1)
}

fn corruption_checksum_phase_2(input: &String) -> u32 {
    corruption_checksum(input, row_checksum_phase_2)
}

fn corruption_checksum(input: &String, row_checksum: fn(&str) -> u32) -> u32 {
    let rows = input.lines();
    rows.fold(0, |sum, r| sum + row_checksum(r))
}

#[test]
fn test_example_phase_1() {
    let data = "5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(corruption_checksum_phase_1(&String::from(data)), 18);
}

#[test]
fn test_example_phase_2() {
    let data = "5 9 2 8
9 4 7 3
3 8 6 5";
    assert_eq!(corruption_checksum_phase_2(&String::from(data)), 9);
}

