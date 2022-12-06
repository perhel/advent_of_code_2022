use std::{fs, collections::VecDeque};

use itertools::Itertools;

fn part_1(input: String) -> u32 {
    get_position_after_n_unique(input, 4)
}

fn part_2(input: String) -> u32 {
    get_position_after_n_unique(input, 14)
}

fn get_position_after_n_unique(input: String, n: usize) -> u32 {
    let chars = input.char_indices().into_iter();

    let mut buff: VecDeque<char> = VecDeque::new();

    for c in chars {
        buff.push_back(c.1);

        if buff.len() > n {
            buff.pop_front();
        }
        
        if buff.len() == n && buff.iter().all_unique() {
            return c.0 as u32 + 1
        }
    }

    panic!("No marker found")
}

fn main() {
    let input = fs::read_to_string("inputs/2022/day06.txt").expect("Could not read file");
    
    println!("### Day 6 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#.to_string();

        let result = get_position_after_n_unique(input, 4);

        assert_eq!(result, 5);
    }

    #[test]
    fn part_2() {
        let input = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#.to_string();

        let result = get_position_after_n_unique(input, 14);

        assert_eq!(result, 19);
    }
}