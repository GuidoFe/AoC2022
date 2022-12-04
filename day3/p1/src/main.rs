use std::fs;
use std::collections::HashSet;

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
    let sum: u32 = list.lines().map(|line| {
        let mut set_l = HashSet::<char>::new();
        let mut set_r = HashSet::<char>::new();
        let half_size = line.len() / 2;
        let (l, r) = line.split_at(half_size);
        let mut found = '?';
        for (a, b) in l.chars().zip(r.chars()) {
            set_l.insert(a);
            set_r.insert(b);
            if set_r.contains(&a) {
                found = a;
                break;
            }
            if set_l.contains(&b) {
                found = b;
                break;
            }
        }
        item_priority(&found)
    }).sum();
    assert_eq!(1, item_priority(&'a'));
    assert_eq!(27, item_priority(&'A'));
    println!("{}", sum);
    
}
