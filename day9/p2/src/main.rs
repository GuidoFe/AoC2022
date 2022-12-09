#![feature(iter_advance_by)]
use std::{fs, thread, time::Duration};
use std::collections::HashSet;
extern crate termsize;
//use itertools::Itertools;

struct Screen {
    w: usize,
    h: usize,
    origin: Coord,
    frame: Vec<Vec<char>>
}

impl Screen {
    fn render_rope(&mut self, rope: &Vec<Coord>) {
        for i in 0..rope.len() {
            match self.global_to_frame(&rope[i]) {
                Some(local) => { self.frame[local.y as usize][local.x as usize] = '█'},
                None => {}
            }
        }
    }

    fn render_background(&mut self) {
        for x in 0..self.w {
            for y in 0..self.h {
                let global = self.frame_to_global(&Coord{x: x as i32, y: y as i32});
                let c = if global.x == 0 && global.y == 0 {
                    '╬'
                } else if global.x == 0 {
                    '║'
                } else if global.y == 0 {
                    '═'
                } else if global.x % 5 == 0 && global.y % 5 == 0 {
                    '·'
                } else {
                    ' '
                };
                self.frame[y][x] = c;
            }
        }
    }

    fn render_and_draw_everything(&mut self, rope: &Vec<Coord>, string: &str) {
        Screen::clear_screen();
        let mut new_origin_x = self.origin.x;
        if rope[0].x < self.origin.x {
            new_origin_x = rope[0].x
        } else if rope[0].x >= self.origin.x + self.w as i32 {
            new_origin_x = rope[0].x - self.w as i32;
        };
        let mut new_origin_y = self.origin.y;
        if rope[0].y < self.origin.y {
            new_origin_y = rope[0].y
        } else if rope[0].y >= self.origin.y + self.h as i32 {
            new_origin_y = rope[0].y - self.h as i32;
        };
        self.origin = Coord { x: new_origin_x, y: new_origin_y };
        self.render_background();
        self.render_rope(rope);
        self.render_caption(string);
        self.draw_frame();
    }

    fn frame_to_global(&self, frame: &Coord) -> Coord {
        return Coord { x: frame.x + self.origin.x, y: frame.y + self.origin.y };
    }

    fn global_to_frame(&self, global: &Coord) -> Option<Coord> {
        let frame = Coord { x: global.x - self.origin.x, y: global.y - self.origin.y };
        if self.is_frame_point_on_screen(&frame) {
            Some(frame)
        } else {
            None
        }
    }

    fn is_frame_point_on_screen(&self, frame: &Coord) -> bool {
        return frame.x >= 0 && frame.x < self.w.try_into().unwrap() && frame.y >= 0 && frame.y < self.h.try_into().unwrap();
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn draw_frame(&self) {
        for y in 0..self.h {
            let line: String = self.frame[y].iter().collect();
            println!("{line}");
        }
    }
    fn render_caption(&mut self, string: &str) {
        if string.len() > self.w || string.len() == 0 {
            return;
        }
        let start_index = self.w - string.len();
        let chars: Vec<char> = string.chars().collect();
        for i in 0..chars.len() {
            self.frame[1][start_index + i] = chars[i];
        }
    }
}

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
    let screen_size = termsize::get().unwrap();
    let width = screen_size.cols;
    let height = screen_size.rows;
    let mut screen = Screen {
        w: width as usize,
        h: height as usize,
        origin: Coord::zero(),
        frame: vec![]
    };
    for _ in 0..height {
        screen.frame.push(vec![' '; width as usize]);
    }
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let mut rope = vec![Coord::zero();10];
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("0,0".to_string());
    for line in file.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        let (dir, n) = (words[0], words[1].parse::<u32>().unwrap());
        for k in 0..n {
            let caption = if n == 1 {
                format!("{dir} 1")
            } else {
                format!("{} {}/{}", dir, k + 1, n)
            };
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
            screen.render_and_draw_everything(&rope, &caption);
            thread::sleep(Duration::from_millis(50));
        }
    }
    println!("{}", visited.len());
}
