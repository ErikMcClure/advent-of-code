use std::fs;
use std::io::BufRead;

fn parse_line(input: Result<String, std::io::Error>) -> [i64; 4] {
    let str = input.unwrap();
    let mut parts = str.split(" -> ");
    let mut p1 = parts.next().unwrap().split(',');
    let mut p2 = parts.next().unwrap().split(',');
    let line: [i64; 4] = [
        p1.next().unwrap().parse().unwrap(),
        p1.next().unwrap().parse().unwrap(),
        p2.next().unwrap().parse().unwrap(),
        p2.next().unwrap().parse().unwrap(),
    ];

    // Swap points if they go backwards
    if line[0] > line[2] || line[1] > line[3] {
        return [line[2], line[3], line[0], line[1]];
    }
    return line;
}

fn draw_board(b: &[u16], side: usize) {
    for i in 0..b.len() {
        if b[i] == 0 {
            print!(".");
        } else {
            print!("{}", b[i]);
        }
        if i % side == side - 1 {
            println!();
        }
    }
}

fn getsign(x: i64) -> i64 {
    if x < 0 {
        -1
    } else {
        1
    }
}

fn main() {
    let file = fs::File::open("aoc5.txt").expect("failed to open file");
    let lines = std::io::BufReader::new(file).lines().map(parse_line);
    let mut board: Vec<u16> = Vec::new();
    //let side = 10;
    let side = 1000;
    board.resize(side * side, 0);

    for [x1, y1, x2, y2] in lines {
        if x1 == x2 || y1 == y2 {
            for x in x1..=x2 {
                for y in y1..=y2 {
                    assert!(x < side as i64);
                    board[x as usize + y as usize * side] += 1;
                }
            }
        } else {
            let xi = getsign(x2 - x1);
            let yi = getsign(y2 - y1);
            let mut x = x1;
            let mut y = y1;
            loop {
                board[x as usize + y as usize * side] += 1;

                if x == x2 && y == y2 {
                    break;
                }
                x += xi;
                y += yi;
            }
        }
    }

    let mut count = 0;
    for i in &board {
        if *i >= 2 {
            count += 1;
        }
    }

    draw_board(&board, side);
    println!("count: {}", count);
}
