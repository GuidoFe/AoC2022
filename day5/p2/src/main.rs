#![feature(iter_advance_by)]
use std::{fs, ops::Deref};
use itertools::Itertools;

// Only iterators, because why not
fn main() {
    let list = fs::read_to_string("../input").expect("Couldn't read file");
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines = list.lines().peekable();
    let n_columns = lines.peek().unwrap().deref().len() / 3;
    for  _ in 0..n_columns {
       stacks.push(Vec::new());
    }
    loop {
        let line = lines.next().unwrap();
        let mut chars =  line.chars().peekable();
        chars.next();
        let label = chars.peek().unwrap();
        if *label == '1' {
            break;        
        }
        for (i, c) in chars.step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    for i in 0..n_columns {
        stacks[i].reverse();
    }
    lines.next();
    for line in lines {
        let words = line.split(" ").collect_vec();
        let (quantity, source, dest) = (
            words[1].parse::<usize>().unwrap(),
            words[3].parse::<usize>().unwrap() - 1,
            words[5].parse::<usize>().unwrap() - 1,
        );
        let len = stacks[source].len();
        let mut slice: Vec<char> = stacks[source].drain(len - quantity..).collect();
        stacks[dest].append(&mut slice);
    }
    for i in 0..n_columns {
        print!("{}", stacks[i].last().unwrap_or(&' '));
    }
}
