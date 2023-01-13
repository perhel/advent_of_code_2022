use std::{fs, collections::VecDeque, cmp::Ordering};
use itertools::Itertools;
use serde_json::Value;
use stopwatch::Stopwatch;

#[cfg(debug_assertions)]
fn get_env() -> &'static str {
    "DEBUG"
}

#[cfg(not(debug_assertions))]
fn get_env() -> &'static str {
    "RELEASE"
}

#[derive(Debug, Clone, PartialEq)]
enum PacketPart {
    Int(i64),
    List(VecDeque<PacketPart>)
}

impl PacketPart {
    fn from_str(s: &str) -> VecDeque<PacketPart> {
        let json_val: Value = serde_json::from_str(s).unwrap();
        let arr = json_val.as_array().unwrap();
        
        PacketPart::from_value_vec(arr)
    }

    fn from_value_vec(v: &Vec<Value>) -> VecDeque<PacketPart> {
        let mut output = VecDeque::new();
        v.into_iter().for_each(|v| {
            match v {
                Value::Number(n) => output.push_back(PacketPart::Int(n.as_i64().unwrap())),
                Value::Array(arr) => output.push_back(PacketPart::List(PacketPart::from_value_vec(arr))),
                _ => panic!()
            }
        });

        output
    }

    fn cmp(l: PacketPart, r: PacketPart) -> Order {
        match (l.clone(), r.clone()) {
            (PacketPart::Int(n_l), PacketPart::Int(n_r)) => Order::new(n_l.cmp(&n_r)),
            (PacketPart::List(mut arr_l), PacketPart::List(mut arr_r)) => {
                while let Some(l) = arr_l.pop_front() {
                    if let Some(r) = arr_r.pop_front() {
                        let o = PacketPart::cmp(l, r); 
                        match o {
                            Order::Equal => continue,
                            _ => return o
                        }
                    } else {
                        return Order::Wrong;
                    }
                }

                if arr_r.len() != 0 {
                    return Order::Correct;
                }

                Order::Equal
            },
            (PacketPart::List(_), PacketPart::Int(_)) => PacketPart::cmp(l, PacketPart::List(vec![r].into())),
            (PacketPart::Int(_), PacketPart::List(_)) => PacketPart::cmp(PacketPart::List(vec![l].into()), r)
        }
    }

    fn compare_packets(left: &mut VecDeque<PacketPart>, right: &mut VecDeque<PacketPart>) -> Order {
        while let Some(l) = left.pop_front() {
            if let Some(r) = right.pop_front() {
                let o = PacketPart::cmp(l, r); 
                    match o {
                        Order::Equal => continue,
                        _ => return o
                    }
            } else {
                return Order::Wrong;
            }
        }

        if right.len() != 0 {
            return Order::Correct;
        }

        Order::Equal
    }

    fn is_divider_packet(&self) -> bool {
        if *self == PacketPart::List(VecDeque::from([PacketPart::List(VecDeque::from([PacketPart::Int(2)]))])) ||
            *self == PacketPart::List(VecDeque::from([PacketPart::List(VecDeque::from([PacketPart::Int(6)]))])) {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
enum Order {
    Correct,
    Wrong,
    Equal
}

impl Order {
    fn new(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => Order::Correct,
            Ordering::Greater => Order::Wrong,
            Ordering::Equal => Order::Equal
        }
    }

    fn is_correct(&self) -> bool {
        if *self == Order::Correct {
            true
        } else {
            false
        }
    }

    fn as_ordering(&self) -> Ordering {
        match self {
            Order::Correct => Ordering::Less,
            Order::Equal => Ordering::Equal,
            Order::Wrong => Ordering::Greater
        }
    }
}

fn part_1(input: String) -> u32 {
    let pairs = input.lines()
        .filter(|l| l.len() > 0)
        .chunks(2)
        .into_iter()
        .map(|c| c.into_iter()
            .map(|l| PacketPart::from_str(l))
            .collect_tuple::<(VecDeque<PacketPart>, VecDeque<PacketPart>)>()
            .unwrap()
        ).collect_vec();

    pairs.iter().enumerate()
        .map(|(i, (left, right))| (i + 1, PacketPart::compare_packets(&mut left.clone(), &mut right.clone())))
        .filter(|(_, order)| order.is_correct())
        .fold(0, |mut acc, (i, _)| {
            acc += i as u32;
            acc
        })
}

fn part_2(mut input: String) -> u32 {
    input.push_str("\n[[2]]");
    input.push_str("\n[[6]]");

    input.lines()
        .filter(|l| l.len() > 0)
        .map(|l| PacketPart::List(PacketPart::from_str(l)))
        .sorted_by(|a, b| PacketPart::cmp(a.clone(), b.clone(),).as_ordering())
        .into_iter().enumerate()
        .fold(1, |mut acc, (index, part)| {
            if part.is_divider_packet() {
                acc *= index as u32 + 1;
            }

            acc
        })
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day13.txt").expect("Could not read file");
    
    println!("### Day 13 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2: {}", part_2(input));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}ms total ({})--", ms.as_millis(), get_env());
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn part_1() {
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#.to_string();

        let pairs = input.lines()
            .filter(|l| l.len() > 0)
            .chunks(2)
            .into_iter()
            .map(|c| c.into_iter()
                .map(|l| PacketPart::from_str(l))
                .collect_tuple::<(VecDeque<PacketPart>, VecDeque<PacketPart>)>()
                .unwrap()
            ).collect_vec();

        let sum = pairs.iter().enumerate()
            .map(|(i, (left, right))| (i + 1, PacketPart::compare_packets(&mut left.clone(), &mut right.clone())))
            .filter(|(_, order)| order.is_correct())
            .fold(0, |mut acc, (i, _)| {
                acc += i as i32;
                acc
            });

        assert_eq!(sum, 13);
    }

    #[test]
    fn part_2() {
        let mut input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#.to_string();

        input.push_str("\n[[2]]");
        input.push_str("\n[[6]]");
        let divider_packet_indice_product = input.lines()
            .filter(|l| l.len() > 0)
            .map(|l| PacketPart::List(PacketPart::from_str(l)))
            .sorted_by(|a, b| PacketPart::cmp(a.clone(), b.clone(),).as_ordering())
            .into_iter().enumerate()
            .fold(1, |mut acc, (index, part)| {
                if part.is_divider_packet() {
                    acc *= index as i32 + 1;
                }

                acc
            });

        assert_eq!(divider_packet_indice_product, 140);
    }
}