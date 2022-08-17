use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("src/04/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", high_entropy_passphrases_phase_1(contents));
    println!("Phase 2: {}", high_entropy_passphrases_phase_2(contents));
}

fn is_valid_passphrase_phase_1(passphrase: &str) -> bool {
    let mut used_words = HashSet::new();
    for w in passphrase.split_whitespace() {
        match used_words.get(w) {
            Some(_) => { return false },
            None => { used_words.insert(w); },
        }
    }
    true
}

fn is_valid_passphrase_phase_2(passphrase: &str) -> bool {
    let mut used_words = HashSet::new();
    let words = passphrase.split_whitespace().map(|w| w.chars().collect::<Vec<char>>());
    let words = words.map(|mut w| { w.sort(); w });
    for w in words {
        match used_words.get(&w) {
            Some(_) => { return false },
            None => { used_words.insert(w); },
        }
    }
    true
}

fn high_entropy_passphrases_phase_1(input: &String) -> usize {
    high_entropy_passphrases(input, is_valid_passphrase_phase_1)
}

fn high_entropy_passphrases_phase_2(input: &String) -> usize {
    high_entropy_passphrases(input, is_valid_passphrase_phase_2)
}

fn high_entropy_passphrases(input: &String, passphrase_checker: fn(&str) -> bool) -> usize {
    let good_passphrases = input.lines().filter(|l| passphrase_checker(*l));
    good_passphrases.collect::<Vec<&str>>().len()
}

#[test]
fn test_valid_passphrases() {
    assert_eq!(is_valid_passphrase_phase_1("aa bb cc dd ee"), true);
    assert_eq!(is_valid_passphrase_phase_1("aa bb cc dd aa"), false);
    assert_eq!(is_valid_passphrase_phase_1("aa bb cc dd aaa"), true);

    assert_eq!(is_valid_passphrase_phase_2("abcde fghij"), true);
    assert_eq!(is_valid_passphrase_phase_2("abcde xyz ecdab"), false);
    assert_eq!(is_valid_passphrase_phase_2("a ab abc abd abf abj"), true);
    assert_eq!(is_valid_passphrase_phase_2("iiii oiii ooii oooi oooo"), true);
    assert_eq!(is_valid_passphrase_phase_2("oiii ioii iioi iiio"), false);
}
