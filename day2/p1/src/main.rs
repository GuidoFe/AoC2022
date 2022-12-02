use std::fs;

fn sub_to_char(c: &char, amount: &u32) -> char {
    let mut n = *c as u32 - *amount;
    if n < 65 {
        n += 3;
    }
    char::from_u32(n).unwrap()
}

fn point_value(my_char: &char) -> u32 {
    *my_char as u32 - 87
}

fn main() {
    let list = fs::read_to_string("../input").expect("Couldn't read file");
    let points: u32 = list.lines().map(|line| {
        let chars: Vec::<char> = line.chars().collect();
        let (adv, me) = ( chars[0], chars[2]);
        let mut round = 0;
        let me_val = point_value(&me);
        match sub_to_char(&adv, &me_val) {
           'B' => round += 6 + me_val,
           'C' => round += 3 + me_val,
           'A' => round += me_val,
           _ => {},
        }
        round
    }).sum(); 
    println!("Total points: {}", points);
    
    assert_eq!('C', sub_to_char(&'A', &1));
    assert_eq!(1, point_value(&'X'));
}
