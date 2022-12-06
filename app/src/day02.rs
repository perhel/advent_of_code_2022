use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Weapon {
    Rock,
    Paper,
    Scissors
}

impl Weapon {
    pub fn from_str(str: &str) -> Weapon {
        match str {
            "A" | "X" => Weapon::Rock,
            "B" | "Y" => Weapon::Paper,
            "C" | "Z" => Weapon::Scissors,
            _ => panic!("Invalid input")
        }
    }

    pub fn get_score(you: Weapon, other: Weapon) -> u32 {
        let mut score: u32 = 0;

        match you {
            Weapon::Rock => score += 1,
            Weapon::Paper => score += 2,
            Weapon::Scissors => score += 3
        };

        match Outcome::cmp(you, other) {
            Outcome::Win => score += 6,
            Outcome::Draw => score += 3,
            Outcome::Loss => score += 0
        };

        score
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw
}

impl Outcome {
    pub fn cmp(you: Weapon, other: Weapon) -> Self {
        match (you, other) {
            (Weapon::Rock, Weapon::Scissors) |
            (Weapon::Paper, Weapon::Rock) |
            (Weapon::Scissors, Weapon::Paper)
                => Outcome::Win,
            (Weapon::Rock, Weapon::Paper) |
            (Weapon::Paper, Weapon::Scissors) |
            (Weapon::Scissors, Weapon::Rock)
                => Outcome::Loss,
            _ => Outcome::Draw
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    pub opponent: Weapon,
    pub you: Option<Weapon>,
    pub target_outcome: Option<Outcome>,
    pub your_score: u32
}

impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let choices: Vec<&str> = s.split(char::is_whitespace).collect();
        if choices.len() != 2 {
            panic!("Wrong number of arguments.")
        };

        let opponent = Weapon::from_str(choices[0]);
        let you = Weapon::from_str(choices[1]);

        Round {
            opponent,
            you: Some(you),
            target_outcome: None,
            your_score: Weapon::get_score(you, opponent)
        }
    }
}

impl Round {
    pub fn from_expected_outcome(&mut self) -> Round {
        if let Some(hand) = self.you {
            match hand {
                Weapon::Rock => self.target_outcome = Some(Outcome::Loss),
                Weapon::Paper => self.target_outcome = Some(Outcome::Draw),
                Weapon::Scissors => self.target_outcome = Some(Outcome::Win)
            };
            self.you = match (self.opponent, self.target_outcome.as_ref().unwrap()) {
                (Weapon::Rock, Outcome::Win) => Some(Weapon::Paper),
                (Weapon::Rock, Outcome::Loss) => Some(Weapon::Scissors),
                (Weapon::Paper, Outcome::Win) => Some(Weapon::Scissors),
                (Weapon::Paper, Outcome::Loss) => Some(Weapon::Rock),
                (Weapon::Scissors, Outcome::Win) => Some(Weapon::Rock),
                (Weapon::Scissors, Outcome::Loss) => Some(Weapon::Paper),
                _ => Some(self.opponent.clone())
            };
            self.your_score = Weapon::get_score(self.you.unwrap(), self.opponent);
        };
        self.clone()
    }
}

fn part_1(input: String) -> u32 {
    input.lines().into_iter()
        .map(|s| Round::from(s.clone()))
        .map(|r| r.your_score)
        .sum()
}

fn part_2(input: String) -> u32 {
    input.lines().into_iter()
        .map(|s| 
            Round::from(s.clone())
            .from_expected_outcome()
        )
        .map(|r| r.your_score)
        .sum()
}

fn main() {
    let input = fs::read_to_string("inputs/2022/day02.txt").expect("Could not read file");

    println!("### Day 2 ###");
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"A Y
        B X
        C Z"#;
        let sum: u32 = input.lines().into_iter()
            .map(|s| Round::from(s.clone().trim()))
            .map(|r| r.your_score)
            .sum();

        assert_eq!(sum, 15);
    }

    #[test]
    fn part_2() {
        let input = r#"A Y
        B X
        C Z"#;
        let rounds: Vec<Round> = input.lines().into_iter()
            .map(|s| 
                Round::from(s.clone().trim())
                .from_expected_outcome()
            )
            .collect();

        let sum: u32 = rounds.into_iter()
            .map(|r| r.your_score)
            .sum();

        assert_eq!(sum, 12);
    }
}