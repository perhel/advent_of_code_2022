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

fn part_1(input: String) -> String {
    let x = input.lines().into_iter()
        .map(|l| to_decimal(l))
        .collect_vec();

    let sum: i64 = x.iter().sum();
    to_snafu(sum)
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day25.txt").expect("Could not read file");
    
    println!("### Day 25 ###");
    println!("# Part 1: {}", part_1(input));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}ms total ({})--", ms.as_millis(), get_env());
}

fn to_decimal(line: &str) -> i64 {
    let mut multiple = line.len() as u32;
    let mut sum = 0;
    for c in line.chars() {
        multiple -= 1;
        sum += match c {
            '=' => 5_i64.pow(multiple) * -2,
            '-' => 5_i64.pow(multiple) * -1,
            '0' => 5_i64.pow(multiple) * 0,
            '1' => 5_i64.pow(multiple) * 1,
            '2' => 5_i64.pow(multiple) * 2,
            _ => panic!()
        };
    }

    sum
}

fn to_snafu(n: i64) -> String {
    let powers = (0..50).into_iter()
        .rev()
        .map(|n| i128::pow(5, n))
        .collect_vec();

    let mut current = 0;
    let mut result = String::new();

    for pow in powers {
        let mut closest = i128::MAX;
        let mut pick = None;
        
        for i in -2..=2 {
            let test = i * pow + current;
            let abs_val = (test - n as i128).abs();
            if abs_val >= closest {
                continue;
            };
            closest = abs_val;
            pick = Some(i);
        }

        if let Some(p) = pick {
            current += p * pow;
            let ch = match p {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!()
            };
            result.push(ch);
        }
    }

    result.trim_start_matches('0').into()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn part_1() {
        let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#.to_string();

        let x = input.lines().into_iter()
            .map(|l| to_decimal(l))
            .collect_vec();

        let sum: i64 = x.iter().sum();
        let res = to_snafu(sum);
        
        assert_eq!(res, "2=-1=0");
    }
}