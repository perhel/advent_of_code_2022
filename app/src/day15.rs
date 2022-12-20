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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn get_distance(&self, target: Coordinate) -> i32 {
        (self.x - target.x).abs() + (self.y - target.y).abs()
    }
}

struct Sensor {
    coordinates: Coordinate,
    beacon: Coordinate
}

impl Sensor {
    fn parse(l: &str) -> Self {
        let parts = l.split(&['=', ',', ':'][..]).collect_vec();
        Sensor {
            coordinates: Coordinate {
                x: parts[1].parse().unwrap(),
                y: parts[3].parse().unwrap()
            },
            beacon: Coordinate {
                x: parts[5].parse().unwrap(),
                y: parts[7].parse().unwrap()
            }
        }
    }

    fn scanned_x_range_at_y(&self, y: i32, limit: Option<(i32, i32)>) -> Option<(i32, i32)> {
        let max_distance = self.coordinates.get_distance(self.beacon);
        let x_range = (max_distance - (self.coordinates.y - y).abs()).abs();

        if let Some(lim) = limit {
            if (self.coordinates.y - y).abs() > max_distance {
                return None
            }
            let x_lower = (self.coordinates.x - x_range).clamp(lim.0, lim.1);
            let x_upper = (self.coordinates.x + x_range).clamp(lim.0, lim.1);

            Some((x_lower, x_upper))
        } else {
            Some((self.coordinates.x - x_range, self.coordinates.x + x_range - 1))
        }
    }
}

fn part_1(input: String) -> usize {
    let sensors: Vec<Sensor> = input.lines().into_iter()
        .map(|l| Sensor::parse(l)).collect_vec();

    let res = sensors.iter()
        .map(|s| s.scanned_x_range_at_y(2000000, None))
        .filter(|r| r.is_some()).map(|r| r.unwrap())
        .fold(HashSet::new(), |mut acc, r| {
            for x in r.0..=r.1 {
                acc.insert(x); 
            }
            acc
        });

    res.len()
}

fn part_2(input: String) -> u64 {
    let sensors: Vec<Sensor> = input.lines().into_iter()
        .map(|l| Sensor::parse(l)).collect_vec();

    let mut res = (0, 0);
    'y: for y in 2000001..4000001 {
        let sweeps_at_y = sensors.iter()
            .map(|s| s.scanned_x_range_at_y(y, Some((0, 4000000))))
            .filter(|r| r.is_some()).map(|r| r.unwrap())
            .sorted_by(|a, b| a.0.cmp(&b.0)).collect_vec();

        if sweeps_at_y[0].0 != 0 {
            res = (0, y);
            break 'y;
        }
        
        let mut _reduced_scan = (0, 0);
        'x: for sweep in sweeps_at_y {
            if sweep.0 <= _reduced_scan.1 && sweep.1 <= _reduced_scan.1 {
                continue 'x;
            }
            if sweep.0 > _reduced_scan.1 + 1 {
                res = (_reduced_scan.1 + 1, y);
                break 'y;
            }
            if sweep.1 == 4000000 {
                _reduced_scan = (sweep.1, y);
                break 'x;
            }
            _reduced_scan.1 = sweep.1;
        }

    }

    res.0 as u64 * 4000000 + res.1 as u64
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day15.txt").expect("Could not read file");
    
    println!("### Day 15 ###");
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
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#.to_string();

        let sensors: Vec<Sensor> = input.lines().into_iter()
            .map(|l| Sensor::parse(l)).collect_vec();

        let res = sensors.iter()
            .map(|s| s.scanned_x_range_at_y(10, None))
            .filter(|r| r.is_some()).map(|r| r.unwrap())
            .fold(HashSet::new(), |mut acc, r| {
                for x in r.0..=r.1 {
                   acc.insert(x); 
                }
                acc
            });

        assert_eq!(res.len(), 26);
    }

    #[test]
    fn part_2() {
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#.to_string();

        let sensors: Vec<Sensor> = input.lines().into_iter()
            .map(|l| Sensor::parse(l)).collect_vec();

        let mut res = (0, 0);
        'y: for y in 0..21 {
            let sweeps_at_y = sensors.iter()
                .map(|s| s.scanned_x_range_at_y(y, Some((0, 20))))
                .filter(|r| r.is_some()).map(|r| r.unwrap())
                .sorted_by(|a, b| a.0.cmp(&b.0)).collect_vec();

            if sweeps_at_y[0].0 != 0 {
                res = (0, y);
                break 'y;
            }
            
            let mut _reduced_scan = (0, 0);
            'x: for sweep in sweeps_at_y {
                if sweep.0 <= _reduced_scan.1 && sweep.1 <= _reduced_scan.1 {
                    continue 'x;
                }
                if sweep.0 > _reduced_scan.1 + 1 {
                    res = (_reduced_scan.1 + 1, y);
                    break 'y;
                }
                if sweep.1 == 20 {
                    _reduced_scan = (sweep.1, y);
                    break 'x;
                }
                _reduced_scan.1 = sweep.1;
            }
        }

        let result = res.0 * 4000000 + res.1;

        assert_eq!(result, 56000011);
    }
}