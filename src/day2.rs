use super::*;

#[derive(Debug, Copy, Clone)]
pub enum Gesture {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
pub enum GameResults {
    Win,
    Draw,
    Loss
}

#[allow(dead_code)]
fn determine_winner(your_gesture: Gesture, opponent_gesture: Gesture) -> GameResults {
    match your_gesture {
        Gesture::Rock => {
            match opponent_gesture {
                Gesture::Rock => GameResults::Draw,
                Gesture::Paper => GameResults::Loss,
                Gesture::Scissors => GameResults::Win,
            }
        }
        Gesture::Paper => {
            match opponent_gesture {
                Gesture::Rock => GameResults::Win,
                Gesture::Paper => GameResults::Draw,
                Gesture::Scissors => GameResults::Loss,
            }
        }
        Gesture::Scissors => {
            match opponent_gesture {
                Gesture::Rock => GameResults::Loss,
                Gesture::Paper => GameResults::Win,
                Gesture::Scissors => GameResults::Draw,
            }
        }
    }
}

fn determine_gesture(opponent_gesture: Gesture, game_result: GameResults) -> Gesture {
    match opponent_gesture {
        Gesture::Rock => {
            match game_result {
                GameResults::Win => Gesture::Paper,
                GameResults::Draw => Gesture::Rock,
                GameResults::Loss => Gesture::Scissors
            }
        }
        Gesture::Paper => {
            match game_result {
                GameResults::Win => Gesture::Scissors,
                GameResults::Draw => Gesture::Paper,
                GameResults::Loss => Gesture::Rock
            }
        }
        Gesture::Scissors => {
            match game_result {
                GameResults::Win => Gesture::Rock,
                GameResults::Draw => Gesture::Scissors,
                GameResults::Loss => Gesture::Paper
            }
        }
    }
}

fn get_gesture_score(gesture: Gesture) -> u64 {
    match gesture {
        Gesture::Rock => 1,
        Gesture::Paper => 2,
        Gesture::Scissors => 3,
    }
}

fn get_result_score(result: GameResults) -> u64 {
    match result {
        GameResults::Win => 6,
        GameResults::Draw => 3,
        GameResults::Loss => 0,
    }
}

impl From<&str> for Gesture {
    fn from(char: &str) -> Self {
        match char {
            _ if char == "A" => Self::Rock,
            _ if char == "B" => Self::Paper,
            _ if char == "C" => Self::Scissors,
            _ => Self::Rock // Shouldn't happen
        }
    }
}

impl From<&str> for GameResults {
    fn from(char: &str) -> Self {
        match char {
            _ if char == "X" => Self::Loss,
            _ if char == "Y" => Self::Draw,
            _ if char == "Z" => Self::Win,
            _ => Self::Win // Shouldn't happen
        }
    }
}

pub(crate) fn rock_paper_scissors() {
    println!("===== Initiating paper scissors rock protocol");
    let path = "src/inputs/day2.txt";
    let mut total_score = 0;
    read_file(path).for_each(|line| {
        if let Ok(line) = line {
            // Vec [opponents_choice, your choice]
            let digest: Vec<&str> = line.split(' ').collect();
            let opponents_gesture = Gesture::from(digest[0]);
            let expected_result = GameResults::from(digest[1]);
            let your_gesture: Gesture = determine_gesture(opponents_gesture, expected_result);
            total_score += get_result_score(expected_result) + get_gesture_score(your_gesture);
        }
    });

    println!("Total Score: {:?}", total_score );
    // Result: 12683
}