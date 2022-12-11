#![feature(int_roundings)]
use std::fs;

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    divisible_by: i32,
    if_true: usize,
    if_false: usize,
    count: u32,
}

impl Monkey {
    fn parse(lines: &mut std::str::Lines) -> Option<Monkey> {
        let mut monkey = Monkey {
            items: Vec::new(),
            operation: Box::new(|a| a),
            divisible_by: 3,
            if_true: 0,
            if_false: 0,
            count: 0
        };
        let mut lines_read = 0;
        loop {
            let res = lines.next();
            if res.is_none() { break };
            let line_str = res.unwrap();
            if line_str.is_empty() { break; }
            let line = line_str.trim().split(" ").collect::<Vec<&str>>();
            lines_read += 1;
            match line[..] {
                ["Monkey", ..] => {},
                ["Starting", "items:", ..] => {
                   for i in 2..line.len() {
                       monkey.items.push(line[i].replace(",", "").parse().unwrap());
                   }
                },
                ["Operation:", "new", "=", "old", operator, value] => {
                    let value: Option<i32> = if value == "old" {
                        None
                    } else {
                        Some(value.parse().unwrap())
                    };
                    match operator {
                        "*" => monkey.operation = Box::new(move |x| x * value.unwrap_or(x)),
                        "/" => monkey.operation = Box::new(move |x| x / value.unwrap_or(x)),
                        "+" => monkey.operation = Box::new(move |x| x + value.unwrap_or(x)),
                        "-" => monkey.operation = Box::new(move |x| x - value.unwrap_or(x)),
                        _ => eprintln!("Error: operator {operator} not supported")
                    }
                },
                ["Test:", "divisible", "by", value] => 
                    monkey.divisible_by = value.parse().unwrap(),
                ["If", "true:", "throw", "to", "monkey", value] => 
                    monkey.if_true = value.parse().unwrap(),
                ["If", "false:", "throw", "to", "monkey", value] => 
                    monkey.if_false = value.parse().unwrap(),
                [..] => eprintln!("Error: pattern {line_str} not regognized."),
            }

        }
        if lines_read > 2 {
            Some(monkey)
        } else {
            None
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>) {
    for m in 0..monkeys.len() {
        let monkey = &monkeys[m];
        for i in 0..monkey.items.len() {
            monkeys[m].count += 1;
            let mut new_item = (monkeys[m].operation)(monkeys[m].items[i]); 
            new_item = new_item.div_floor(3);
            let is_divisible = new_item % monkeys[m].divisible_by == 0;
            let dest = if is_divisible {
                monkeys[m].if_true
            } else {
                monkeys[m].if_false
            };
            monkeys[dest].items.push(new_item);
        }
        monkeys[m].items.clear();
    }
}

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let mut lines = file.lines();
    let mut monkeys: Vec<Monkey> = vec![];
    loop {
        match Monkey::parse(&mut lines) {
            None => break,
            Some(monkey) => monkeys.push(monkey)
        }
    }
    for _ in 0..20 {
        round(&mut monkeys);
    }
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i} inspected items {} times.", monkey.count);
    }
    let mut counts: Vec<u32> = monkeys.iter().map(|m| m.count).collect();
    counts.sort();
    counts.reverse();
    println!("Monkey business: {}", counts[0] * counts[1]);
    
    
    
}
