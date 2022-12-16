use std::{fs, collections::{VecDeque, HashSet, HashMap}};
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
struct Cell {
    x: i32,
    y: i32,
    z: u8,
    value: char
}

impl Cell {
    fn new(x: i32, y: i32, z: u8, value: char) -> Cell {
        Cell { x, y, z, value }
    }

    fn is_reachable(&self, from: &Cell) -> bool {
        self.z <= from.z + 1
    }
}

struct Map {
    cells: Vec<Vec<Cell>>
}

impl Map {
    fn new(input: String) -> Map {
        let mut cells: Vec<Vec<Cell>> = vec![vec![]; input.lines().last().unwrap().len()];

        input.lines().enumerate()
            .for_each(|(i, l)| {
                l.char_indices().for_each(|(j, c)| {
                    let z = match c {
                        'S' => 'a' as u8 - 96,
                        'E' => 'z' as u8 - 96,
                        _ => c as u8 - 96
                    };
                    cells[j].push(Cell::new(j as i32, i as i32, z, c));
                })
            });

        Map { cells }
    }

    fn get_neighbours(&self, position: Cell) -> Vec<Cell> {
        let mut neighbours: Vec<Cell> = vec![];
        
        if position.x > 0 {
            neighbours.push(self.cells[position.x as usize - 1][position.y as usize].clone());
        }
        if position.x < self.cells.len() as i32 - 1 {
            neighbours.push(self.cells[position.x as usize + 1][position.y as usize].clone());
        }
        if position.y > 0 {
            neighbours.push(self.cells[position.x as usize][position.y as usize - 1].clone());
        }
        if position.y < self.cells[0].len() as i32 - 1 {
            neighbours.push(self.cells[position.x as usize][position.y as usize + 1].clone());
        }

        neighbours
    }

    fn find_path(&self, start: Cell, goal: Cell) -> Option<Vec<Cell>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut prev = HashMap::new();
    
        queue.push_back(start);
        visited.insert(start);
    
        while let Some(node) = queue.pop_front() {
            if node == goal {
                let mut path = Vec::new();
                let mut curr = node;

                while let Some(prev) = prev.get(&curr) {
                    path.push(*prev);
                    curr = *prev;
                }

                path.reverse();
                return Some(path);
            }
    
            for neighbor in self.get_neighbours(node).iter() {
                if neighbor.is_reachable(&node) && !visited.contains(neighbor) {
                    visited.insert(*neighbor);
                    prev.insert(*neighbor, node);
                    queue.push_back(*neighbor);
                }
            }
        }

        None
    }

    fn display(&self) {
        for i in 0..self.cells[0].len() {
            let s = self.cells.iter().flatten().filter(|c| c.y as usize == i).map(|c| c.value).join("");
            println!("{}", s);
        }
        println!();
    }

    fn display_trail(&self, trail: &Vec<Cell>) {
        for i in 0..self.cells[0].len() {
            let s = self.cells.iter().flatten()
                .filter(|c| c.y as usize == i)
                .map(|c| {
                    let mut rep = '.';
                    if trail.contains(c) {
                        let n_i = trail.iter().position(|c_t| c_t == c).unwrap();
                        if let Some((_, c_n)) = trail.iter().enumerate().find(|&(p_i, _)| p_i == n_i + 1) {
                            if c_n.x > c.x {
                                rep = '>';
                            } else if c_n.x < c.x {
                                rep = '<';
                            } else if c_n.y > c.y {
                                rep = 'v';
                            } else if c_n.y < c.y {
                                rep = '^';
                            }
                        } else {
                            rep = 'E';
                        }
                    }
                    rep
                })
                .join("");
            println!("{}", s);
        }
        println!();
    }
}

fn part_1(input: String) -> u32 {
    let map = Map::new(input);
    map.display();
    println!();

    let start = map.cells.iter().flatten()
        .find(|c| c.value == 'S').expect("Should be a start").clone();
    let target = map.cells.iter().flatten()
        .find(|c| c.value == 'E').expect("Should be a target").clone();

    if let Some(path) = map.find_path(start, target) {
        map.display_trail(&path);
        return path.len() as u32;
    }

    panic!("No path found");
}

fn part_2(input: String) -> u32 {
    let map = Map::new(input);
    // map.display();

    let starts = map.cells.iter().flatten()
    .filter(|c| c.value == 'a').cloned().collect_vec();
    let target = map.cells.iter().flatten()
    .find(|c| c.value == 'E').expect("Should be a target").clone();

    let mut paths: Vec<Vec<Cell>> = vec![];
    for i in 0..starts.len() {
        if let Some(path) = map.find_path(starts[i], target) {
            paths.push(path);
        }
    }

    if let Some(path_len) = paths.iter().map(|p| p.len()).min() {
        return path_len as u32;
    }

    panic!("No paths found");
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day12.txt").expect("Could not read file");
    
    println!("### Day 12 ###");
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
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#.to_string();

        let map = Map::new(input);
        map.display();

        
        let start = map.cells.iter().flatten()
        .find(|c| c.value == 'S').expect("Should be a start").clone();
        let target = map.cells.iter().flatten()
        .find(|c| c.value == 'E').expect("Should be a target").clone();
        
        let path = map.find_path(start, target).expect("Should be a path");

        assert_eq!(path.len(), 31);
    }

    #[test]
    fn part_2() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#.to_string();

        let map = Map::new(input);
        map.display();

        let starts = map.cells.iter().flatten()
        .filter(|c| c.value == 'a').cloned().collect_vec();
        let target = map.cells.iter().flatten()
        .find(|c| c.value == 'E').expect("Should be a target").clone();

        let mut paths: Vec<Vec<Cell>> = vec![];
        for i in 0..starts.len() {
            if let Some(path) = map.find_path(starts[i], target) {
                paths.push(path);
            }
        }

        let result = paths.iter().map(|p| p.len()).min().expect("Should be a path");

        assert_eq!(result, 29);
    }
}