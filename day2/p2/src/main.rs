use std::fs;

fn add_to_char(c: &char, amount: &u32) -> char {
    let mut n = *c as u32 + *amount;
    if n > 67 {
        n -= 3;
    }
    char::from_u32(n).unwrap()
}

fn point_value(my_char: &char) -> u32 {
    *my_char as u32 - 87
}

fn adv_letter_value(letter: &char) -> u32 {
    match letter {
        'C' => 1,
        'A' => 2,
        'B' => 3,
        _ => 0,
    }
}

fn main() {
    let list = fs::read_to_string("../input").expect("Couldn't read file");
    let points: u32 = list.lines().map(|line| {
        let chars: Vec::<char> = line.chars().collect();
        let (adv, me) = ( chars[0], chars[2]);
        let mut round: u32 = 0;
        let me_val = point_value(&me);
        round += adv_letter_value(&add_to_char(&adv, &me_val)) + (me_val - 1) * 3;
        round
    }).sum(); 
    println!("Total points: {}", points);
    assert_eq!('B', add_to_char(&'A', &1));
    assert_eq!('A', add_to_char(&'C', &1));
}
