use std::fs;

fn part_1(input: String) -> Option<u32> {
    let mut elves: Vec<u32> = Vec::new();
    let mut cargo: Vec<u32> = Vec::new();

    for line in input.lines() {
        if line.len() != 0 {
            cargo.push(line.parse::<u32>().expect("Should parse"));
        } else {
            elves.push(cargo.iter().sum());
            cargo.clear();
        }
    }
    elves.iter().max().copied()
}

fn part_2(input: String) -> u32 {
    for i in 7..25 {
        fs::File::create(format!("inputs/2022/day0{}.txt", i)).unwrap();
        fs::File::create(format!("src/day0{}.rs", i)).unwrap();
    }

    0
}

fn main() {
    let input = fs::read_to_string("inputs/2022/day01.txt").expect("Could not read file");   
    
    println!("### Day 1 ###");
    println!("# Part 1: {:?}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input.clone()));
}