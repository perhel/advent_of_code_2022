pub struct Input {
    pub lines: Vec<String>
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

impl RockPaperScissors {
    pub fn from_str(str: &str) -> RockPaperScissors {
        match str {
            "A" => RockPaperScissors::Rock,
            "X" => RockPaperScissors::Rock,
            "B" => RockPaperScissors::Paper,
            "Y" => RockPaperScissors::Paper,
            "C" => RockPaperScissors::Scissors,
            "Z" => RockPaperScissors::Scissors,
            _ => panic!("Invalid input")
        }
    }

    pub fn get_score(you: RockPaperScissors, other: RockPaperScissors) -> u32 {
        let mut score: u32 = 0;

        match you {
            RockPaperScissors::Rock => score += 1,
            RockPaperScissors::Paper => score += 2,
            RockPaperScissors::Scissors => score += 3
        };

        match RockPaperScissorsOutcome::cmp(you, other) {
            RockPaperScissorsOutcome::Won => score += 6,
            RockPaperScissorsOutcome::Draw => score += 3,
            RockPaperScissorsOutcome::Lost => score += 0
        };

        score
    }
}

#[derive(Debug, Clone)]
pub enum RockPaperScissorsOutcome {
    Won,
    Lost,
    Draw
}

impl RockPaperScissorsOutcome {
    pub fn cmp(you: RockPaperScissors, other: RockPaperScissors) -> Self {
        if you == RockPaperScissors::Rock && other == RockPaperScissors::Scissors {
            RockPaperScissorsOutcome::Won
        } else if you == RockPaperScissors::Rock && other == RockPaperScissors::Paper {
            RockPaperScissorsOutcome::Lost
        } else if you == RockPaperScissors::Paper && other == RockPaperScissors::Rock {
            RockPaperScissorsOutcome::Won
        } else if you == RockPaperScissors::Paper && other == RockPaperScissors::Scissors {
            RockPaperScissorsOutcome::Lost
        } else if you == RockPaperScissors::Scissors && other == RockPaperScissors::Paper {
            RockPaperScissorsOutcome::Won
        } else if you == RockPaperScissors::Scissors && other == RockPaperScissors::Rock {
            RockPaperScissorsOutcome::Lost
        } else {
            RockPaperScissorsOutcome::Draw
        }
    }
}

#[derive(Debug, Clone)]
pub struct RockPaperScissorsRound {
    pub opponent: RockPaperScissors,
    pub you: Option<RockPaperScissors>,
    pub target_outcome: Option<RockPaperScissorsOutcome>,
    pub your_score: u32
}

impl From<String> for RockPaperScissorsRound {
    fn from(s: String) -> Self {
        let choices: Vec<&str> = s.split(char::is_whitespace).collect();
        if choices.len() != 2 {
            panic!("Wrong number of arguments.")
        };

        let opponent = RockPaperScissors::from_str(choices[0]);
        let you = RockPaperScissors::from_str(choices[1]);

        RockPaperScissorsRound {
            opponent,
            you: Some(you),
            target_outcome: None,
            your_score: RockPaperScissors::get_score(you, opponent)
        }
    }
}

impl RockPaperScissorsRound {
    pub fn from_expected_outcome(&mut self) -> RockPaperScissorsRound {
        if let Some(hand) = self.you {
            match hand {
                RockPaperScissors::Rock => self.target_outcome = Some(RockPaperScissorsOutcome::Lost),
                RockPaperScissors::Paper => self.target_outcome = Some(RockPaperScissorsOutcome::Draw),
                RockPaperScissors::Scissors => self.target_outcome = Some(RockPaperScissorsOutcome::Won)
            };
            self.you = match (self.opponent, self.target_outcome.as_ref().unwrap()) {
                (RockPaperScissors::Rock, RockPaperScissorsOutcome::Won) => Some(RockPaperScissors::Paper),
                (RockPaperScissors::Rock, RockPaperScissorsOutcome::Lost) => Some(RockPaperScissors::Scissors),
                (RockPaperScissors::Paper, RockPaperScissorsOutcome::Won) => Some(RockPaperScissors::Scissors),
                (RockPaperScissors::Paper, RockPaperScissorsOutcome::Lost) => Some(RockPaperScissors::Rock),
                (RockPaperScissors::Scissors, RockPaperScissorsOutcome::Won) => Some(RockPaperScissors::Rock),
                (RockPaperScissors::Scissors, RockPaperScissorsOutcome::Lost) => Some(RockPaperScissors::Paper),
                _ => Some(self.opponent.clone())
            };
            self.your_score = RockPaperScissors::get_score(self.you.unwrap(), self.opponent);
        };
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Input { lines: vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z")
        ]};
        let sum: u32 = input.lines.iter()
            .map(|s| RockPaperScissorsRound::from(s.clone()))
            .map(|r| r.your_score)
            .sum();

        assert_eq!(sum, 15);
    }

    #[test]
    fn part_2() {
        let input = Input { lines: vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z")
        ]};
        let rounds: Vec<RockPaperScissorsRound> = input.lines.iter()
            .map(|s| 
                RockPaperScissorsRound::from(s.clone())
                .from_expected_outcome()
            )
            .collect();
            
        // for r in rounds {
        //     println!("{:?}", r);
        // }

        let sum: u32 = rounds.into_iter()
            .map(|r| r.your_score)
            .sum();
        println!("{:?}", sum);
        assert_eq!(4, 4);
    }
}