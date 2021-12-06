use std::fs;
use std::io::BufRead;

fn main() {
    let file = fs::File::open("aoc6.txt").expect("failed to open file");
    let fish: Vec<usize> = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut counts: [usize; 9] = [0; 9];
    for f in fish {
        counts[f] += 1;
    }

    //for i in 0..80 {
    for i in 0..256 {
        let mut process: [usize; 9] = [0; 9];

        for j in 0..8 {
            process[j] = counts[j + 1];
        }
        process[6] += counts[0];
        process[8] += counts[0];

        counts = process;
        /*println!("day: {} - ", i);
        for i in 0..counts.len() {
            for j in 0..counts[i] {
                print!("{},", i);
            }
        }*/
    }

    println!("{}", counts.into_iter().fold(0, |a, e| a + e));
}
