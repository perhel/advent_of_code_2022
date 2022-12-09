use std::{fs, str::CharIndices};
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

type Map = Vec<Vec<(usize, u32)>>;
type Tree = (usize, u32);

fn part_1(map: Map) -> u32 {
    let mut visible: u32 = get_outer_visible(&map);

        for row in 1..map.len() - 1 {
            for col in 1..map[row].len() - 1 {
                let tree: Tree = map[row][col];
                if tree_is_visible(&map, row, tree) {
                    visible += 1;
                }
            }
        }
    visible
}

fn part_2(map: Map) -> u32 {
    let mut score: u32 = 0;

    for row in 1..map.len() - 1 {
        for col in 1..map[row].len() - 1 {
            let tree: Tree = map[row][col];
            let new_score = get_scenic_score(&map, row, tree);
            if new_score > score {
                score = new_score;                   
            }
        }
    }

    score
}

fn to_map(input: String) -> Vec<Vec<(usize, u32)>> {
    input.lines().into_iter()
        .map(|l| l.char_indices().map(|a| (a.0, a.1.to_digit(10).expect("Should parse"))).collect_vec())
        .collect()
}

fn get_outer_visible(map: &Map) -> u32 {
    (map.len() as u32 * 2) + (map[0].len() as u32 * 2) - 4
}

fn tree_is_visible(map: &Map, row: usize, tree: Tree) -> bool {
    let left = map[row].iter()
        .filter(|t| t.0 < tree.0)
        .all(|t| t.1 < tree.1);

    let right = map[row].iter()
        .filter(|t| t.0 > tree.0)
        .all(|t| t.1 < tree.1);

    let top = map.iter()
        .enumerate()
        .filter(|r| r.0 < row)
        .all(|r| r.1[tree.0].1 < tree.1);

    let bottom = map.iter()
        .enumerate()
        .filter(|r| r.0 > row)
        .all(|r| r.1[tree.0].1 < tree.1);
    
    left || right || top || bottom
}

fn get_scenic_score(map: &Map, row: usize, tree: Tree) -> u32 {
    let mut left = 0;
    for t in map[row].iter().filter(|t| t.0 < tree.0).rev() {
        left += 1;
        if t.1 >= tree.1 {
            break;
        }
    }

    let mut right = 0;
    for t in map[row].iter().filter(|t| t.0 > tree.0) {
        right += 1;
        if t.1 >= tree.1 {
            break;
        }
    }

    let mut up = 0;
    for r in map.iter().enumerate().filter(|r| r.0 < row).rev() {
        up += 1;
        if r.1[tree.0].1 >= tree.1 {
            break;
        }
    }

    let mut down = 0;
    for r in map.iter().enumerate().filter(|r| r.0 > row) {
        down += 1;
        if r.1[tree.0].1 >= tree.1 {
            break;
        }
    }

    (left * right * up * down) as u32
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day08.txt").expect("Could not read file");
    
    let map: Vec<Vec<(usize, u32)>> = to_map(input);

    println!("### Day 8 ###");
    println!("# Part 1: {}", part_1(map.clone()));
    println!("# Part 2: {}", part_2(map));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}μs total ({})--", ms.as_micros(), get_env());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"30373
25512
65332
33549
35390"#.to_string();

        let map: Vec<Vec<(usize, u32)>> = to_map(input);

        let mut visible: u32 = get_outer_visible(&map);

        for row in 1..map.len() - 1 {
            for col in 1..map[row].len() - 1 {
                let tree: Tree = map[row][col];
                if tree_is_visible(&map, row, tree) {
                    visible += 1;
                }
            }
        }

        assert_eq!(visible, 21);
    }

    #[test]
    fn part_2() {
        let input = r#"30373
25512
65332
33549
35390"#.to_string();

        let map: Vec<Vec<(usize, u32)>> = to_map(input);

        let mut score: u32 = 0;

        for row in 1..map.len() - 1 {
            for col in 1..map[row].len() - 1 {
                let tree: Tree = map[row][col];
                let new_score = get_scenic_score(&map, row, tree);
                if new_score > score {
                    score = new_score;                   
                }
            }
        }

        assert_eq!(score, 8);
    }
}