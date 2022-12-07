#![feature(iter_advance_by)]
use std::{fs, collections::HashMap};
//use itertools::Itertools;

struct Folder {
    parent: Option<usize>,
    children: HashMap<String, usize>,
    size: u32
}

fn sum_node_size_to_parents(size: &u32, start: &usize, nodes:&mut Vec<Folder>) {
    let mut parent_index = Some(*start);
    while parent_index.is_some() {
        let current_parent = &mut nodes[parent_index.unwrap()];
        current_parent.size += size;
        parent_index = current_parent.parent
    }

}

fn print_nodes(name: &str, tabs: u32, index: &usize, nodes: &Vec<Folder>) {
    for _ in 0..tabs {
        print!(" ");
    }
    println!("{}: {}", name, nodes[*index].size);
    for (child_name, child_index) in &nodes[*index].children {
        print_nodes(child_name, tabs + 4, child_index, nodes);
    }
}

fn main() {
    let file = fs::read_to_string("../input").expect("Couldn't read file");
    let mut lines = file.lines();
    lines.next();
    let mut nodes: Vec<Folder> = vec![];    
    nodes.push(Folder { parent: Option::None, children: HashMap::new(), size: 0 });
    let mut cursor: usize = 0;
    for line in lines {
        //println!("{}", line);
        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["$", "cd", dir] => {
                match dir {
                    "/" => {
                        cursor = 0;
                    },
                    ".." => {
                        cursor = nodes[cursor].parent.unwrap();
                    },
                    _ => {
                        cursor = *nodes[cursor].children.get(dir).unwrap();
                    }
                }
            },
            ["$", "ls"] => {},
            ["dir", name] => {
                let current_len = nodes.len();
                nodes[cursor].children.insert(name.to_string(), current_len);
                nodes.push(Folder { parent: Some(cursor), children: HashMap::new(), size: 0 });
            },
            [size, _] => {
                sum_node_size_to_parents(&size.parse::<u32>().unwrap(), &cursor, &mut nodes)
            },
            [..] => {}
        }
    }
    let sum: u32 = nodes.iter().map(|f| f.size).filter(|s| s <= &100000).sum();
    println!("{}", sum);
    //print_nodes("/", 0, &0, &nodes);
}
