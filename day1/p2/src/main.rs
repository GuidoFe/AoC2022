use std::fs;

fn check_sum(local_sum: &i32, rank: &mut [i32]) {
    for i in 0..3 {
        if *local_sum > rank[i] {
            for j in ((i+1)..3).rev() {
                rank[j] = rank[j-1];
            }
            rank[i] = *local_sum;
            break;
        }
    }
}

fn main() {
    let list = fs::read_to_string("../input").expect("Couldn't read file");
    let mut local_sum = 0;
    let mut rank = [0; 3];
    for line in list.lines() {
        if line.is_empty() { 
            check_sum(&local_sum, &mut rank);
            local_sum = 0;
            continue;
        }
        local_sum += line.parse::<i32>().expect("Couldn't parse {line} as num");

    }
    check_sum(&local_sum, &mut rank);
    let total_sum: i32 = rank.iter().sum();
    println!("{:?}", rank);
    println!("{}", total_sum);
    
}
