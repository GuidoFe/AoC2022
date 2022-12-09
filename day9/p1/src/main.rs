#![feature(iter_advance_by)]
use std::fs;
use std::collections::HashSet;
//use itertools::Itertools;

#[derive(Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn zero() -> Coord {
        Coord{ x: 0, y: 0}
    }
}

#[derive(Eq, PartialEq)]
struct Rope {
    head: Coord,
    tail: Coord
}

impl Rope {
    fn move_head(&mut self, dir: &str) -> bool {
        match dir {
            "L" => { self.head.x -= 1; },
            "U" => { self.head.y -= 1; },
            "R" => { self.head.x += 1; },
            "D" => { self.head.y += 1; },
            _ => { eprintln!("Error: Movement {dir} not recognized") }
        }
        self.update_tail()
    }


    fn update_tail(&mut self) -> bool {
        if (self.tail.x - self.head.x).abs() <= 1 && (self.tail.y - self.head.y).abs() <= 1 {
            return false;
        }
        let delta = Coord{
            x: (self.head.x - self.tail.x).signum(),
            y: (self.head.y - self.tail.y).signum()
        };
        self.tail.x += delta.x;
        self.tail.y += delta.y;
        return true;
    }
}

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let mut rope = Rope { head: Coord::zero(), tail: Coord::zero()};
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("0,0".to_string());
    for line in file.lines() {
        println!("{line}");
        let words: Vec<&str> = line.split(" ").collect();
        let (dir, n) = (words[0], words[1].parse::<u32>().unwrap());
        for _ in 0..n {
            let has_moved = rope.move_head(dir);
            println!("Head: {}, {}\nTail: {}, {}", rope.head.x, rope.head.y, rope.tail.x, rope.tail.y);
            if has_moved {
                let key = rope.tail.x.to_string() + "," + &rope.tail.y.to_string();
                println!("Tail moved to {key}");
                visited.insert(key);
            }
        }
        
        println!("===================================");
    }
    println!("{}", visited.len());
}
