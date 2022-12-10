use std::{fs, process};


//Current acc value, previous acc value, current cycle
fn parse_code(initial_state: (i32, i32, u32), line: &str) -> (i32, i32, u32) {
    let (mut register, _, mut cycle) = initial_state;
    let prev = register;
    match line.split(" ").collect::<Vec<&str>>()[..] {
        ["noop"] => { cycle += 1 },
        ["addx", amount] => { 
            cycle += 2;
            register += amount.parse::<i32>().unwrap();
        },
        _ => { 
            eprint!("Command not recognized: {line}");
            process::exit(1);
        }
    };
    (register, prev, cycle)
}

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let mut lines = file.lines();
    let mut status = parse_code((1, 0, 1), lines.next().unwrap());
    let mut total_strength = 0;
    'stoploop: for stop in [20, 60, 100, 140, 180, 220] {
        loop {
            let (reg, prev, cycles) = parse_code(status, lines.next().unwrap());
            status = (reg, prev, cycles);
            if cycles == stop {
                let strength = stop as i32 * reg;
                total_strength += strength;
                println!("{stop} -> X: {reg} STR: {strength}");
                continue 'stoploop;
            } else if cycles > stop {
                let strength = stop as i32 * prev;
                total_strength += strength;
                println!("{cycles} ({stop}) -> X: {prev}, STR: {strength}");
                continue 'stoploop;
            }
        }
    }
    println!("Total strength: {total_strength}");
}
