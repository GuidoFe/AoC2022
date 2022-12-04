use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn item_priority(item: &char) -> u32 {
    let n = *item as u32;
    if *item <= 'Z' {
        n - 38
    } else {
        n - 96
    }
}

fn main() {
    let list = fs::read_to_string("../input").expect("Couldn't read file");
    let res: u32 = list.lines().map(|l| l.chars().collect::<HashSet<char>>())
        .tuples().map(|(a, b, c)| {
        a.intersection(&b).map(|el| *el).collect::<HashSet<char>>().intersection(&c).next().unwrap().to_owned()
    }).map(|c| item_priority(&c)).sum();
    println!("{}", res);
    assert_eq!(1, item_priority(&'a'));
    assert_eq!(27, item_priority(&'A'));
    
}
