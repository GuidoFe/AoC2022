#![feature(iter_advance_by)]
use std::{fs, collections::HashSet};
//use itertools::Itertools;

// Only iterators, because why not
fn main() {
    let chars: Vec<char> = fs::read_to_string("../input").expect("Couldn't read file").chars().collect();
    let (n, _) = chars.windows(4).enumerate().find(|(_, slice)| {
        let set: HashSet<char> = slice.iter().copied().collect();
        return set.len() == 4;
    }).unwrap();
    println!("{}", n + 4);


}
