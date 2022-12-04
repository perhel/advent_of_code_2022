use std::fs;

struct CleanupPair {
    first: Vec<u32>,
    second: Vec<u32>
}

impl From<&str> for CleanupPair {
    fn from(s: &str) -> Self {
        let pair: Vec<&str> = s.split(',').collect();
        if pair.len() != 2 {
            panic!()
        }

        let mut ranges: Vec<Vec<u32>> = Vec::new();
        for &r in &pair {
            ranges.push(
                r.split('-')
                    .map(|s| s.parse::<u32>().expect("Should parse")).collect()
            );
        }

        CleanupPair {
            first: (ranges[0][0]..ranges[0][1] + 1).collect(),
            second: (ranges[1][0]..ranges[1][1] + 1).collect()
        }
    }
}

impl CleanupPair {
    fn is_full_overlap(&self) -> bool {
        self.second.iter().all(|n| self.first.contains(n)) ||
        self.first.iter().all(|n| self.second.contains(n))
    }

    fn is_overlap(&self) -> bool {
        self.second.iter().any(|n| self.first.contains(n)) ||
        self.first.iter().any(|n| self.second.contains(n))
    }
}

fn part_1(input: String) -> i32 {
    input.lines().into_iter()
        .map(|l| CleanupPair::from(l.trim()))
        .map(|cl| {
            match cl.is_full_overlap() {
                true => 1,
                false => 0
            }
        })
        .sum()
}

fn part_2(input: String) -> i32 {
    input.lines().into_iter()
        .map(|l| CleanupPair::from(l.trim()))
        .map(|cl| {
            match cl.is_overlap() {
                true => 1,
                false => 0
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("inputs/2022/day04.txt").expect("Could not read file");

    println!("### Day 4 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;

        let result: i32 = input.lines().into_iter()
            .map(|l| CleanupPair::from(l.trim()))
            .map(|cl| {
                match cl.is_full_overlap() {
                    true => 1,
                    false => 0
                }
            })
            .sum();

        assert_eq!(result, 2);
    }

    #[test]
    fn part_2() {
        let input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;

        let result: i32 = input.lines().into_iter()
            .map(|l| CleanupPair::from(l.trim()))
            .map(|cl| {
                match cl.is_overlap() {
                    true => 1,
                    false => 0
                }
            })
            .sum();

        assert_eq!(result, 4);
    }
}