use std::io::BufRead;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

const Fields:[&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main2() {
    let file = fs::File::open("aoc4.txt").expect("failed to open file");
    let mut passports:Vec<HashMap<String, String>> = Vec::new();
    passports.push(HashMap::new());

    for line in std::io::BufReader::new(file).lines() {
        let x = line.unwrap();
        if x.is_empty() {
            passports.push(HashMap::new());
        } else {
            for entry in x.split(' ') {
                let mut values = entry.split(':');
                passports.last_mut().unwrap().insert(values.next().unwrap().to_string(), values.next().unwrap().to_string());
            }
        }
    }
    
    let mut valid = 0;
    'outer: for passport in passports {
        for field in Fields.iter().copied() {
            if !passport.contains_key(field) {
                continue 'outer;
            }
        }
        valid += 1;
    }

    println!("{}", valid)
}

fn validate_year(passport : &HashMap<String, String>, s : &str, low : i32, high : i32) -> bool {
    let v = passport.get(s);
    if v.is_none() {
        return false;
    }
    let n : i32 = v.unwrap().parse().unwrap();
    return n >= low && n <= high;
}

fn validate_regex(passport : &HashMap<String, String>, s : &str, pattern : &str) -> bool {
    let v = passport.get(s);
    if v.is_none() {
        return false;
    }
    let r = Regex::new(pattern).unwrap();
    return r.is_match(v.unwrap().trim());
}

fn main() {
    let file = fs::File::open("aoc4.txt").expect("failed to open file");
    let mut passports:Vec<HashMap<String, String>> = Vec::new();
    passports.push(HashMap::new());

    for line in std::io::BufReader::new(file).lines() {
        let x = line.unwrap();
        if x.is_empty() {
            passports.push(HashMap::new());
        } else {
            for entry in x.split(' ') {
                let mut values = entry.split(':');
                passports.last_mut().unwrap().insert(values.next().unwrap().to_string(), values.next().unwrap().to_string());
            }
        }
    }
    
    let mut valid = 0;
    'outer: for passport in passports {
        if !validate_year(&passport,"byr", 1920, 2002) ||
            !validate_year(&passport,"iyr", 2010, 2020) ||
            !validate_year(&passport,"eyr", 2020, 2030) ||
            !validate_regex(&passport, "hgt", r"^[0-9]+(in|cm)$") ||
            !validate_regex(&passport, "hcl", r"^\#[0-9a-fA-F]{6}$") ||
            !validate_regex(&passport, "ecl", r"^(amb|blu|brn|gry|grn|hzl|oth)$") ||
            !validate_regex(&passport, "pid", r"^[0-9]{9}$")
        {
            continue 'outer;
        }
        let r = Regex::new("^([0-9]+)(in|cm)$").unwrap();
        let captures = r.captures(passport.get("hgt").unwrap()).unwrap();
        let hgt:i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let dim = captures.get(2).unwrap().as_str();
        if (dim == "cm" && hgt >= 150 && hgt <= 193) || (dim == "in" && hgt >= 59 && hgt <= 76) {
            valid += 1;
        }
    }

    println!("{}", valid)
}