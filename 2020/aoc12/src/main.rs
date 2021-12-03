use std::io::BufRead;
use std::{collections::HashSet, ops::Add, ops::Rem};

fn operation(x: Result<String, std::io::Error>) -> (i32, i32, i32, i32) {
    let x = x.unwrap();
    let n = (&x[1..]).parse().unwrap();
    return match x.chars().next().unwrap() {
        'N' => (0, n, 0, 0),
        'S' => (0, -n, 0, 0),
        'E' => (n, 0, 0, 0),
        'W' => (-n, 0, 0, 0),
        'L' => (0, 0, n, 0),
        'R' => (0, 0, -n, 0),
        'F' => (0, 0, 0, n),
        _ => panic!("invalid command!"),
    };
}

// % is not modulo, it's remainder, we need correct modulo here
fn modulo<T>(a: T, b: T) -> T
where
    T: Rem<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    return ((a % b) + b) % b;
}

fn apply(ship: (i32, i32, i32), op: (i32, i32, i32, i32)) -> (i32, i32, i32) {
    let mut ship = (ship.0 + op.0, ship.1 + op.1, modulo(ship.2 + op.2, 360));

    match ship.2 / 90 {
        0 => ship.0 += op.3, // east
        1 => ship.1 += op.3, // north
        2 => ship.0 -= op.3, // west
        3 => ship.1 -= op.3, // south
        _ => panic!("Invalid direction! {}", ship.2),
    };
    return ship;
}

pub fn main1() {
    let mut result =
        std::io::BufReader::new(std::fs::File::open("aoc12.txt").expect("Failed to open file"))
            .lines()
            .map(operation)
            .fold((0, 0, 0), |ship, x| apply(ship, x));

    println!("{} {} {}", result.0, result.1, result.2);
}

struct Data {
    ship: (i32, i32),
    waypoint: (i32, i32),
}

fn apply2(ship: Data, op: (i32, i32, i32, i32)) -> Data {
    let mut ship = ship;
    ship.waypoint.0 += op.0;
    ship.waypoint.1 += op.1;

    ship.waypoint = match modulo(op.2, 360) / 90 {
        0 => ship.waypoint,                        // east
        1 => (-ship.waypoint.1, ship.waypoint.0),  // north
        2 => (-ship.waypoint.0, -ship.waypoint.1), // west
        3 => (ship.waypoint.1, -ship.waypoint.0),  // south
        _ => panic!("Invalid direction!"),
    };
    ship.ship.0 += ship.waypoint.0 * op.3;
    ship.ship.1 += ship.waypoint.1 * op.3;
    return ship;
}

pub fn main() {
    let start = Data {
        ship: (0, 0),
        waypoint: (10, 1),
    };

    let mut result =
        std::io::BufReader::new(std::fs::File::open("aoc12.txt").expect("Failed to open file"))
            .lines()
            .map(operation)
            .fold(start, |ship, x| apply2(ship, x));

    println!(
        "{} {} {} {}",
        result.ship.0, result.ship.1, result.waypoint.0, result.waypoint.1
    );
}
