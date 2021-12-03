use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

fn get_rule(s: &str) -> ((i32, i32, i32, i32), String) {
    lazy_static! {
        static ref RULE: Regex =
            Regex::new(r"([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
    }

    let captures = RULE.captures(&s).unwrap();
    return (
        (
            captures.get(2).unwrap().as_str().parse().unwrap(),
            captures.get(3).unwrap().as_str().parse().unwrap(),
            captures.get(4).unwrap().as_str().parse().unwrap(),
            captures.get(5).unwrap().as_str().parse().unwrap(),
        ),
        captures.get(1).unwrap().as_str().to_string(),
    );
}

fn validate(ticket: &Vec<i32>, rulemap: &HashMap<(i32, i32, i32, i32), String>) -> i32 {
    let mut errors = 0;
    'outer: for field in ticket {
        for rule in rulemap.keys() {
            if (*field >= rule.0 && *field <= rule.1) || (*field >= rule.2 && *field <= rule.3) {
                continue 'outer;
            }
        }
        errors += field;
    }
    return errors;
}

pub fn main1() {
    let mut contents = String::new();
    std::fs::File::open("aoc16.txt")
        .expect("Failed to open file")
        .read_to_string(&mut contents)
        .unwrap();

    let mut sections = contents.split("\n\n");
    let rulemap: HashMap<(i32, i32, i32, i32), String> =
        sections.next().unwrap().split('\n').map(get_rule).collect();

    let mut ticket = sections.next().unwrap().split('\n').skip(1);
    let ticket: Vec<i32> = ticket
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let data = sections.next().unwrap().split('\n').skip(1);
    let data: Vec<Vec<i32>> = data
        .map(|x| x.split(',').map(|c| c.parse().unwrap()).collect())
        .collect();

    let mut errors = 0;
    for ticket in data {
        let err = validate(&ticket, &rulemap);
        if err > 0 {
            errors += err;
            print!("Invalid ticket");
            for value in ticket {
                print!(" {}", value);
            }
            println!(";");
        }
    }

    println!("RESULT: {}", errors);
}

pub fn main() {
    let mut contents = String::new();
    std::fs::File::open("aoc16.txt")
        .expect("Failed to open file")
        .read_to_string(&mut contents)
        .unwrap();

    let mut sections = contents.split("\n\n");
    let rulemap: HashMap<(i32, i32, i32, i32), String> =
        sections.next().unwrap().split('\n').map(get_rule).collect();

    let mut mine = sections.next().unwrap().split('\n').skip(1);
    let mine: Vec<i32> = mine
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let data = sections.next().unwrap().split('\n').skip(1);
    let mut data: Vec<Vec<i32>> = data
        .map(|x| x.split(',').map(|c| c.parse().unwrap()).collect())
        .collect();

    //data.push(mine);
    let data = data.iter().filter(|x| validate(*x, &rulemap) == 0);

    let mut fullhash: HashSet<usize> = HashSet::new();
    for i in 0..rulemap.len() {
        fullhash.insert(i);
    }

    let mut track: HashMap<(i32, i32, i32, i32), HashSet<usize>> = HashMap::new();
    rulemap.iter().for_each(|x| {
        track.insert(*x.0, fullhash.clone());
    });

    for ticket in data {
        for i in 0..ticket.len() {
            let field = ticket[i];
            for rule in rulemap.keys() {
                if !((field >= rule.0 && field <= rule.1) || (field >= rule.2 && field <= rule.3)) {
                    track.get_mut(rule).unwrap().remove(&i);
                }
            }
        }
    }

    let mut results: HashMap<usize, (i32, i32, i32, i32)> = HashMap::new();

    while track.len() > 0 {
        let mut index = 99999999usize;
        for t in track.iter() {
            if t.1.len() == 1 {
                index = *t.1.iter().next().unwrap();
                results.insert(index, *t.0);
                break;
            }
            if t.1.len() == 0 {
                index = 999 + track.len();
                results.insert(index, *t.0);
                break;
            }
        }
        if index < 99999999usize {
            track.remove(&results[&index]);

            for t in rulemap.iter() {
                if let Some(element) = track.get_mut(t.0) {
                    element.remove(&index);
                }
            }
        }
    }

    for r in results {
        println!(
            "{}: {} {}",
            rulemap[&r.1],
            r.0,
            if r.0 < mine.len() { mine[r.0] } else { -1 }
        );
    }
}
