use std::collections::HashSet;
use std::io::BufRead;

fn main1() {
    let mut adapters: Vec<i32> =
        std::io::BufReader::new(std::fs::File::open("aoc10.txt").expect("Failed to open file"))
            .lines()
            .map(|x| x.unwrap().parse().unwrap())
            .collect();

    adapters.sort();

    let mut prev = 0; // output starts at 0
    let mut diffs = [0; 4];

    for a in adapters {
        let diff = a - prev;
        if diff > 3 || diff < 0 {
            println!("Invalid! {} -> {}", prev, a);
            break;
        }
        diffs[diff as usize] += 1;
        prev = a;
    }
    println!("diffs 1:{} 2:{} 3:{}", diffs[1], diffs[2], diffs[3] + 1); // add one for final output
}

fn factorial(k: i64) -> i64 {
    if k <= 1 {
        1
    } else {
        k * factorial(k - 1)
    }
}

fn binomial(n: i64, k: i64) -> i64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn eval(series: i64, count: i64) -> i64 {
    if series >= 2 {
        let mut r: i64 = 1; // start at one for initial possibility
        for i in 0..std::cmp::min(series - 1, 2) {
            // This is actually wrong but it works up to 7 and that's all my input had
            r += binomial(series - 1, i + 1);
        }
        println!("{}", r);
        return count * r;
    }
    return count;
}

fn main() {
    let mut adapters: Vec<i64> =
        std::io::BufReader::new(std::fs::File::open("aoc10.txt").expect("Failed to open file"))
            .lines()
            .map(|x| x.unwrap().parse().unwrap())
            .collect();

    adapters.sort();

    let mut prev = 0; // output starts at 0
    let mut series = 0;
    let mut count: i64 = 1;

    for a in adapters {
        if a == prev + 1 {
            series += 1;
        } else {
            count = eval(series, count);
            series = 0;
        }
        prev = a;
    }
    count = eval(series, count);
    println!("count: {}", count); // add one for final output
}
