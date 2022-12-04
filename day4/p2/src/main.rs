use std::fs;


fn main() {
    let list = fs::read_to_string("../input").expect("Couldn't read file");
    let sum: u32 = list.lines().map(|line| {
        let pairs: Vec<Vec<u32>> = line.split(",").map(|s| 
            s.split('-').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        ).collect();
        if pairs[0][1] < pairs[1][0] || pairs[0][0] > pairs[1][1] {
            0
        } else {
            1
        }
    }).sum();
    println!("{}", sum);
}
