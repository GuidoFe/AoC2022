#![feature(iter_advance_by)]
use std::fs;
use colored::Colorize;
//use itertools::Itertools;


fn print_grid(grid: &Vec<Vec<(i8, bool)>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let el = grid[i][j];
            if el.1 {
                print!("{}", format!("{}", el.0).bright_green()); 
            } else {
                print!("{}", format!("{}", el.0).green()); 
            }
        }
        println!();
    };
}

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let row_len = file.lines().next().unwrap().len();
    let mut map: Vec<Vec<(i8, bool)>> = Vec::new();
    for (i, line) in file.lines().enumerate() {
        let mut max: i8 = 0;
        let mut last: i8 = -1;
        map.push(line.chars().map(|c| {
            let d: i8 = c.to_digit(10).unwrap().try_into().unwrap();
            if d > max {
                max = d;
            };
            let is_visible = if d > last {
                last = d;
                true
            } else {
                false
            };
            (d, is_visible)
        }).collect());
        last = -1;
        for j in (0..row_len).rev() {
            let (height, _) = map[i][j];
            if height > last {
                last = height;
                map[i][j].1 = true;
            }
            if height == max {
                break;
            }
        }
    }
    for j in 0..map[0].len() {
        let mut max: i8 = 0;
        let mut last: i8 = -1;
        for i in 0..map.len() {
            let mut el = &mut map[i][j];
            if el.0 > max {
                max = el.0;
            }
            if el.0 > last {
                last = el.0;
                el.1 = true;
            }
            if el.0 == 9 {
                break;
            }
        }
        last = -1;
        for i in (0..map.len()).rev() {
            let mut el = &mut map[i][j];
            if el.0 > last {
                last = el.0;
                el.1 = true;
            }
            if el.0 == max {
                break;
            }
        }
    }
    let sum: i32 = map.iter().map(|line| line.iter().filter(|c| c.1).count() as i32).sum();
    print_grid(&map);
    println!("{}", sum);
}
