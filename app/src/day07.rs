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

struct Filesystem {
    directories: Vec<Directory>
}

impl Filesystem {
    fn new() -> Filesystem {
        Filesystem { directories: vec![] }
    }

    fn populate(mut self, input: &String) -> Self {
        let mut iterator = input.lines().into_iter();

        let mut current_dir: Vec<String> = vec![];

        while let Some(line) = iterator.next() {
            match CommandLine::from_str(line) {
                CommandLine::CD { dir } => {
                    if dir == ".." {
                        current_dir.pop();
                    } else {
                        current_dir.push(dir);
                        self.directories.push(Directory { path: current_dir.join("/"), file_sizes: vec![] });
                    }
                },
                CommandLine::LS => {
                    'inner: while let Some(output) = iterator.next() {
                        match CommandLine::from_str(output) {
                            CommandLine::Output { content } => {
                                match content {
                                    Content::File { size } => {
                                        self.directories.iter_mut()
                                            .find(|d| d.path.as_str() == current_dir.join("/"))
                                            .expect("msg").file_sizes.push(size);
                                    },
                                    _ => continue 'inner
                                }
                            },
                            CommandLine::CD { dir } => {
                                if dir == ".." {
                                    current_dir.pop();
                                } else {
                                    current_dir.push(dir);
                                    self.directories.push(Directory { path: current_dir.join("/"), file_sizes: vec![] });
                                }
                                break 'inner;
                            },
                            _ => {
                                break 'inner;
                            }
                        }
                    }
                },
                _ => break
            }
        }
        
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Content {
    Directory { dir: Directory, size: u32 },
    File { size: u32 }
}

impl Content {
    fn from_str(s: &str) -> Content {
        let split = s.split(char::is_whitespace).collect_vec();
        if let Ok(size) = split[0].parse::<u32>() {
            Content::File { size }
        } else {
            Content::Directory { dir: Directory { path: String::new(), file_sizes: vec![] }, size: 0 }
        }
    }
}

#[derive(Debug, PartialEq)]
enum CommandLine {
    CD { dir: String},
    LS,
    Output { content: Content }
}

impl CommandLine {
    fn from_str(s: &str) -> CommandLine {
        let parts = s.split(char::is_whitespace).collect_vec();
        match parts[1] {
            "cd" => CommandLine::CD { dir: parts[2].into() },
            "ls" => CommandLine::LS,
            _ => CommandLine::Output { content: Content::from_str(s) }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Directory {
    path: String,
    file_sizes: Vec<u32>
}

fn part_1(filesystem: &Filesystem) -> u32 {
    filesystem.directories.iter()
        .map(|d| {
            filesystem.directories.iter()
                .filter(|&di| di.path.starts_with(d.path.as_str()))
                .map(|dii| dii.file_sizes.iter().sum::<u32>()).sum::<u32>()
        })
        .filter(|&s| s <= 100000)
        .sum()
}

fn part_2(filesystem: &Filesystem) -> u32 {
    let dirs: Vec<u32> = filesystem.directories.iter()
        .map(|d| {
            filesystem.directories.iter()
                .filter(|&di| di.path.starts_with(d.path.as_str()))
                .map(|dii| dii.file_sizes.iter().sum::<u32>()).sum::<u32>()
        }).sorted().collect();

    let tot_size = 70000000;
    let used_size = dirs.iter().sorted_by(|a, b| a.cmp(b)).last().unwrap();
    let target_free = 30000000;
    let result = dirs.iter().filter(|&s| tot_size - used_size + s >= target_free)
        .sorted_by(|a, b| b.cmp(a))
        .last().unwrap();

    let d_dir = filesystem.directories.iter().map(|d| {
        let size = filesystem.directories.iter()
            .filter(|&di| di.path.starts_with(d.path.as_str()))
            .map(|dii| dii.file_sizes.iter().sum::<u32>()).sum::<u32>();

        (d.path.clone(), size)
    }).find(|x| x.1 == *result)
    .unwrap();

    d_dir.1
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input = fs::read_to_string("inputs/2022/day07.txt").expect("Could not read file");

    let filesystem = Filesystem::new().populate(&input);
    
    println!("### Day 7 ###");
    println!("# Part 1: {}", part_1(&filesystem));
    println!("# Part 2: {}", part_2(&filesystem));
    let ms = sw.elapsed();
    sw.stop();
    println!("-- {}μs total ({})--", ms.as_micros(), get_env());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#.to_string();

        let filesystem = Filesystem::new().populate(&input);
        let result: u32 = filesystem.directories.iter()
            .map(|d| {
                filesystem.directories.iter()
                    .filter(|&di| di.path.starts_with(d.path.as_str()))
                    .map(|dii| dii.file_sizes.iter().sum::<u32>()).sum::<u32>()
            })
            .filter(|&s| s <= 100000)
            .sum();

        assert_eq!(result, 95437);
    }

    #[test]
    fn part_2() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#.to_string();

        let filesystem = Filesystem::new().populate(&input);
        let dirs: Vec<u32> = filesystem.directories.iter()
            .map(|d| {
                filesystem.directories.iter()
                    .filter(|&di| di.path.starts_with(d.path.as_str()))
                    .map(|dii| dii.file_sizes.iter().sum::<u32>()).sum::<u32>()
            }).sorted().collect();

        let tot_size = 70000000;
        let used_size = dirs.iter().sorted_by(|a, b| a.cmp(b)).last().unwrap();
        let target_free = 30000000;
        let result = dirs.iter().filter(|&s| tot_size - used_size + s >= target_free)
            .sorted_by(|a, b| b.cmp(a))
            .last().unwrap();

        let d_dir = filesystem.directories.iter().map(|d| {
            let size = filesystem.directories.iter()
                .filter(|&di| di.path.starts_with(d.path.as_str()))
                .map(|dii| dii.file_sizes.iter().sum::<u32>()).sum::<u32>();

            (d.path.clone(), size)
        }).find(|x| x.1 == *result)
        .unwrap();

        assert_eq!(d_dir.1, 24933642);
    }
}