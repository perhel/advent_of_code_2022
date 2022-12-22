use std::{fs, collections::{VecDeque, HashSet}};
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

#[derive(Debug, Clone)]
struct HeadPos {
    x: u32,
    y: u32,
    tail: VecDeque<TailPos>
}

#[derive(Debug, Clone, Copy)]
struct TailPos {
    x: u32,
    y: u32
}

impl TailPos {
    fn new(x: u32, y: u32) -> TailPos {
        TailPos { x, y }
    }
}

struct RopeBridge {
    matrix: Vec<Vec<Position>>,
    head_pos: HeadPos,
    rope_length: u32
}

impl RopeBridge {
    fn new(rope_length: u32) -> RopeBridge {
        let mut rb = RopeBridge {
            matrix: vec![],
            head_pos: HeadPos { x: 100, y: 100, tail: VecDeque::new() },
            rope_length
        };
        rb.head_pos.tail.push_back(TailPos { x: 100, y: 100 });
        rb
    }

    fn fill(mut self, moves: &Vec<Move>) -> Self {
        let mut x = 1000;
        let mut y = 1000;
        let mut x_max = 0;
        let mut y_max = 0;
        for mv in moves.iter() {
            match mv.direction {
                Direction::Up => {
                    y += mv.steps;
                    if y > y_max {
                        y_max = y;
                    }
                },
                Direction::Down => y -= mv.steps,
                Direction::Right => {
                    x += mv.steps;
                    if x > x_max {
                        x_max = x;
                    }
                },
                Direction::Left => x -= mv.steps
            };
        };

        for i in 0..x_max {
            self.matrix.push(vec![Position::new(i, 100)]);
            for j in 0..y_max {
                self.matrix[i as usize].push(Position::new(i, j));
            }
        };

        self.matrix[100][100].tail_visited = true;

        self
    }

