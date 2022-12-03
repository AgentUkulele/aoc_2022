#[cfg(test)]
static FILE: &'static str = include_str!("./day2.test");

#[cfg(not(test))]
static FILE: &'static str = include_str!("./day2.prod");

pub fn day2_1() -> usize {
    let total_score = FILE
        .lines()
        .map(|game| {
            let mut score = 0;

            let selections = game
                .split(" ")
                .map(|s| match s {
                    "A" | "X" => 1,
                    "B" | "Y" => 2,
                    "C" | "Z" => 3,
                    _ => panic!("Invalid RPS state provided"),
                })
                .collect::<Vec<usize>>();

            let (mut player_1, mut player_2) = (selections[0], selections[1]);

            score += player_2;

            player_1 -= 1;
            player_2 -= 1;

            if (player_1 + 1) % 3 == player_2 {
                score += 6;
            } else if player_1 == player_2 {
                score += 3;
            }

            return score;
        })
        .sum::<usize>();

    return total_score;
}

/// Describes the Ideal gamestate required
/// Internal value describes the choice of the other player
/// 1 = Rock, 2 = Paper, 3 = Scissors
#[derive(Debug)]
enum IdealState {
    Win(Play),
    Draw(Play),
    Lose(Play),
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Into<IdealState> for &str {
    fn into(self) -> IdealState {
        let mut split = self.split(" ");
        let opponent_choice = match split.next().unwrap() {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Invalid RPS choice provided"),
        };
        let ideal_state = split.next().unwrap();

        let game_state = match ideal_state {
            "X" => IdealState::Lose(opponent_choice),
            "Y" => IdealState::Draw(opponent_choice),
            "Z" => IdealState::Win(opponent_choice),
            _ => panic!("Invalid Ideal Game State provided"),
        };

        return game_state;
    }
}

impl IdealState {
    fn wins_against(play: &Play) -> Play {
        use Play::*;
        match *play {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

pub fn day2_2() -> usize {
    let total_score = FILE
        .lines()
        .map(|game| {
            let mut score = 0;

            let ideal_state: IdealState = game.into();

            match ideal_state {
                IdealState::Win(opponent) => {
                    score += 6;
                    score += IdealState::wins_against(&opponent) as usize;
                }
                IdealState::Draw(opponent) => {
                    score += 3;
                    score += opponent as usize;
                }
                IdealState::Lose(opponent) => {
                    score += 0;
                    let win_condition = IdealState::wins_against(&opponent);
                    let cannot_be = [opponent, win_condition];
                    if !cannot_be.contains(&Play::Rock) {
                        score += 1;
                    } else if !cannot_be.contains(&Play::Paper) {
                        score += 2;
                    } else {
                        score += 3;
                    }
                }
            }
            return score;
        })
        .sum::<usize>();

    return total_score;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day2_1() {
        assert_eq!(super::day2_1(), 15);
    }

    #[test]
    fn test_day2_2() {
        assert_eq!(super::day2_2(), 12);
    }
}
