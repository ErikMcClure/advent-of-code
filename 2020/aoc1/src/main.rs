use std::io::BufRead;

fn main2() {
    let mut n: Vec<i32> =
        std::io::BufReader::new(std::fs::File::open("aoc1.txt").expect("Failed to open file"))
            .lines()
            .map(|x| {
                x.expect("Failed to get line")
                    .parse()
                    .expect("failed to parse number")
            })
            .collect();
    n.sort();

    let mut r = n.len() - 1;
    let mut l = 0;
    while r > l {
        let v = n[l] + n[r];
        if v == 2020 {
            break;
        } else if v > 2020 {
            r -= 1;
        } else if v < 2020 {
            l += 1;
        }
    }

    println!("{}", n[l] * n[r]);
}

fn main() {
    let mut n: Vec<i32> =
        std::io::BufReader::new(std::fs::File::open("aoc1.txt").expect("Failed to open file"))
            .lines()
            .map(|x| {
                x.expect("Failed to get line")
                    .parse()
                    .expect("failed to parse number")
            })
            .collect();
    n.sort();

    let mut r = n.len() - 1;
    while n[0] + n[1] + n[r] > 2020 {
        r -= 1;
    }

    let mut l = 0;
    let mut m = 1;
    'outer: while r > 0 {
        l = 0;
        while l + 1 < r && n[l] + n[l + 1] + n[r] <= 2020 {
            m = l + 1;
            while m < r && n[l] + n[m] + n[r] <= 2020 {
                if n[l] + n[m] + n[r] == 2020 {
                    break 'outer;
                }
                m += 1;
            }
            l += 1;
        }
        r -= 1;
    }

    println!("n[{}] * n[{}] * n[{}] = {}", l, m, r, n[l] * n[m] * n[r]);
}
struct Bar {
    int quux;
}
struct Foo {
    Bar bar;
}
struct Obj {
    Foo foo;
}

fn test() {
    std::Vec<Obj> objs;
    objs.
}