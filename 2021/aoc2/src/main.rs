use std::fs;
use std::io::BufRead;

fn parse_to_pair(input: Result<String, std::io::Error>) -> (i32, i32) {
    let s = input.expect("failed to parse string");
    let mut parts = s.split_whitespace();
    let dir = match parts.next().expect("expected instruction") {
        "forward" => 0,
        "up" => 1,
        "down" => -1,
        _ => panic!(),
    };

    return (dir, parts.next().unwrap().parse().unwrap());
}

fn main2() {
    let file = fs::File::open("aoc2.txt").expect("failed to open file");
    let data = std::io::BufReader::new(file).lines().map(parse_to_pair);

    let mut x = 0;
    let mut y = 0;
    for (d, a) in data {
        match d {
            0 => x += a,
            1 => y -= a,
            -1 => y += a,
            _ => panic!(),
        }
    }

    println!("x: {} depth: {} multiply: {}", x, y, x * y);
}

// part 2
fn main() {
    let file = fs::File::open("aoc2.txt").expect("failed to open file");
    let data = std::io::BufReader::new(file).lines().map(parse_to_pair);

    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for (d, a) in data {
        match d {
            0 => {
                x += a;
                y += aim * a;
            }
            1 => aim -= a,
            -1 => aim += a,
            _ => panic!(),
        }
    }

    println!("x: {} depth: {} aim: {}, multiply: {}", x, y, aim, x * y);
}
