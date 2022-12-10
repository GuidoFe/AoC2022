use std::{fs, process};


//Current acc value, previous acc value, current cycle
fn parse_code(val: &mut i32, line: &str) {
    match line.split(" ").collect::<Vec<&str>>()[..] {
        ["noop"] => {},
        ["addx", amount] => { 
            *val += amount.parse::<i32>().unwrap();
        },
        _ => { 
            eprint!("Command not recognized: {line}");
            process::exit(1);
        }
    };
}

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let mut lines = file.lines().flat_map(|line| if line == "noop" {
        vec![line]
    } else {
        vec!["noop", line]
    });
    let mut addr = 1;
    for _ in 0..6 {
        for col in 0..40 {
            if [addr-1, addr, addr+1].contains(&col) {
                print!("#");
            } else {
                print!(".");
            }
            parse_code(&mut addr, lines.next().unwrap());
        }
        println!();
    }
}
