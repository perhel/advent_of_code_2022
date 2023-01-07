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

struct FilePart {
    original: usize,
    current: usize,
    value: i64
}

impl FilePart {
    fn new(original: usize, current: usize, value: i64) -> Self {
        FilePart { original, current, value }
    }
}

fn part_1(input: String) -> i64 {
    let mut file: Vec<FilePart> = input.lines().into_iter()
        .enumerate()
        .map(|(i, l)| FilePart::new(i, i, l.parse::<i64>().expect("Should parse")))
        .collect_vec();

    for i in 0..file.len() {
        mix(&mut file, i);
    }

    let val_0_index = file.iter().find(|fp| fp.value == 0).unwrap().current;

    (file[(val_0_index + 1000) % file.len()].value) +
        (file[(val_0_index + 2000) % file.len()].value) +
        (file[(val_0_index + 3000) % file.len()].value)
}

fn part_2(input: String) -> i64 {
    let mut file: Vec<FilePart> = input.lines().into_iter()
        .enumerate()
        .map(|(i, l)| FilePart::new(i, i, l.parse::<i64>().expect("Should parse") * 811589153))
        .collect_vec();

    for _ in 0..10 {
        for i in 0..file.len() {
            mix(&mut file, i);
        }
    }

    let val_0_index = file.iter().find(|fp| fp.value == 0).unwrap().current;

    (file[(val_0_index + 1000) % file.len()].value) +
        (file[(val_0_index + 2000) % file.len()].value) +
        (file[(val_0_index + 3000) % file.len()].value)
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day20.txt").expect("Could not read file");
    
    println!("### Day 20 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}ms total ({})--", ms.as_millis(), get_env());
}

fn mix(file: &mut Vec<FilePart>, i: usize) {
    let len = file.len();
    let item = file.remove(file.iter().find(|f| f.original == i).unwrap().current);

    let mut _insert_at = (item.current as i64 + item.value) % (len as i64 - 1);
    if _insert_at < 0 {
        _insert_at += len as i64 - 1;
    }
    file.insert(_insert_at as usize, item);

    for i in 0..file.len() {
        file[i].current = i;
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn part_1() {
        let input = r#"1
2
-3
3
-2
0
4"#.to_string();

        let mut file: Vec<FilePart> = input.lines().into_iter()
            .enumerate()
            .map(|(i, l)| FilePart::new(i, i, l.parse::<i64>().expect("Should parse")))
            .collect_vec();

        for i in 0..file.len() {
            mix(&mut file, i);
        }

        let val_0_index = file.iter().find(|fp| fp.value == 0).unwrap().current;

        let result = (file[(val_0_index + 1000) % file.len()].value) +
            (file[(val_0_index + 2000) % file.len()].value) +
            (file[(val_0_index + 3000) % file.len()].value);

        assert_eq!(result, 3);
    }

    #[test]
    fn part_2() {
        let input = r#"1
2
-3
3
-2
0
4"#.to_string();

        let mut file: Vec<FilePart> = input.lines().into_iter()
            .enumerate()
            .map(|(i, l)| FilePart::new(i, i, l.parse::<i64>().expect("Should parse") * 811589153))
            .collect_vec();

        for _ in 0..10 {
            for i in 0..file.len() {
                mix(&mut file, i);
            }
        }

        let val_0_index = file.iter().find(|fp| fp.value == 0).unwrap().current;

        let result = (file[(val_0_index + 1000) % file.len()].value) +
            (file[(val_0_index + 2000) % file.len()].value) +
            (file[(val_0_index + 3000) % file.len()].value);

        assert_eq!(result, 1623178306);
    }
}