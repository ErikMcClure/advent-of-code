use std::io::BufRead;
use std::fs;

fn parse_to_int(input : Result<String, std::io::Error>) -> u32 {
    let mut i:u32 = 0;
    let mut r:u32 = 0;

    for c in input.unwrap().chars() {
        if c == '#' {
            r |= 1<<i;
        }
        i += 1;
    }

    return r;
}
const LINE_LEN:usize = 31;

fn hit(x : usize, y : usize, s : &[u32]) -> bool {
    /*let mut debug = String::with_capacity(32);
    for i in 0..31 {
        if s[y] & (1 << i) != 0 {
            debug.push('#')
        } else {
            debug.push('.')
        }
    }
    println!("{}", debug);*/
    return (s[y] & (1 << (x % LINE_LEN))) != 0;
}

fn slope(x : usize, y : usize, s : &[u32]) -> usize {
    let count = s.len();
    let mut posx = 0usize;
    let mut posy = 0usize;
    let mut trees = 0;
    while posy < count {
        if hit(posx, posy, s) {
            trees += 1;
        }
        posx += x;
        posy += y;
    }
    return trees;
}

fn main() {
    let file = fs::File::open("aoc3.txt").expect("failed to open file");
    let data:Vec<u32> = std::io::BufReader::new(file).lines().map(parse_to_int).collect();

    let a = slope(1, 1, &data);
    let b = slope(3, 1, &data);
    let c = slope(5, 1, &data);
    let d = slope(7, 1, &data);
    let e = slope(1, 2, &data);
    
    println!("{} {} {} {} {}: {}", a, b, c, d, e, a*b*c*d*e);
}