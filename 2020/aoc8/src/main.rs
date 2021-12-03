use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
    Acc,
    Nop,
    Jmp,
}

fn parse_ins(x: Result<String, std::io::Error>) -> (Instruction, i32) {
    lazy_static! {
        static ref RULE: Regex = Regex::new(r"(acc|nop|jmp) (\+[0-9]+|\-[0-9]+)").unwrap();
    }

    let x = x.unwrap();
    let captures = RULE.captures(&x).unwrap();

    let ins = match captures.get(1).unwrap().as_str() {
        "acc" => Instruction::Acc,
        "nop" => Instruction::Nop,
        "jmp" => Instruction::Jmp,
        &_ => Instruction::Nop,
    };

    return (ins, captures.get(2).unwrap().as_str().parse().unwrap());
}

fn main1() {
    let program: Vec<(Instruction, i32)> =
        std::io::BufReader::new(std::fs::File::open("aoc8.txt").expect("Failed to open file"))
            .lines()
            .map(parse_ins)
            .collect();

    let mut counter: i32 = 0;
    let mut executed: HashSet<i32> = HashSet::new();
    let mut acc = 0;

    while counter < program.len() as i32 {
        if executed.contains(&counter) {
            println!("{}", acc);
            break;
        }
        executed.insert(counter);

        let ins = &program[counter as usize];
        println!("{}: ({:?} {})", counter, ins.0, ins.1);
        match &ins.0 {
            Instruction::Acc => {
                counter += 1;
                acc += ins.1
            }
            Instruction::Jmp => counter += ins.1,
            Instruction::Nop => counter += 1,
        }
    }

    println!("done\n");
}

fn execute(program: &Vec<(Instruction, i32)>, history: &mut Vec<i32>) -> i32 {
    let mut counter: i32 = 0;
    let mut executed: HashSet<i32> = HashSet::new();
    let mut acc = 0;

    while counter < program.len() as i32 {
        if executed.contains(&counter) {
            return acc;
        }
        executed.insert(counter);

        history.push(counter);
        let ins = &program[counter as usize];
        match &ins.0 {
            Instruction::Acc => {
                counter += 1;
                acc += ins.1
            }
            Instruction::Jmp => counter += ins.1,
            Instruction::Nop => counter += 1,
        }
    }

    history.clear();
    return acc;
}

fn swap(program: &mut Vec<(Instruction, i32)>, i: i32) -> bool {
    match program[i as usize].0 {
        Instruction::Acc => return false,
        Instruction::Jmp => program[i as usize].0 = Instruction::Nop,
        Instruction::Nop => program[i as usize].0 = Instruction::Jmp,
    }
    return true;
}

fn main2() {
    let mut program: Vec<(Instruction, i32)> =
        std::io::BufReader::new(std::fs::File::open("aoc8.txt").expect("Failed to open file"))
            .lines()
            .map(parse_ins)
            .collect();

    let mut history: Vec<i32> = Vec::new();
    execute(&program, &mut history);
    for h in history.iter().rev() {
        if swap(&mut program, *h) {
            let mut buf: Vec<i32> = Vec::new();
            let acc = execute(&program, &mut buf);
            println!("acc: {}", acc);
            if buf.len() == 0 {
                break;
            }
            swap(&mut program, *h);
        }
    }
    println!("done!");
}
