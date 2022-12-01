use std::{fs, str::FromStr};

fn main() {
    let input_string = fs::read_to_string("src/input.txt").expect("File not found");

    let input: Vec<Vec<u32>> = input_string
        .split('|')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| {
            s.split(';')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|s| s.len() > 0)
            .map(|s| {
                FromStr::from_str(s.trim()).unwrap()
            })
            .collect::<Vec<u32>>()
        })
        .collect();

    print!("Part one: ");
    part_one(&input);
    print!("Part two: ");
    part_two(&input);
}

fn part_one(input: &Vec<Vec<u32>>) {
    let elfes: Vec<u32> = input.iter().map(|arr| arr.iter().sum()).collect();
    match elfes.iter().max() {
        Some(max) => println!("{:?}", max),
        None => println!("U f:ed up...")
    }
}

fn part_two(input: &Vec<Vec<u32>>) {
    let mut elfes: Vec<u32> = input.iter()
        .map(|arr| arr.iter().sum())
        .collect();

    elfes.sort_by(|a, b| b.cmp(a));

    match elfes.chunks(3)
    .nth(0) {
        Some(arr) => println!("{}", arr.iter().sum::<u32>()),
        None => println!("Somethings wrong...")
    };
}
