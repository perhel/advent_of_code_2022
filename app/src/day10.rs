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

#[derive(Debug, Clone)]
struct CRT {
    pixels: Vec<Pixel>
}

impl CRT {
    fn new() -> CRT {
        let mut crt = CRT { pixels: vec![Pixel::new(0); 240] };
        
        for i in 0..crt.pixels.len() {
            crt.pixels[i].cycle_pos = (i + 1) as i32;
        };

        crt
    }

    fn process(&mut self, instructions: &Vec<Instruction>) {
        let mut cycle: i32 = 0;
        let mut register_pos: i32 = 1;
        let mut position = 0;

        for instruction in instructions {
            cycle += 1;
            match instruction {
                Instruction::Noop => {
                    if self.sprite_covers_position(register_pos, position) {
                        self.light_pixel(cycle);
                    }
                    
                    position += 1;
                    if self.line_break(position) {
                        position = 0;
                    }
                },
                Instruction::AddX { x } => {
                    if self.sprite_covers_position(register_pos, position) {
                        self.light_pixel(cycle);
                    }

                    cycle += 1;
                    position += 1;
                    if self.line_break(position) {
                        position = 0;
                    }

                    if self.sprite_covers_position(register_pos, position) {
                        self.light_pixel(cycle);
                    }

                    register_pos += x;
                    position += 1;
                    if self.line_break(position) {
                        position = 0;
                    }
                }
            };
        }
    }

    fn sprite_covers_position(&self, register: i32, position: i32) -> bool {
        position == register - 1 ||
        position == register ||
        position == register + 1
    }

    fn line_break(&self, position: i32) -> bool {
        position == 40 ||
        position == 80 ||
        position == 120 ||
        position == 160 ||
        position == 200
    }

    fn light_pixel(&mut self, cycle_pos: i32) {
        self.pixels.iter_mut()
            .find(|p| p.cycle_pos == cycle_pos).unwrap()
            .lit = true;
    }

    fn display(&self) {
        let line_breaks: Vec<i32> = vec![39,79,119,159,199];
        for i in 0..self.pixels.len() as i32 {
            print!("{}", self.pixels[i as usize].display());
            if line_breaks.contains(&i) {
                println!();
            }
        };
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
struct Pixel {
    cycle_pos: i32,
    lit: bool
}

impl Pixel {
    fn new(pos: i32) -> Pixel {
        Pixel { cycle_pos: pos, lit: false }
    }

    fn display(&self) -> char {
        if self.lit {
            '#'
        } else {
            '.'
        }
    }
}

enum Instruction {
    Noop,
    AddX { x: i32 }
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let parts = s.split(char::is_whitespace).collect_vec();
        match parts.len() {
            2 => Instruction::AddX { x: parts[1].parse().expect("Should parse") },
            _ => Instruction::Noop
        }
    }
}

fn part_1(input: String) -> i32 {
    let instructions: Vec<Instruction> = input.lines().into_iter()
        .map(|l| Instruction::from_str(l)).collect();

    let mut cycle = 1;
    let return_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut register = 1;
    let mut signals: Vec<i32> = vec![];

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                if return_cycles.contains(&cycle) {
                    signals.push(cycle * register);
                }
            },
            Instruction::AddX { x } => {
                cycle += 1;
                if return_cycles.contains(&cycle) {
                    signals.push(cycle * register);
                }
                register += x;
                cycle += 1;
                if return_cycles.contains(&cycle) {
                    signals.push(cycle * register);
                }
            }
        }
    }

    signals.iter().sum()
}

fn part_2(input: String) {
    let instructions: Vec<Instruction> = input.lines().into_iter()
        .map(|l| Instruction::from_str(l)).collect();

        let mut crt: CRT = CRT::new();

        crt.process(&instructions);

        crt.display();
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day10.txt").expect("Could not read file");
    
    println!("### Day 10 ###");
    println!("# Part 1: {}", part_1(input.clone()));
    println!("# Part 2:");
    part_2(input);
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}μs total ({})--", ms.as_micros(), get_env());
}