use std::io::BufRead;

const LOOKBACK: usize = 25;

pub fn aoc() -> i64 {
    let n: Vec<i64> =
        std::io::BufReader::new(std::fs::File::open("aoc9.txt").expect("Failed to open file"))
            .lines()
            .map(|x| x.unwrap().parse().unwrap())
            .collect();

    let mut invalid: i64 = 0;
    'outer: for i in LOOKBACK..n.len() {
        for j in i - LOOKBACK..i {
            for k in j + 1..i {
                if n[j] + n[k] == n[i] {
                    continue 'outer;
                }
            }
        }
        invalid = n[i];
        break;
    }

    let n: Vec<i64> =
        std::io::BufReader::new(std::fs::File::open("aoc9.txt").expect("Failed to open file"))
            .lines()
            .map(|x| x.unwrap().parse().unwrap())
            .collect();

    for i in 0..n.len() {
        for j in i + 1..n.len() {
            let sum = n[i..j].iter().fold(0, |acc, x| acc + x);
            if sum == invalid {
                let small = n[i..j].iter().min().unwrap();
                let large = n[i..j].iter().max().unwrap();
                return small + large;
            } else if sum > invalid {
                break;
            }
        }
    }

    return -1;
}
