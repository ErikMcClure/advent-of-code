use std::fs;
use std::io::BufRead;

fn parse_int(input: Result<String, std::io::Error>) -> u64 {
    return u64::from_str_radix(&input.unwrap(), 2).unwrap();
}

fn most_common_bit(input: &[u64], bit: u32, tie: bool) -> u64 {
    let mut zero = 0;
    let mut one = 0;

    for i in input {
        if (i & (1 << bit)) != 0 {
            one += 1;
        } else {
            zero += 1;
        }
    }

    println!("bit: {} zeros: {} ones: {}", bit, zero, one);
    return if zero > one {
        0
    } else if one > zero {
        1
    } else if tie {
        1
    } else {
        0
    };
}

fn main2() {
    let file = fs::File::open("aoc3.txt").expect("failed to open file");
    let data: Vec<u64> = std::io::BufReader::new(file)
        .lines()
        .map(parse_int)
        .collect();

    let mut gamma: u64 = 0;
    for i in 0..15 {
        gamma = gamma | (most_common_bit(&data, i, false) << i)
    }

    println!(
        "{} {}: {}",
        gamma,
        (!gamma & 0b111111111111),
        gamma * (!gamma & 0b111111111111)
    );
}

// part 2
fn main() {
    let file = fs::File::open("aoc3.txt").expect("failed to open file");
    let mut data: Vec<u64> = std::io::BufReader::new(file)
        .lines()
        .map(parse_int)
        .collect();

    let mut data2 = data.clone();

    for i in (0..12).rev() {
        let bit = most_common_bit(&data, i, true);
        data = data
            .into_iter()
            .filter(|x| return (x & (1 << i)) >> i == bit)
            .collect();
        if data.len() == 1 {
            break;
        }
    }

    println!("BREAK");

    for i in (0..12).rev() {
        let bit = 1 - most_common_bit(&data2, i, true);
        data2 = data2
            .into_iter()
            .filter(|x| return (x & (1 << i)) >> i == bit)
            .collect();
        println!("size: {}", data2.len());
        if data2.len() == 1 {
            break;
        }
    }

    println!("{} {}: {}", data[0], data2[0], data[0] * data2[0]);
}
