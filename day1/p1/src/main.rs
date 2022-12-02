use std::fs;

fn parse_sum(local_sum: &i32, max_sum: &mut i32) {
    if local_sum > max_sum {
        *max_sum = *local_sum;
    }
}

fn main() {
    let list = fs::read_to_string("../input").expect("Couldn't read file");
    let mut max_sum = 0;
    let mut local_sum = 0;
    for line in list.lines() {
        if line.is_empty() { 
            parse_sum(&local_sum, &mut max_sum);
            local_sum = 0;
            continue;
        }
        local_sum += line.parse::<i32>().expect("Couldn't parse {line} as num");

    }
    parse_sum(&local_sum, &mut max_sum);
    println!("{}", max_sum);
    
}
