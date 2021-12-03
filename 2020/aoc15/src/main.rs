use std::collections::HashMap;

fn main() {
    let numbers = [1, 20, 11, 6, 12, 0];

    let mut memory = HashMap::new();
    let mut turn = 1;
    let mut spoken = numbers[0];
    for n in numbers.iter().skip(1) {
        memory.insert(spoken, turn);
        spoken = *n;
        turn += 1;
    }

    while turn < 30000000 {
        let cur;
        if let Some(element) = memory.get(&spoken) {
            cur = turn - element;
        } else {
            cur = 0;
        }
        //println!("Turn {}: {} -> {}", turn, spoken, cur);
        memory.insert(spoken, turn);
        spoken = cur;
        turn += 1;
    }

    println!("{}", spoken);
}
