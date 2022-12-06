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

#[derive(Clone)]
struct Ship {
    stacks: Vec<(u8, Vec<char>)>
}

impl Ship {
    fn from_string_vec(input: Vec<String>) -> Ship {
        let mut stacks: Vec<(u8, Vec<char>)> = Vec::new();

        let mut two_d_map: Vec<Vec<(usize, char)>> = input.iter()
            .map(|s| s.char_indices()
                .collect())
            .collect();

        two_d_map.reverse();

        for outer in two_d_map {
            for inner in outer {
                if let Some(_) = inner.1.to_digit(10) {
                    stacks.push((inner.0 as u8, vec![]));
                } else if inner.1.is_ascii_alphabetic() {
                    stacks.iter_mut()
                        .find(|s| s.0 == inner.0 as u8)
                        .expect("Should be a mathing tuple in there...").1
                        .push(inner.1);
                }
            }
        }

        for i in 0..stacks.len() {
            stacks[i].0 = (i + 1) as u8;
        }

        Ship { stacks }
    }

    fn move_crate(&mut self, mv: &Move) {
        for _ in 0..mv.amount {
            let from =  &self.stacks.iter_mut()
                .find(|s| s.0 == mv.from)
                .expect("Should be there").1.pop().expect("Should be a value here");

            self.stacks.iter_mut()
                .find(|s| s.0 == mv.to)
                .expect("Should be there").1.push(*from);
        }
    }

    fn move_crates(&mut self, mv: &Move) {
        let mut crane: Vec<char> = vec![];
        for _ in 0..mv.amount {
            let from =  &self.stacks.iter_mut()
                .find(|s| s.0 == mv.from)
                .expect("Should be there").1.pop().expect("Should be a value here");

            crane.push(*from);
        }

        crane.reverse();
        
        self.stacks.iter_mut()
            .find(|s| s.0 == mv.to)
            .expect("Should be there").1.append(&mut crane);
    }

    fn get_top_crates(&self) -> String {
        self.stacks.to_owned().iter_mut()
            .map(|s| s.1.last().expect("Should be at least one element vec"))
            .join("")
    }
}

struct Move {
    amount: u8,
    from: u8,
    to: u8
}

impl Move {
    fn from_string_vec(input: Vec<String>) -> Vec<Move> {
        input.iter().map(|a| Move::from_string(a.trim())).collect_vec()
    }

    fn from_string(input: &str) -> Move {
        let split = input.split(char::is_whitespace).collect_vec();
        
        Move { 
            amount: split[1].parse().expect("Should parse"),
            from: split[3].parse().expect("Should parse"),
            to: split[5].parse().expect("Should parse")
        }
    }
}

fn part_1(mut ship: Ship, moves: &Vec<Move>) -> String {
    for mv in moves {
        ship.move_crate(mv);
    }
    
    ship.get_top_crates()
}

fn part_2(mut ship: Ship, moves: &Vec<Move>) -> String {
    for mv in moves {
        ship.move_crates(mv);
    }
    
    ship.get_top_crates()
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day05.txt").expect("Could not read file");
    
    let (ship, moves) = extract_ship_and_moves(input.clone());
    
    println!("### Day 5 ###");
    println!("# Part 1: {}", part_1(ship.clone(), &moves));
    println!("# Part 2: {}", part_2(ship, &moves));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}μs total ({})--", ms.as_micros(), get_env());
}

fn extract_ship_and_moves(input: String) -> (Ship, Vec<Move>) {
    input.lines().collect_vec()
        .split(|line| line.trim().len() == 0)
        .take(2)
        .map(|a| a.to_vec().iter().map(|&s| s.to_string()).collect())
        .collect_tuple::<(Vec<String>, Vec<String>)>()
        .map(|(ship, moves)| (Ship::from_string_vec(ship), Move::from_string_vec(moves))).expect("Should work. Maybe")

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
        
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#.to_string();

        let (mut ship, moves) = extract_ship_and_moves(input);

        for mv in moves {
            ship.move_crate(&mv);
        }

        let res = ship.get_top_crates();
        
        assert_eq!(res, "CMZ".to_string());
    }

    #[test]
    fn part_2() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
        
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#.to_string();

        let (mut ship, moves) = extract_ship_and_moves(input);

        for mv in moves {
            ship.move_crates(&mv);
        }

        let res = ship.get_top_crates();
        
        assert_eq!(res, "MCD".to_string());
    }
}