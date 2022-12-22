use std::{fs, collections::HashMap};
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

#[derive(Debug, Clone, Copy)]
struct Monkey<'a> {
    name: &'a str,
    depends_on: Option<(&'a str, &'a str)>,
    message: Message
}

impl <'a> Monkey<'a> {
    fn from_str(s: &'a str) -> Self {
        let ch: &[_] = &[':', ' '];
        let parts = s.split(ch).collect_vec();
        if parts.len() == 3 {
            Monkey {
                name: parts[0],
                depends_on: None,
                message: Message::Integer(parts[2].parse().unwrap())
            }
        } else {
            Monkey {
                name: parts[0],
                depends_on: Some((
                 parts[2],
                 parts[4]
                )),
                message: Message::operation(parts)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Integer(i64),
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Message {
    fn operation(s: Vec<&str>) -> Self {
        match s[3] {
            "+" => Message::Add,
            "-" => Message::Subtract,
            "*" => Message::Multiply,
            "/" => Message::Divide,
            _ => panic!("Invalid operation")
        }
    }

    fn into_inner(&self) -> Option<i64> {
        match self {
            Message::Integer(i) => Some(*i),
            _ => None
        }
    }
}

fn part_1(input: String) -> i64 {
    let monkey_queue = input.lines()
        .map(|l| Monkey::from_str(l))
        .collect_vec();

    let mut known: HashMap<&str, i64> = HashMap::new();
    let mut i = 0;
    while known.len() < monkey_queue.len() {
        let m = monkey_queue[i];
        if m.depends_on == None {
            known.insert(m.name, m.message.into_inner().unwrap());
        } else if let Some((dep_a, dep_b)) = m.depends_on {
            if known.contains_key(dep_a) && known.contains_key(dep_b) {
                let dep_a = known.get(dep_a).unwrap();
                let dep_b = known.get(dep_b).unwrap();
                let int = match m.message {
                    Message::Add => dep_a + dep_b,
                    Message::Subtract => dep_a - dep_b,
                    Message::Multiply => dep_a * dep_b,
                    Message::Divide => dep_a / dep_b,
                    _ => panic!()
                };
                known.insert(m.name, int);
            }
        }

        if i == monkey_queue.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }

    *known.get("root").unwrap()
}

fn part_2(input: String) -> i64 {
    let monkey_queue = input.lines()
        .map(|l| Monkey::from_str(l))
        .collect_vec();

    let correct_assignments = monkey_queue.iter()
        .filter(|m| m.name != "root" && m.name != "humn")
        .collect_vec();

    
    let mut known_numbers: HashMap<&str, i64> = HashMap::new();
    loop {
        let mut new_known = 0;
        for i in 0..correct_assignments.len() {
            let m = correct_assignments[i];
            if m.depends_on == None {
                if let None = known_numbers.insert(m.name, m.message.into_inner().unwrap()) {
                    new_known += 1;
                }
            } else if let Some((dep_a, dep_b)) = m.depends_on {
                if known_numbers.contains_key(dep_a) && known_numbers.contains_key(dep_b) {
                    let dep_a = known_numbers.get(dep_a).unwrap();
                    let dep_b = known_numbers.get(dep_b).unwrap();
                    let int = match m.message {
                        Message::Add => dep_a + dep_b,
                        Message::Subtract => dep_a - dep_b,
                        Message::Multiply => dep_a * dep_b,
                        Message::Divide => dep_a / dep_b,
                        _ => panic!()
                    };
                    if let None = known_numbers.insert(m.name, int) {
                        new_known += 1;
                    }
                }
            }
        }

        if new_known == 0 {
            break;
        }
    }

    let root = monkey_queue.iter().find(|m| m.name == "root").unwrap();
    let root_known_part = known_numbers.iter()
        .filter(|p| p.0 == &root.depends_on.unwrap().0 || p.0 == &root.depends_on.unwrap().1)
        .last().unwrap();

    let mut _unknown_part = "";
    if root_known_part.0 == &root.depends_on.unwrap().0 {
        _unknown_part = root.depends_on.unwrap().1;
    } else {
        _unknown_part = &root.depends_on.unwrap().0;
    }

    let mut should_say = *known_numbers.iter().find(|m| m.0 == root_known_part.0).unwrap().1;
    loop {
        let m = monkey_queue.iter().find(|m| m.name == _unknown_part).unwrap();
        if let Some((a, b)) = m.depends_on {
            let mut backwards = false;
            let mut unknown = a;
            let mut known = b;
            if known_numbers.contains_key(a) {
                unknown = b;
                known = a;
                backwards = true;
            }
            let known = known_numbers.iter().find(|m| *m.0 == known).unwrap();
            let unknown = monkey_queue.iter().find(|m| m.name == unknown).unwrap();
            should_say = match m.message {
                Message::Add => should_say - known.1,
                Message::Subtract => {
                    if backwards {
                        known.1 - should_say
                    } else {
                        should_say + known.1
                    }
                },
                Message::Multiply => should_say / known.1,
                Message::Divide => {
                    if backwards {
                        known.1 / should_say
                    } else {
                        should_say * known.1
                    }
                },
                _ => panic!()
            };

            known_numbers.insert(unknown.name, should_say);
            _unknown_part = unknown.name;
        }

        if _unknown_part == "humn" {
            break;
        }
    }

    *known_numbers.get("humn").unwrap()
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day21.txt").expect("Could not read file");
    
    println!("### Day 21 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}ms total ({})--", ms.as_millis(), get_env());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use super::*;

    #[test]
    fn part_1() {
        let input = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#.to_string();

        let monkey_queue = input.lines()
            .map(|l| Monkey::from_str(l))
            .collect_vec();

        let mut known: HashMap<&str, i64> = HashMap::new();
        let mut i = 0;
        while known.len() < monkey_queue.len() {
            let m = monkey_queue[i];
            if m.depends_on == None {
                known.insert(m.name, m.message.into_inner().unwrap());
            } else if let Some((dep_a, dep_b)) = m.depends_on {
                if known.contains_key(dep_a) && known.contains_key(dep_b) {
                    let dep_a = known.get(dep_a).unwrap();
                    let dep_b = known.get(dep_b).unwrap();
                    let int = match m.message {
                        Message::Add => dep_a + dep_b,
                        Message::Subtract => dep_a - dep_b,
                        Message::Multiply => dep_a * dep_b,
                        Message::Divide => dep_a / dep_b,
                        _ => panic!()
                    };
                    known.insert(m.name, int);
                }
            }

            if i == monkey_queue.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
        }

        let result = known.get("root").unwrap();
        
        assert_eq!(*result, 152);
    }

    #[test]
    fn part_2() {
        let input = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#.to_string();

        let monkey_queue = input.lines()
            .map(|l| Monkey::from_str(l))
            .collect_vec();

        let correct_assignments = monkey_queue.iter()
            .filter(|m| m.name != "root" && m.name != "humn")
            .collect_vec();

        
        let mut known_numbers: HashMap<&str, i64> = HashMap::new();
        loop {
            let mut new_known = 0;
            for i in 0..correct_assignments.len() {
                let m = correct_assignments[i];
                if m.depends_on == None {
                    if let None = known_numbers.insert(m.name, m.message.into_inner().unwrap()) {
                        new_known += 1;
                    }
                } else if let Some((dep_a, dep_b)) = m.depends_on {
                    if known_numbers.contains_key(dep_a) && known_numbers.contains_key(dep_b) {
                        let dep_a = known_numbers.get(dep_a).unwrap();
                        let dep_b = known_numbers.get(dep_b).unwrap();
                        let int = match m.message {
                            Message::Add => dep_a + dep_b,
                            Message::Subtract => dep_a - dep_b,
                            Message::Multiply => dep_a * dep_b,
                            Message::Divide => dep_a / dep_b,
                            _ => panic!()
                        };
                        if let None = known_numbers.insert(m.name, int) {
                            new_known += 1;
                        }
                    }
                }
            }

            if new_known == 0 {
                break;
            }
        }

        let root = monkey_queue.iter().find(|m| m.name == "root").unwrap();
        let root_known_part = known_numbers.iter()
            .filter(|p| p.0 == &root.depends_on.unwrap().0 || p.0 == &root.depends_on.unwrap().1)
            .last().unwrap();

        let mut _unknown_part = "";
        if root_known_part.0 == &root.depends_on.unwrap().0 {
            _unknown_part = root.depends_on.unwrap().1;
        } else {
            _unknown_part = &root.depends_on.unwrap().0;
        }

        let mut should_say = *known_numbers.iter().find(|m| m.0 == root_known_part.0).unwrap().1;
        loop {
            let m = monkey_queue.iter().find(|m| m.name == _unknown_part).unwrap();
            if let Some((a, b)) = m.depends_on {
                let mut backwards = false;
                let mut unknown = a;
                let mut known = b;
                if known_numbers.contains_key(a) {
                    unknown = b;
                    known = a;
                    backwards = true;
                }
                let known = known_numbers.iter().find(|m| *m.0 == known).unwrap();
                let unknown = monkey_queue.iter().find(|m| m.name == unknown).unwrap();
                should_say = match m.message {
                    Message::Add => should_say - known.1,
                    Message::Subtract => {
                        if backwards {
                            known.1 - should_say
                        } else {
                            should_say + known.1
                        }
                    },
                    Message::Multiply => should_say / known.1,
                    Message::Divide => {
                        if backwards {
                            known.1 / should_say
                        } else {
                            should_say * known.1
                        }
                    },
                    _ => panic!()
                };

                known_numbers.insert(unknown.name, should_say);
                _unknown_part = unknown.name;
            }

            if _unknown_part == "humn" {
                break;
            }
        }

        let result = known_numbers.get("humn").unwrap();

        assert_eq!(*result, 301);
    }
}