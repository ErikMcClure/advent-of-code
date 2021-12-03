use std::io::BufRead;

fn seat(s : Result<String, std::io::Error>) -> i32 {
    let s = s.unwrap();
    let r = (&s[0..6]).chars().fold((0,128), |p, x| if x == 'F'{ (p.0, p.1 / 2) } else { (p.0 + (p.1 / 2), p.1 / 2) });
    let r = if s.chars().nth(6).unwrap() == 'B' { r.0 + 1 } else { r.0 };
    let c = (&s[7..9]).chars().fold((0,8), |p, x| if x == 'L'{ (p.0, p.1 / 2) } else { (p.0 + p.1 / 2, p.1 / 2) });
    let c = if s.chars().nth(9).unwrap() == 'R' { c.0 + 1 } else { c.0 };
    return (r*8) + c;
}
fn main1() {
    println!("{}", std::io::BufReader::new(std::fs::File::open("aoc5.txt").expect("Failed to open file")).lines().map(seat).fold(-1, |a, x| if a > x { a } else { x }));
}
fn main() {
    let mut n: Vec<i32> = std::io::BufReader::new(std::fs::File::open("aoc5.txt").expect("Failed to open file")).lines().map(seat).collect();
    n.sort();
    for i in 1..n.len() {
        if n[i-1] + 1 != n[i] {
            println!("{} {}", n[i-1], n[i]);
        }
    }
}