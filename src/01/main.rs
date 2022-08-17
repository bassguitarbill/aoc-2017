use std::fs;

fn main() {
    let contents = fs::read_to_string("src/01/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", inverse_captcha_phase_1(contents));
    println!("Phase 2: {}", inverse_captcha_phase_2(contents));
}

fn inverse_captcha(input: &String, rotations: fn(&String) -> (&str, String)) -> u32 {
    let mut sum = 0;
    let (reference, comparison) = rotations(&input);
    let mut reference = reference.chars();
    let mut comparison = comparison.chars();
    loop {
        let r = reference.next();
        match r {
            Some(r) => {
                let c = comparison.next().unwrap();
                if r == c {
                    let val:u32 = r.to_digit(10).unwrap();
                    sum += val;
                }
            },
            None => { break; }
        }
    }
    sum
}

fn string_rotations_phase_1<'a>(input: &'a String) -> (&'a str, String) {
    let normal_slice = input.as_str();
    let first_half = &input[0..1];
    let second_half = &input[1..input.len()];
    let inverse_slice = format!("{}{}", second_half, first_half);
    (normal_slice, inverse_slice)
}

fn inverse_captcha_phase_1(input: &String) -> u32 {
    inverse_captcha(input, string_rotations_phase_1)
}

fn string_rotations_phase_2<'a>(input: &'a String) -> (&'a str, String) {
    let normal_slice = input.as_str();
    let first_half = &input[0..input.len() / 2];
    let second_half = &input[input.len() / 2..input.len()];
    let inverse_slice = format!("{}{}", second_half, first_half);
    (normal_slice, inverse_slice)
}

fn inverse_captcha_phase_2(input: &String) -> u32 {
    inverse_captcha(input, string_rotations_phase_2)
}

#[test]
fn test_1122_phase_1() {
    assert_eq!(inverse_captcha_phase_1(&String::from("1122")), 3);
}

#[test]
fn test_1111_phase_1() {
    assert_eq!(inverse_captcha_phase_1(&String::from("1111")), 4);
}

#[test]
fn test_1234_phase_1() {
    assert_eq!(inverse_captcha_phase_1(&String::from("1234")), 0);
}

#[test]
fn test_91212129_phase_1() {
    assert_eq!(inverse_captcha_phase_1(&String::from("91212129")), 9);
}

#[test]
fn test_1212_phase_2() {
    assert_eq!(inverse_captcha_phase_2(&String::from("1212")), 6);
}

#[test]
fn test_1221_phase_2() {
    assert_eq!(inverse_captcha_phase_2(&String::from("1221")), 0);
}

#[test]
fn test_123425_phase_2() {
    assert_eq!(inverse_captcha_phase_2(&String::from("123425")), 4);
}

#[test]
fn test_123123_phase_2() {
    assert_eq!(inverse_captcha_phase_2(&String::from("123123")), 12);
}

#[test]
fn test_12131415_phase_2() {
    assert_eq!(inverse_captcha_phase_2(&String::from("12131415")), 4);
}
