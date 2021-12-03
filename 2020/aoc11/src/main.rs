use std::io::BufRead;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum State {
    Floor = 0,
    Empty = 1,
    Occupied = 2,
}

fn neighbor(s: &Vec<Vec<State>>, x: usize, y: usize, dir: i32) -> bool {
    if dir == 4 {
        // This is the seat itself, which we don't count
        return false;
    }

    // Transform 0-8 dir into 2D coords ranging from -1 to 1
    let x = x as i32 + (dir % 3) - 1;
    let y = y as i32 + (dir / 3) - 1;
    if x < 0 || x as usize >= s[0].len() || y < 0 || y as usize >= s.len() {
        return false;
    }
    return s[y as usize][x as usize] == State::Occupied;
}

fn step(s: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut r: Vec<Vec<State>> = s.clone();
    for y in 0..s.len() {
        for x in 0..s[y].len() {
            let count = (0..9).fold(
                0,
                |acc, dir| if neighbor(s, x, y, dir) { acc + 1 } else { acc },
            );
            if s[y][x] == State::Empty && count == 0 {
                r[y][x] = State::Occupied;
            }
            if s[y][x] == State::Occupied && count >= 4 {
                r[y][x] = State::Empty;
            }
        }
    }
    return r;
}

fn dump(state: &Vec<Vec<State>>) {
    state.iter().for_each(|x| {
        let s = x
            .iter()
            .map(|c| match c {
                State::Occupied => '#',
                State::Empty => 'L',
                _ => '.',
            })
            .collect::<String>();
        println!("{}", s);
    });
}

fn count(state: &Vec<Vec<State>>) -> i32 {
    return state.iter().fold(0, |acc, x| {
        acc + x.iter().fold(0, |count, i| {
            if *i == State::Occupied {
                count + 1
            } else {
                count
            }
        })
    });
}

fn get_state(path: &str) -> Vec<Vec<State>> {
    return std::io::BufReader::new(std::fs::File::open(path).expect("Failed to open file"))
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| match c {
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => State::Floor,
                })
                .collect()
        })
        .collect();
}

pub fn main1() {
    let mut state: Vec<Vec<State>> = get_state("aoc11.txt");

    loop {
        let change = step(&state);
        //dump(&change);
        if change == state {
            break;
        }
        state = change;
    }

    println!("#: {}", count(&state));
}

fn neighbor2(s: &Vec<Vec<State>>, x: usize, y: usize, dir: i32) -> bool {
    if dir == 4 {
        // This is the seat itself, which we don't count
        return false;
    }

    // Transform 0-8 dir into 2D coords ranging from -1 to 1
    let mut x = x as i32;
    let mut y = y as i32;
    loop {
        x += (dir % 3) - 1;
        y += (dir / 3) - 1;
        if x < 0 || x as usize >= s[0].len() || y < 0 || y as usize >= s.len() {
            return false;
        }
        if s[y as usize][x as usize] == State::Floor {
            continue;
        }
        return s[y as usize][x as usize] == State::Occupied;
    }
}

fn step2(s: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut r: Vec<Vec<State>> = s.clone();
    for y in 0..s.len() {
        for x in 0..s[y].len() {
            let count = (0..9).fold(0, |acc, dir| {
                if neighbor2(s, x, y, dir) {
                    acc + 1
                } else {
                    acc
                }
            });
            if s[y][x] == State::Empty && count == 0 {
                r[y][x] = State::Occupied;
            }
            if s[y][x] == State::Occupied && count >= 5 {
                r[y][x] = State::Empty;
            }
        }
    }
    return r;
}

pub fn main() {
    let mut state: Vec<Vec<State>> = get_state("aoc11.txt");

    loop {
        let change = step2(&state);
        //dump(&change);
        if change == state {
            break;
        }
        state = change;
    }

    println!("#: {}", count(&state));
}
