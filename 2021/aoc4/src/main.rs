use std::collections::HashSet;
use std::fs;
use std::io::BufRead;

fn check_winner(board: &[i32; 25]) -> bool {
    for y in 0..5 {
        if board[(y * 5)..(y * 5) + 5].iter().all(|x| *x < 0) {
            return true;
        }
    }

    for x in 0..5 {
        let mut count = 0;
        for y in 0..5 {
            if board[x + (y * 5)] < 0 {
                count += 1;
            }
        }
        if count == 5 {
            return true;
        }
    }

    return false;
}

fn get_score(board: &[i32; 25]) -> i32 {
    board.iter().fold(0, |a, x| if *x > 0 { a + *x } else { a })
}

fn main() {
    let file = fs::File::open("aoc4.txt").expect("failed to open file");
    let mut numbers: Vec<i32> = Vec::new();
    let mut boards: Vec<[i32; 25]> = Vec::new();
    let mut cur: Vec<i32> = Vec::new();

    for line in std::io::BufReader::new(file).lines() {
        let l = line.unwrap();
        if numbers.is_empty() {
            numbers = l.split(',').map(|x| x.parse().unwrap()).collect();
        } else if l.is_empty() {
            if cur.len() > 0 {
                boards.push(cur.try_into().unwrap());
                cur = Vec::new();
            }
        } else {
            for n in l.split_whitespace() {
                cur.push(n.parse().unwrap());
            }
        }
    }
    if cur.len() > 0 {
        boards.push(cur.try_into().unwrap());
    }

    println!("{} {}", numbers.len(), boards.len());

    let mut winners = HashSet::new();
    for n in numbers {
        for i in 0..boards.len() {
            for e in &mut boards[i] {
                if *e == n {
                    *e = -1;
                }
            }

            if !winners.contains(&i) && check_winner(&boards[i]) {
                let score = get_score(&boards[i]);

                winners.insert(i);
                if winners.len() >= boards.len() {
                    println!(
                        "LAST WINNER: {}, SCORE: {} NUMBER: {} ANSWER: {}",
                        i,
                        score,
                        n,
                        score * n
                    );
                    return;
                }
                // Part 1
                //println!("LAST WINNER: {}, SCORE: {} NUMBER: {} ANSWER: {}", i, score, n, score*n)
                //return;
            }
        }
    }
    println!("NO WINNER???");
}
