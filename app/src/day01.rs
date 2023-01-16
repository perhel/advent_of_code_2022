use std::fs;
use itertools::Itertools;
use stopwatch::Stopwatch;

#[cfg(debug_assertions)]
fn get_env() -> &'static str {
    "DEBUG"
}

#[cfg(not(debug_assertions))]
fn get_env() -> &'static str {
    "RELEASE"
}

fn part_1(input: String) -> u32 {
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
    elves.into_iter().max().unwrap()
}

fn part_2(input: String) -> u32 {
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

    elves.iter().sorted_by(|a, b| b.cmp(a)).take(3).sum()
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day01.txt").expect("Could not read file");   
    
    println!("### Day 1 ###");
    println!("# Part 1: {:?}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input.clone()));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}ms total ({})--", ms.as_millis(), get_env());
}