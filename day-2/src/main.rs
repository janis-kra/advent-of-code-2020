use std::fs;
use regex::Regex;

struct Policy {
    first_option: i32,
    second_option: i32,
    letter: String
}

fn main() {
    let puzzle = read_puzzle();
    let mut matching_password_amount = 0;
    for line in puzzle.lines() {
        let policy = get_policy(line);
        let password = get_password(line);
        if policy.is_some() && password != "not found" {
            if is_match(password, policy.unwrap()) {   
                println!("Password Match! {}", password);
                matching_password_amount += 1;
            } else {
                println!("Password Rejected! {}", password);
            }
        }
    }
    println!("Matching passwords total: {}", matching_password_amount);
}

fn read_puzzle() -> String {
    println!("Reading puzzle input...");
    fs::read_to_string("puzzle")
        .expect("Error :(")
}

fn get_policy(line: &str) -> Option<Policy> {
    let re = Regex::new(r"^([1-9]+[0-9]*)\-([1-9]+[0-9]*)\s([a-z])").unwrap();
    if re.is_match(line) {
        let cap = re.captures(line).unwrap();
        let min_amount = cap.get(1).unwrap().as_str();
        let max_amount = cap.get(2).unwrap().as_str();
        let letter = cap.get(3).unwrap().as_str();
        let policy = Policy {
            first_option: min_amount.parse::<i32>().unwrap(),
            second_option: max_amount.parse::<i32>().unwrap(),
            letter: String::from(letter)
        };
        Some(policy)
    } else {
        None
    }
}

fn get_password(line: &str) -> &str {
    let re = Regex::new(r"[a-z]{2,}$").unwrap();
    if re.is_match(line) {
        let cap = re.captures(line).unwrap();
        cap.get(0).unwrap().as_str()    
    } else {
        "not found"
    }
}

fn is_match(password: &str, policy: Policy) -> bool {
    is_matching_letter(password, policy.first_option as usize, policy.letter.as_str())
        ^ is_matching_letter(password, policy.second_option as usize, policy.letter.as_str())
}

fn is_matching_letter(password: &str, at: usize, letter: &str) -> bool {
    let char = password.chars().nth(at - 1);
    char.is_some() && char.unwrap().to_string() == letter
}

