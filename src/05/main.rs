use std::fs;

fn main() {
    let contents = fs::read_to_string("src/05/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", twisty_trampolines_phase_1(contents));
    println!("Phase 2: {}", twisty_trampolines_phase_2(contents));
}

fn twisty_trampolines_phase_1(input: &String) -> isize {
    twisty_trampolines(input, step_phase_1)
}
fn twisty_trampolines_phase_2(input: &String) -> isize {
    twisty_trampolines(input, step_phase_2)
}

fn twisty_trampolines(input: &String, step: fn(Vec<isize>, usize, isize) -> (Vec<isize>, usize, isize)) -> isize {
    let mut numbers: Vec<isize> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
    let mut count = 0;
    let mut index: isize = 0;
    while index >= 0 && index < numbers.len() as isize {
        (numbers, count, index) = step(numbers, count, index);
    }
    count as isize
}

fn step_phase_1(mut numbers: Vec<isize>, count: usize, index: isize) -> (Vec<isize>, usize, isize) {
    let target = index + numbers[index as usize];
    numbers[index as usize] += 1;
    (numbers, count + 1, target)
}

fn step_phase_2(mut numbers: Vec<isize>, count: usize, index: isize) -> (Vec<isize>, usize, isize) {
    let target = index + numbers[index as usize];
    if numbers[index as usize] >= 3 {
        numbers[index as usize] -= 1;
    } else {
        numbers[index as usize] += 1;
    }
    (numbers, count + 1, target)
}

#[test]
fn test_phase_1() {
    let data = "0
3
0
1
-3";
    assert_eq!(twisty_trampolines_phase_1(&String::from(data)), 5);
}

#[test]
fn test_phase_2() {
    let data = "0
3
0
1
-3";
    assert_eq!(twisty_trampolines_phase_2(&String::from(data)), 10);
}
