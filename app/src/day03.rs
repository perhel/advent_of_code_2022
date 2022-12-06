use std::fs;
use stopwatch::Stopwatch;

#[cfg(debug_assertions)]
fn get_env() -> &'static str {
    "DEBUG"
}

#[cfg(not(debug_assertions))]
fn get_env() -> &'static str {
    "RELEASE"
}

struct Backpack {
    compartment_one: Vec<char>,
    compartment_two: Vec<char>,
    raw: Vec<char>
}

impl Backpack {
    pub fn from_str(s: &str) -> Backpack {
        let (comp_a, comp_b) = s.split_at(s.len() / 2);

        if comp_a.len() != comp_b.len() {
            panic!()
        }

        Backpack {
            compartment_one: comp_a.chars().collect(),
            compartment_two: comp_b.chars().collect(),
            raw: s.chars().collect()
        }
    }

    pub fn find_duplicate(&self) -> Option<char> {
        match self.compartment_one.iter()
            .find(|c| self.compartment_two.contains(c)) {
                Some(c) => Some(*c),
                None => None
        }
    }

    pub fn find_duplicate_in_group(group: &Vec<Backpack>) -> Option<char> {
        let start: Vec<char> = group[0].raw.iter()
        .filter(|c| group[1].raw.contains(c))
        .map(|c| *c)
        .collect();

        match start.iter()
            .find(|c| group[2].raw.contains(c)) {
                Some(c) => Some(*c),
                None => None
        }
    }
}

fn part_1(input: String) -> u32 {
    input.lines().into_iter()
        .map(|line| {
            match Backpack::from_str(line.trim()).find_duplicate() {
                Some(c) => char_to_points(c),
                None => panic!("No duplicate")
            }
        })
        .sum()
}

fn part_2(input: String) -> u32 {
    input.lines()
        .into_iter().collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| chunk.iter()
            .map(|&str| Backpack::from_str(str.trim()))
            .collect()
        )
        .map(|g| Backpack::find_duplicate_in_group(&g).expect("No common char"))
        .map(|c| char_to_points(c))
        .sum()
}

fn char_to_points(c: char) -> u32 {
    let mut num: u8 = c as u8;
    if num >= 97 {
        num -= 96
    } else {
        num -= 38
    }

    num.into()
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day03.txt").expect("Could not read file");

    println!("### Day 3 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input.clone()));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}μs total ({})--", ms.as_micros(), get_env());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let result: u32 = input.lines().into_iter()
            .map(|line| {
                match Backpack::from_str(line.trim()).find_duplicate() {
                    Some(c) => char_to_points(c),
                    None => panic!("No duplicate")
                }
            })
            .sum();

        assert_eq!(result, 157);
    }

    #[test]
    fn part_2() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let result: u32 = input.lines()
            .into_iter().collect::<Vec<&str>>()
            .chunks(3)
            .map(|chunk| chunk.iter()
                .map(|&str| Backpack::from_str(str.trim()))
                .collect()
            )
            .map(|g| Backpack::find_duplicate_in_group(&g).expect("No common char"))
            .map(|c| char_to_points(c))
            .sum();

        assert_eq!(result, 70);
    }
}