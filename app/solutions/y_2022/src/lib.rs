use models::{Input, RockPaperScissorsRound};

pub fn day_2_p_1(input: &Input) -> u32 {
    input.lines.iter()
        .map(|s| 
            RockPaperScissorsRound::from(s.clone())
        )
        .map(|r| r.your_score)
        .sum()
}

pub fn day_2_p_2(input: &Input) -> u32 {
    input.lines.iter()
        .map(|s| 
            RockPaperScissorsRound::from(s.clone())
            .from_expected_outcome()
        )
        .map(|r| r.your_score)
        .sum()
}