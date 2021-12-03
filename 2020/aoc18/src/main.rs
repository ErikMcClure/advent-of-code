use std::io::BufRead;

fn num<I>(tokens: &mut I) -> i64
where
    I: Iterator<Item = char>,
{
    return match tokens.next() {
        Some('(') => expr(tokens),
        Some(x) => x.to_digit(10).unwrap() as i64,
        None => panic!("Expected expression but found nothing!"),
    };
}

fn expr<I>(tokens: &mut I) -> i64
where
    I: Iterator<Item = char>,
{
    let mut l = num(tokens);

    loop {
        match tokens.next() {
            Some('*') => l = l * num(tokens),
            Some('+') => l = l + num(tokens),
            Some(')') => return l,
            None => return l,
            Some(x) => panic!("Invalid operator: {}", x),
        };
    }
}

pub fn main1() {
    let result =
        std::io::BufReader::new(std::fs::File::open("aoc18.txt").expect("Failed to open file"))
            .lines()
            .map(|x| expr(&mut x.unwrap().chars().filter(|c| !c.is_whitespace())))
            .fold(0i64, |total, x| total + x);

    println!("Result: {}", result);
}

fn num2<I>(cur: &mut Option<char>, tokens: &mut I) -> i64
where
    I: Iterator<Item = char>,
{
    *cur = tokens.next();
    return match *cur {
        Some('(') => mul(cur, tokens),
        Some(x) => x.to_digit(10).unwrap() as i64,
        None => panic!("Expected expression but found nothing!"),
    };
}

fn add<I>(cur: &mut Option<char>, tokens: &mut I) -> i64
where
    I: Iterator<Item = char>,
{
    let mut l = num2(cur, tokens);

    loop {
        *cur = tokens.next();
        match cur {
            Some('+') => l = l + num2(cur, tokens),
            Some('*') => return l,
            Some(')') => return l,
            None => return l,
            Some(x) => panic!("Invalid operator: {}", x),
        };
    }
}

fn mul<I>(cur: &mut Option<char>, tokens: &mut I) -> i64
where
    I: Iterator<Item = char>,
{
    let mut l = add(cur, tokens);

    loop {
        match cur {
            Some('*') => l = l * add(cur, tokens),
            Some(')') => return l,
            None => return l,
            Some(x) => panic!("Invalid operator: {}", x),
        };
    }
}

pub fn main() {
    let mut cur: Option<char> = None;
    let result =
        std::io::BufReader::new(std::fs::File::open("aoc18.txt").expect("Failed to open file"))
            .lines()
            .map(|x| {
                mul(
                    &mut cur,
                    &mut x.unwrap().chars().filter(|c| !c.is_whitespace()),
                )
            })
            .fold(0i64, |total, x| total + x);

    println!("Result: {}", result);
}
