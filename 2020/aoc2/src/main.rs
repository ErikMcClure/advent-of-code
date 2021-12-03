use std::io::BufRead;
use std::fs;

fn parse_to_struct(input : Result<String, std::io::Error>) -> (usize, usize, char, String) {
    let s = input.expect("failed to parse string");
    let mut parts = s.split(':');
    let mut policy = parts.next().unwrap().split(' ');
    let password = parts.next().unwrap().trim();

    let mut range = policy.next().unwrap().split('-');
    let c = policy.next().unwrap().chars().next().unwrap();
    return (range.next().unwrap().parse().expect("failed to parse number"),
        range.next().unwrap().parse().expect("failed to parse number"), 
        c,
        password.to_string())
}

fn main2() {
    let file = fs::File::open("aoc2.txt").expect("failed to open file");
    let data = std::io::BufReader::new(file).lines().map(parse_to_struct);
    
    let mut valid = 0;
    for d in data {
        let c = d.3.matches(d.2).count();
        if c >= d.0 && c <= d.1 {
            valid += 1;
        }
    }

    println!("valid: {}", valid);
}

fn main() {
    let file = fs::File::open("aoc2.txt").expect("failed to open file");
    let data = std::io::BufReader::new(file).lines().map(parse_to_struct);
    
    let mut valid = 0;
    for d in data {
        let a = d.3.chars().nth(d.0 - 1).unwrap_or_else(|| '\0');
        let b = d.3.chars().nth(d.1 - 1).unwrap_or_else(|| '\0');
        if (a == d.2) ^ (b == d.2) {
            valid += 1;
        }
    }

    println!("valid: {}", valid);
}