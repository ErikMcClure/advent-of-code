use std::collections::HashMap;
use std::io::BufRead;
#[derive(PartialEq, Clone)]
pub enum Atom {
    Empty,
    Index(i32),
    Match(String),
}

#[derive(PartialEq, Clone)]
struct Rule {
    left: Vec<Atom>,
    right: Vec<Atom>,
}

fn parse_rule(x: Result<String, std::io::Error>) -> (i32, Rule) {
    let mut x: &str = &x.unwrap();
    if x == "8: 42" {
        x = "8: 42 | 42 8";
    }
    if x == "11: 42 31" {
        x = "11: 42 31 | 42 11 31";
    }
    let mut parts = x.split(':');
    let id: i32 = parts.next().unwrap().parse().unwrap();
    let mut left: Vec<Atom> = Vec::new();
    let mut right: Vec<Atom> = Vec::new();
    let mut cur = &mut left;

    for part in parts.next().unwrap().trim().split_ascii_whitespace() {
        if part == "|" {
            cur = &mut right;
            continue;
        }
        let num = part.parse();
        cur.push(match num {
            Ok(x) => Atom::Index(x),
            Err(_) => Atom::Match(
                part.strip_prefix('"')
                    .unwrap()
                    .strip_suffix('"')
                    .unwrap()
                    .to_string(),
            ),
        });
    }
    return (
        id,
        Rule {
            left: left,
            right: right,
        },
    );
}

fn match_atom<'a>(
    atom: &Atom,
    prev: (bool, &'a str),
    hash: &HashMap<i32, Rule>,
) -> (bool, &'a str) {
    if !prev.0 {
        return prev;
    }

    return match atom {
        Atom::Empty => (true, prev.1),
        Atom::Index(id) => match_rule(*id, prev.1, hash),
        Atom::Match(m) => {
            let check = prev.1.starts_with(m);
            //println!("{}?{} {}", m, prev.1, check);
            (check, if !check { prev.1 } else { &prev.1[m.len()..] })
        }
    };
}

fn match_rule<'a>(id: i32, s: &'a str, hash: &HashMap<i32, Rule>) -> (bool, &'a str) {
    let rule = &hash[&id];

    //println!("rule {}: {}", id, s);
    let left = rule
        .left
        .iter()
        .fold((true, s), |prev, x| match_atom(x, prev, hash));

    let right = rule
        .right
        .iter()
        .fold((true, s), |prev, x| match_atom(x, prev, hash));

    if rule.right.len() > 0 && right.0 && !left.0 {
        return right;
    } else {
        return left;
    };
}

pub fn main() {
    let rules: HashMap<i32, Rule> = std::io::BufReader::new(
        std::fs::File::open("aoc19_rules.txt").expect("Failed to open file"),
    )
    .lines()
    .map(parse_rule)
    .collect();

    let lines = std::io::BufReader::new(
        std::fs::File::open("aoc19_input.txt").expect("Failed to open file"),
    )
    .lines()
    .map(|x| {
        let x = x.unwrap();
        let m = match_rule(0, &x, &rules);
        (x.to_string(), m.0 && m.1.len() == 0)
    });

    let mut count = 0;
    for line in lines {
        //println!("{}: {}", line.0, line.1);
        if line.1 {
            count += 1;
        }
    }
    println!("count: {}", count);
}
