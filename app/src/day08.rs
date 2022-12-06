use std::fs;

fn part_1(input: String) -> u32 {
    0
}

fn part_2(input: String) -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("inputs/2022/day0X.txt").expect("Could not read file");
    
    println!("### Day 6 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"1bc123"#.to_string();
    }

    #[test]
    fn part_2() {
        let input = r#"abc123"#.to_string();
    }
}