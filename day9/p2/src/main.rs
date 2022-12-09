#![feature(iter_advance_by)]
use std::fs;
use std::collections::HashSet;
//use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn zero() -> Coord {
        Coord{ x: 0, y: 0}
    }
}

fn move_head(head: &mut Coord, dir: &str) {
    match dir {
        "L" => { head.x -= 1; },
        "U" => { head.y -= 1; },
        "R" => { head.x += 1; },
        "D" => { head.y += 1; },
        _ => { eprintln!("Error: Movement {dir} not recognized") }
    }
}


fn update_segment(segment: usize, rope: &mut Vec<Coord>) -> bool {
    let head = segment - 1;
    if (rope[segment].x - rope[head].x).abs() <= 1 && (rope[segment].y - rope[head].y).abs() <= 1 {
        return false;
    }
    let delta = Coord{
        x: (rope[head].x - rope[segment].x).signum(),
        y: (rope[head].y - rope[segment].y).signum()
    };
    rope[segment].x += delta.x;
    rope[segment].y += delta.y;
    return true;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let mut rope = vec![Coord::zero();10];
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("0,0".to_string());
    for line in file.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        let (dir, n) = (words[0], words[1].parse::<u32>().unwrap());
        for _ in 0..n {
            move_head(&mut rope[0], &dir);
            for i in 1..10 {
                let has_moved = update_segment(i, &mut rope);
                if !has_moved {
                    break;
                } else {
                    if i == 9 {
                        let key = rope[9].x.to_string() + "," + &rope[9].y.to_string();
                        visited.insert(key);
                    }
                }
            }
        }
    }
    println!("{}", visited.len());
}
