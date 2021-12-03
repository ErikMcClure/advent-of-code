use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

fn parse_list(list: &mut HashMap<String, HashSet<String>>, x: &str) -> HashSet<String> {
    let mut parts = x.split(" (contains ");
    let ingredients: HashSet<String> = parts
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    let allergens = parts
        .next()
        .unwrap()
        .trim_end_matches(')')
        .split(',')
        .map(|x| x.trim());

    for a in allergens {
        if let Some(x) = list.get_mut(a) {
            *x = x
                .intersection(&ingredients)
                .map(|x| x.to_string())
                .collect();
        } else {
            list.insert(a.to_string(), ingredients.clone());
        }
    }

    return ingredients;
}

pub fn main() {
    let mut list = HashMap::new();

    let mut ingredients: HashMap<String, i32> = HashMap::new();

    std::io::BufReader::new(std::fs::File::open("aoc21.txt").expect("Failed to open file"))
        .lines()
        .for_each(|x| {
            let buf = parse_list(&mut list, &x.unwrap());
            for i in buf {
                if let Some(x) = ingredients.get_mut(&i) {
                    *x += 1;
                } else {
                    ingredients.insert(i, 1);
                }
            }
        });

    for v in list.values() {
        for i in v {
            ingredients.remove(i);
        }
    }

    /*println!(
        "\nTOTAL: {}",
        ingredients.iter().fold(0, |acc, x| acc + x.1)
    );*/

    for v in list.iter_mut() {
        for i in &ingredients {
            v.1.remove(i.0);
        }

        /*for i in v.1.iter() {
            println!("{}: {}", v.0, i);
        }*/
    }

    let mut allergens = HashMap::new();
    let count = list.len();
    while allergens.len() < count {
        let mut a = None;
        for i in list.iter() {
            if i.1.len() == 1 {
                a = i.1.iter().next().cloned();
                if let Some(x) = &a {
                    allergens.insert(x.to_string(), i.0.clone());
                }
                break;
            }
        }
        if let Some(x) = &a {
            for i in list.values_mut() {
                i.remove(x);
            }
        }
    }

    for a in allergens {
        println!("{}: {}", a.0, a.1);
    }
}