    fn make_move(&mut self, mv: &Move) {
        for _ in 0..mv.steps {
            self.move_head(&mv.direction);
            self.mark_tail();
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let before = self.head_pos.clone();

        match direction {
                Direction::Up => self.head_pos.y += 1,
                Direction::Down => self.head_pos.y -= 1,
                Direction::Right => self.head_pos.x += 1,
                Direction::Left => self.head_pos.x -= 1
            };

        if self.tail_required() {
            self.head_pos.tail.push_front(TailPos::new(before.x, before.y))
        }
    }

    fn tail_required(&self) -> bool {
        let last_tail_x = self.head_pos.tail.back().unwrap().x;
        let last_tail_y = self.head_pos.tail.back().unwrap().y;
        let x_distance = u32::abs_diff(self.head_pos.x, last_tail_x);
        let y_distance = u32::abs_diff(self.head_pos.y, last_tail_y);

        x_distance >= 2 ||
        y_distance >= 2 ||
        (x_distance + y_distance > 2)
    }

    fn mark_tail(&mut self) {
        if self.head_pos.tail.len() >= self.rope_length as usize {
            self.head_pos.tail.pop_back();
            self.matrix[self.head_pos.tail.back().unwrap().x as usize][self.head_pos.tail.back().unwrap().y as usize].tail_visited = true;
        }
    }
}

struct Position {
    _x: u32,
    _y: u32,
    tail_visited: bool
}

impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { _x: x, _y: y, tail_visited: false }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Move {
    direction: Direction,
    steps: u32
}

impl Move {
    fn from_str(s: &str) -> Move {
        let parts: (&str, &str) = s.split(char::is_whitespace).collect_tuple().expect("Invalid input line");
        match parts.0 {
            "U" => Move { direction: Direction::Up, steps: parts.1.parse().expect("Should parse") },
            "D" => Move { direction: Direction::Down, steps: parts.1.parse().expect("Should parse") },
            "L" => Move { direction: Direction::Left, steps: parts.1.parse().expect("Should parse") },
            "R" => Move { direction: Direction::Right, steps: parts.1.parse().expect("Should parse") },
            _ => panic!("Invalid input")
        }
    }
}

fn part_1(input: String) -> u32 {
    let moves: Vec<Move> = input.lines().into_iter().map(|l| Move::from_str(l)).collect();
    let mut rp: RopeBridge = RopeBridge::new(2).fill(&moves);

    for mv in moves {
        rp.make_move(&mv);
    }

    rp.matrix.iter().flatten()
        .filter(|p| p.tail_visited)
        .count() as u32
}

fn part_2(input: String) -> i32 {
    let moves: Vec<Move> = input.lines().into_iter().map(|l| Move::from_str(l)).collect();
    let mut trails: Vec<HashSet<(i32, i32)>> = vec![];
    let mut rope: Vec<(i32, i32)> = vec![];
    for i in 0..10 as usize {
        trails.push(HashSet::new());
        rope.push((0, 0));
        trails[i].insert(rope[i]);
    }

    for mv in moves {
        for _ in 0..mv.steps {
            match mv.direction {
                Direction::Up => rope[0].0 += 1,
                Direction::Down => rope[0].0 -= 1,
                Direction::Right => rope[0].1 += 1,
                Direction::Left => rope[0].1 -= 1
            };
            for j in 0..9 {
                let x_distance = i32::abs_diff(rope[j].0, rope[j + 1].0);
                let y_distance = i32::abs_diff(rope[j].1, rope[j + 1].1);
                if x_distance <= 1 && y_distance <= 1 {
                    continue;
                }
                rope[j + 1].0 += i32::clamp(rope[j].0 - rope[j + 1].0, -1, 1);
                rope[j + 1].1 += i32::clamp(rope[j].1 - rope[j + 1].1, -1, 1);
            }
            for j in 0..10 {
                trails[j].insert(rope[j]);
            }
        }
    }
    trails[9].iter().count() as i32
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day09.txt").expect("Could not read file");
    
    println!("### Day 9 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input.clone()));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}ms total ({})--", ms.as_millis(), get_env());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#.to_string();

        let moves: Vec<Move> = input.lines().into_iter().map(|l| Move::from_str(l)).collect();
        let mut rp: RopeBridge = RopeBridge::new(2).fill(&moves);

        for mv in moves {
            rp.make_move(&mv);
        }

        let visited = rp.matrix.iter().flatten()
            .filter(|p| p.tail_visited)
            .count();

        assert_eq!(visited, 13);
    }

    #[test]
    fn part_2() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#.to_string();

        let moves: Vec<Move> = input.lines().into_iter().map(|l| Move::from_str(l)).collect();
        let mut trails: Vec<HashSet<(i32, i32)>> = vec![];
        let mut rope: Vec<(i32, i32)> = vec![];
        for i in 0..10 as usize {
            trails.push(HashSet::new());
            rope.push((0, 0));
            trails[i].insert(rope[i]);
        }

        for mv in moves {
            for _ in 0..mv.steps {
                match mv.direction {
                    Direction::Up => rope[0].0 += 1,
                    Direction::Down => rope[0].0 -= 1,
                    Direction::Right => rope[0].1 += 1,
                    Direction::Left => rope[0].1 -= 1
                };
                for j in 0..9 {
                    let x_distance = i32::abs_diff(rope[j].0, rope[j + 1].0);
                    let y_distance = i32::abs_diff(rope[j].1, rope[j + 1].1);
                    if x_distance <= 1 && y_distance <= 1 {
                        continue;
                    }
                    rope[j + 1].0 += i32::clamp(rope[j].0 - rope[j + 1].0, -1, 1);
                    rope[j + 1].1 += i32::clamp(rope[j].1 - rope[j + 1].1, -1, 1);
                }
                for j in 0..10 {
                    trails[j].insert(rope[j]);
                }
            }
        }
        let result = trails[9].iter().count();

        assert_eq!(result, 36);
    }
}