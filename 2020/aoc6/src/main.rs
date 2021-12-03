use std::collections::HashSet;
use std::io::Read;

fn main1() {
    let mut contents = String::new();
    std::fs::File::open("aoc6.txt")
        .expect("Failed to open file")
        .read_to_string(&mut contents)
        .unwrap();
    println!(
        "{}",
        contents
            .split("\n\n")
            .map(|x| {
                let mut c: HashSet<char> = x.chars().collect();
                c.remove(&'\n');
                c
            })
            .fold(0, |acc, x| acc + x.len())
    );
}

fn main() {
    let mut contents = String::new();
    std::fs::File::open("aoc6.txt")
        .expect("Failed to open file")
        .read_to_string(&mut contents)
        .unwrap();
    println!(
        "{}",
        contents
            .split("\n\n")
            .map(|x| x
                .split_whitespace()
                .map(|x| -> HashSet<char> { x.chars().collect() })
                .fold(
                    "abcdefghijklmnopqrstuvwxyz".chars().collect(),
                    |acc, x| -> HashSet<char> { acc.intersection(&x).cloned().collect() }
                ))
            .fold(0, |acc, x| acc + x.len())
    );
}
