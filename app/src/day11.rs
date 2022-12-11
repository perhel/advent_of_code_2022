use std::{fs, str::Lines, collections::VecDeque};
use itertools::{Chunk, Itertools};
use stopwatch::Stopwatch;

#[cfg(debug_assertions)]
fn get_env() -> &'static str {
    "DEBUG"
}

#[cfg(not(debug_assertions))]
fn get_env() -> &'static str {
    "RELEASE"
}

type Item = u128;

#[derive(Debug, Clone, Copy)]
enum Operation {
    None,
    Add { target: OperationTarget },
    Multiply { target: OperationTarget }
}

impl Operation {
    fn from_line(line: &str) -> Operation {
        let parts = line.split(':')
            .map(|s| s.trim())
            .collect_vec()
            .iter()
            .nth(1).expect("Should be two parts")
            .split(char::is_whitespace)
            .collect_vec();
        let target = OperationTarget::from_str(parts.iter().nth(4).expect("Shuld be at least 4 parts"));
        let operator = parts.iter().nth(3).expect("Shuld be at least 3 parts");
        match *operator {
            "+" => Operation::Add { target },
            "*" => Operation::Multiply { target },
            _ => panic!("Operator not supported")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OperationTarget {
    Number { n: u128 },
    Own
}

impl OperationTarget {
    fn from_str(s: &str) -> OperationTarget {
        match s {
            "old" => OperationTarget::Own,
            _ => OperationTarget::Number { n: s.parse().expect("Should parse") }
        }
    }
}

struct Throw {
    target: i32,
    item: u128
}

struct Monkey {
    id: u32,
    items: VecDeque<Item>,
    operation: Operation,
    test_devisable_by: i32,
    throw_to_true: i32,
    throw_to_false: i32,
    inspections: u64
}

impl Monkey {
    fn new() -> Monkey {
        Monkey { id: 0, items: VecDeque::new(), operation: Operation::None, test_devisable_by: 1, throw_to_true: 0, throw_to_false: 0, inspections: 0 }
    }

    fn from_lines(lines: Chunk<Lines>) -> Monkey {
        let mut monkey = Monkey::new();
        lines.into_iter().take(6).enumerate().for_each(|(i, l)| {
            let parts = l.split(':').map(|s| s.trim()).collect_vec();
            match i {
                0 => monkey.id = l.chars()
                        .nth(7).expect("Should be a number here")
                        .to_digit(10).expect("Should parse"),
                1 => parts[1].trim_matches(char::is_whitespace).split(',')
                        .map(|s| s.trim())
                        .for_each(|s| monkey.items.push_back(s.parse().expect("Should parse"))),
                2 => monkey.operation = Operation::from_line(l),
                3 => monkey.test_devisable_by = parts[1].split(char::is_whitespace).into_iter()
                        .nth(2).expect("Should be 3 parts").parse().expect("Should parse"),
                4 => monkey.throw_to_true = parts[1].split(char::is_whitespace).into_iter()
                        .nth(3).expect("Should be 6 parts").parse().expect("Should parse"),
                5 => monkey.throw_to_false = parts[1].split(char::is_whitespace).into_iter()
                        .nth(3).expect("Should be 6 parts").parse().expect("Should parse"),
                _ => {}
            }
        });

        monkey
    }

    fn inspect_and_return_target(&mut self, part: u8, common_div: i32) -> Throw {
        self.inspections += 1;
        self.perform_operation();
        if part == 1 {
            self.items[0] = ((self.items[0] as f64) / 3_f64).floor() as u128;
        } else {
            self.items[0] %= common_div as u128;
        }
        match self.items[0] as i32 % self.test_devisable_by == 0 {
            true => Throw { target: self.throw_to_true, item: self.items.pop_front().expect("msg") },
            false => Throw { target: self.throw_to_false, item: self.items.pop_front().expect("msg") }
        }
    }

    fn perform_operation(&mut self) {
        match self.operation {
            Operation::Add { target } => match target {
                OperationTarget::Number { n } => self.items[0] += n,
                OperationTarget::Own => self.items[0] += self.items[0]
            },
            Operation::Multiply { target } => match target {
                OperationTarget::Number { n } => self.items[0] *= n,
                OperationTarget::Own => self.items[0] = self.items[0].pow(2)
            },
            Operation::None => panic!()
        };
    }
}

fn part_1(input: String) -> u64 {
    let mut monkeys: Vec<Monkey> = input.lines().into_iter()
        .chunks(7).into_iter()
        .map(|c| Monkey::from_lines(c)).collect();

    let common_div = monkeys.iter().map(|m| m.test_devisable_by)
        .fold(1, |acc, x| acc * x);

    for round in 1.. {
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                let throw = monkeys[m].inspect_and_return_target(1, common_div);
                monkeys[throw.target as usize].items.push_back(throw.item);
            }
        }

        if round == 20 {
            break;
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    monkeys[0].inspections * monkeys[1].inspections
}

fn part_2(input: String) -> u64 {
    let mut monkeys: Vec<Monkey> = input.lines().into_iter()
        .chunks(7).into_iter()
        .map(|c| Monkey::from_lines(c)).collect();

    let common_div = monkeys.iter().map(|m| m.test_devisable_by)
        .fold(1, |acc, x| acc * x);

    for round in 1.. {
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                let throw = monkeys[m].inspect_and_return_target(2, common_div);
                monkeys[throw.target as usize].items.push_back(throw.item);
            }
        }

        if round == 10000 {
            break;
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    monkeys[0].inspections * monkeys[1].inspections
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day11.txt").expect("Could not read file");
    
    println!("### Day 11 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}μs total ({})--", ms.as_micros(), get_env());
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn part_1() {
        let input = r#"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3
    
Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0
    
Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3
    
Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1"#.to_string();

        let mut monkeys: Vec<Monkey> = input.lines().into_iter()
            .chunks(7).into_iter()
            .map(|c| Monkey::from_lines(c)).collect();

        let common_div = monkeys.iter().map(|m| m.test_devisable_by)
            .fold(1, |acc, x| acc * x);

        for round in 1.. {
            for m in 0..monkeys.len() {
                for _ in 0..monkeys[m].items.len() {
                    let throw = monkeys[m].inspect_and_return_target(1, common_div);
                    monkeys[throw.target as usize].items.push_back(throw.item);
                }
            }

            if round == 20 {
                break;
            }
        }

        monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

        let res = monkeys[0].inspections * monkeys[1].inspections;

        assert_eq!(res, 10605);
    }

    #[test]
    fn part_2() {
        let input = r#"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3
    
Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0
    
Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3
    
Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1"#.to_string();

        let mut monkeys: Vec<Monkey> = input.lines().into_iter()
            .chunks(7).into_iter()
            .map(|c| Monkey::from_lines(c)).collect();

        let common_div = monkeys.iter().map(|m| m.test_devisable_by)
            .fold(1, |acc, x| acc * x);

        for round in 1.. {
            for m in 0..monkeys.len() {
                for _ in 0..monkeys[m].items.len() {
                    let throw = monkeys[m].inspect_and_return_target(2, common_div);
                    monkeys[throw.target as usize].items.push_back(throw.item);
                }
            }

            if round == 10000 {
                break;
            }
        }

        monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

        let res = monkeys[0].inspections * monkeys[1].inspections;

        assert_eq!(res, 2713310158);
    }
}