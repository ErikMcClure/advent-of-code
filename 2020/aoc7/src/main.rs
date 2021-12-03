use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;

fn parse_rule(x: Result<String, std::io::Error>) -> (String, HashMap<String, i32>) {
    lazy_static! {
        static ref RULE: Regex = Regex::new(r"([a-z]+ [a-z]+) bags? contain ([^.]+)\.").unwrap();
        static ref BAGS: Regex = Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bags?").unwrap();
    }

    let x = x.unwrap();
    let captures = RULE.captures(&x).unwrap();
    let id = captures.get(1).unwrap().as_str().to_string();
    let bags = captures.get(2).unwrap().as_str();
    if bags == "no other bags" {
        return (id, HashMap::new());
    }
    return (id,
            bags
            .split(',')
            .map(|x| -> (String, i32) {
                let caps = BAGS.captures(x).unwrap();
                (
                    caps.get(2).unwrap().as_str().to_string(),
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                )
            })
            .collect()
    );
}

fn main1() {
    let rules: HashMap<String, HashMap<String, i32>> =
        std::io::BufReader::new(std::fs::File::open("aoc7.txt").expect("Failed to open file"))
            .lines()
            .map(parse_rule)
            .collect();
    
    let mut bags : HashSet<&str> = HashSet::new();
    bags.insert("shiny gold");

    loop { // Find the fixed point of this function
        let count = bags.len();
        let checks = bags.clone();
        for ref r in rules.iter() {
            for b in checks.iter() {
                if r.1.contains_key(*b) {
                    bags.insert(r.0);
                }
            }
        }
        if count == bags.len() {
            break;
        }
    }

    println!("{}", bags.len() - 1);
}

fn main() {
    let rules: HashMap<String, HashMap<String, i32>> =
        std::io::BufReader::new(std::fs::File::open("aoc7.txt").expect("Failed to open file"))
            .lines()
            .map(parse_rule)
            .collect();

    let mut count = 0;
    let mut next : Vec<(&str, i32)> = Vec::new();
    next.push(("shiny gold", 1));

    while(next.len() > 0) {
        let level = next.clone();
        next.clear();
        for b in level {
            count += b.1;
            for rule in rules.get(b.0).expect("All referenced bags should exist!") {
                next.push((rule.0, rule.1 * b.1));
            }
        }
    }

    println!("{}", count - 1);
}