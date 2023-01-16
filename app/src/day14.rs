use std::{fs, collections::HashSet};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    fn from_str(s: &str) -> Self {
        let parts: (i32, i32) = s.split(',').into_iter()
            .map(|p| p.parse().unwrap())
            .take(2)    
            .collect_tuple().unwrap();

        Coordinate { x: parts.0, y: parts.1 }
    }
}

struct Cave {
    occupied_coordinates: HashSet<Coordinate>,
    y_max: i32,
    floor: Option<i32>
}

impl Cave {
    fn new() -> Self {
        Cave { occupied_coordinates: HashSet::new(), y_max: 0, floor: None }
    }

    fn add_rocks_from_line(&mut self, line: &str) {
        let coordinates = line.split("->").into_iter()
            .map(|p| Coordinate::from_str(p.trim()))
            .collect_vec();

        for i in 0..coordinates.len() - 1 {
            let curr = &coordinates[i];
            let next = &coordinates[i + 1];

            if curr.x != next.x {
                for x in i32::min(curr.x, next.x)..=i32::max(curr.x, next.x) {
                    self.occupied_coordinates.insert(Coordinate { x, y: curr.y });
                }
            } else if curr.y != next.y {
                for y in i32::min(curr.y, next.y)..=i32::max(curr.y, next.y) {
                    self.occupied_coordinates.insert(Coordinate { x: curr.x, y});
                }
            }
        }
    }

    fn set_y_max(&mut self) {
        self.y_max = self.occupied_coordinates.iter()
        .max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    }

    fn set_floor(&mut self) {
        self.floor = Some(self.y_max + 2);
    }

    fn drop_sand(&mut self, sand_origin: Coordinate) -> bool {
        let mut current_pos = sand_origin.clone();

        while let Some(pos) = self.get_next_fall_position(current_pos.x, current_pos.y) {
            if self.floor.is_none() && self.is_out_of_bounds(pos.y) {
                return false
            }
            current_pos = pos;
        }

        if current_pos == sand_origin {
            return false;
        }

        self.occupied_coordinates.insert(current_pos);

        return true
    }

    fn get_next_fall_position(&self, x: i32, y: i32) -> Option<Coordinate> {
        if self.floor.is_some() && self.floor.unwrap() == y + 1 {
            None
        } else if self.occupied_coordinates.get(&Coordinate::new(x, y + 1)).is_none() {
            Some(Coordinate::new(x, y + 1))
        } else if self.occupied_coordinates.get(&Coordinate::new(x - 1, y + 1)).is_none() {
            Some(Coordinate::new(x - 1, y + 1))
        } else if self.occupied_coordinates.get(&Coordinate::new(x + 1, y + 1)).is_none() {
            Some(Coordinate::new(x + 1, y + 1))
        } else {
            None
        }
    }

    fn is_out_of_bounds(&self, y: i32) -> bool {
        y >= self.y_max
    }
}

fn part_1(input: String) -> u32 {
    let mut cave = Cave::new();
    for line in input.lines() {
        cave.add_rocks_from_line(line);
    }
    cave.set_y_max();

    let mut units_at_rest = 0;
    while cave.drop_sand(Coordinate { x: 500, y: 0 }) {
        units_at_rest += 1;
    }

    units_at_rest
}

fn part_2(input: String) -> u32 {
    let mut cave = Cave::new();
    for line in input.lines() {
        cave.add_rocks_from_line(line);
    }
    cave.set_y_max();
    cave.set_floor();

    let mut units_at_rest = 0;
    while cave.drop_sand(Coordinate { x: 500, y: 0 }) {
        units_at_rest += 1;
    }

    units_at_rest + 1
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day14.txt").expect("Could not read file");
    
    println!("### Day 14 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}ms total ({})--", ms.as_millis(), get_env());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#.to_string();

        let mut cave = Cave::new();
        for line in input.lines() {
            cave.add_rocks_from_line(line);
        }
        cave.set_y_max();

        let mut units_at_rest = 0;
        while cave.drop_sand(Coordinate { x: 500, y: 0 }) {
            units_at_rest += 1;
        }

        assert_eq!(units_at_rest, 24);
    }

    #[test]
    fn part_2() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#.to_string();

        let mut cave = Cave::new();
        for line in input.lines() {
            cave.add_rocks_from_line(line);
        }
        cave.set_y_max();
        cave.set_floor();

        let mut units_at_rest = 0;
        while cave.drop_sand(Coordinate { x: 500, y: 0 }) {
            units_at_rest += 1;
        }

        assert_eq!(units_at_rest + 1, 93);
    }
}