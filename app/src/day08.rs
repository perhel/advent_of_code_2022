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

fn part_1(input: String) -> u32 {
    0
}

fn part_2(input: String) -> u32 {
    0
}

fn main() {
    let mut sw = Stopwatch::start_new();
    todo!();let input = fs::read_to_string("inputs/2022/day0X.txt").expect("Could not read file");
    
    todo!();println!("### Day X ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}μs total ({})--", ms.as_micros(), get_env());
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