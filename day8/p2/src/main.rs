#![feature(iter_advance_by)]
use std::fs;

struct Tile {
    height: i8,
    score: u32,
    current_count: u32
}

fn print_tiles(map: &Vec<Vec<Tile>>) {
    for row in map {
        for col in row {
            print!("{:6}", col.score);
        }
        println!();
        println!();
        println!();
        println!();
    }
}

// UGLY AS F

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let row_len = file.lines().next().unwrap().len();
    let mut map: Vec<Vec<Tile>> = Vec::new();
    for (i, line) in file.lines().enumerate() {
        let mut counting: Vec<usize> = vec![];
        map.push(line.chars().map(|c| {
            let d: i8 = c.to_digit(10).unwrap().try_into().unwrap();
            Tile { height: d, score: 0, current_count: 0 }

        }).collect());
        for j in 0..row_len {
            let mut counting_to_del: Vec<usize> = vec![];
            for (c_idx, prev_tile_index) in counting.iter().enumerate() {
                let mut prev_tile = &mut map[i][*prev_tile_index];
                prev_tile.score += 1;
                if prev_tile.height <= map[i][j].height {
                    counting_to_del.push(c_idx);
                }
            }
            counting_to_del.sort_unstable();
            for c in counting_to_del.iter().rev() {
                counting.remove(*c);
            }
            counting.push(j);
        }
        counting = vec![];
        for j in (0..row_len).rev() {
            let mut counting_to_del: Vec<usize> = vec![];
            for (c_idx, prev_tile_index) in counting.iter().enumerate() {
                let mut prev_tile = &mut map[i][*prev_tile_index];
                prev_tile.current_count += 1;
                if prev_tile.height <= map[i][j].height {
                    counting_to_del.push(c_idx);
                }
            }
            counting_to_del.sort_unstable();
            for c in counting_to_del.iter().rev() {
                let mut prev_tile = &mut map[i][counting[*c]];
                prev_tile.score *= prev_tile.current_count;
                prev_tile.current_count = 0;
                counting.remove(*c);
            }
            counting.push(j);
        }
        for c in counting {
            let mut tile = &mut map[i][c];
            tile.score *= tile.current_count;
            tile.current_count = 0;
        }
    }
    for j in 0..row_len {
        let mut counting: Vec<usize> = vec![];
        for i in 0..map.len() {
            let mut counting_to_del: Vec<usize> = vec![];
            for (c_idx, prev_tile_index) in counting.iter().enumerate() {
                let mut prev_tile = &mut map[*prev_tile_index][j];
                prev_tile.current_count += 1;
                if prev_tile.height <= map[i][j].height {
                    counting_to_del.push(c_idx);
                }
            }
            counting_to_del.sort_unstable();
            for c in counting_to_del.iter().rev() {
                let mut prev_tile = &mut map[counting[*c]][j];
                prev_tile.score *= prev_tile.current_count;
                prev_tile.current_count = 0;
                counting.remove(*c);
            }
            counting.push(i);
        }
        for c in counting {
            let mut tile = &mut map[c][j];
            tile.score *= tile.current_count;
            tile.current_count = 0;
        }
    }
    for j in 0..row_len {
        let mut counting: Vec<usize> = vec![];
        for i in (0..map.len()).rev() {
            let mut counting_to_del: Vec<usize> = vec![];
            for (c_idx, prev_tile_index) in counting.iter().enumerate() {
                let mut prev_tile = &mut map[*prev_tile_index][j];
                prev_tile.current_count += 1;
                if prev_tile.height <= map[i][j].height {
                    counting_to_del.push(c_idx);
                }
            }
            counting_to_del.sort_unstable();
            for c in counting_to_del.iter().rev() {
                let mut prev_tile = &mut map[counting[*c]][j];
                prev_tile.score *= prev_tile.current_count;
                prev_tile.current_count = 0;
                counting.remove(*c);
            }
            counting.push(i);
        }
        for c in counting {
            let mut tile = &mut map[c][j];
            tile.score *= tile.current_count;
            tile.current_count = 0;
        }
    }
    print_tiles(&map);
    println!("{}", map.iter().map(|row| row.iter().map(|tile| tile.score).max().unwrap()).max().unwrap());
}
