use crate::Part;

trait ScoreValue {
    fn val(&self) -> usize;
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        if let Some(val) = value.chars().next() {
            match val {
                'A' => Move::Rock,
                'B' => Move::Paper,
                'C' => Move::Scissors,

                'X' => Move::Rock,
                'Y' => Move::Paper,
                'Z' => Move::Scissors,
                _ => Move::Rock,
            }
        } else {
            Move::Rock
        }
    }
}

impl From<(&Move, &Outcome)> for Move {
    fn from((them, outcome): (&Move, &Outcome)) -> Self {
        match (them, outcome) {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            (Move::Paper, Outcome::Loss) => Move::Rock,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
            
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
        }
    }
}

impl ScoreValue for Move {
    fn val(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        if let Some(val) = value.chars().next() {
            match val {
                'X' => Outcome::Loss,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => Outcome::Draw,
            }
        } else {
            Outcome::Draw
        }
    }
}

impl From<(&Move, &Move)> for Outcome {
    fn from((us, them): (&Move, &Move)) -> Self {
        match (us, them) {
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }
}

impl ScoreValue for Outcome {
    fn val(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

pub(crate) fn solve(mut input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let mut total_score:usize = 0;

    while let Some(line) = input.next() {
        if let Some((left, right)) = line.split_once(' ') {
            let opponent_move:Move = left.into();
            
            total_score += match part {
                Part::Part1 => {
                    let our_move:Move = right.into();
                    calc_score(&our_move, &opponent_move)
                },
                Part::Part2 => {
                    let outcome:Outcome = right.into();
                    calc_score_from(&opponent_move, &outcome)
                }
            };
        }
    }

    format!("{}", total_score)
}

fn calc_score(us: &Move, them: &Move) -> usize {
    let outcome: Outcome =  (us, them).into();

    us.val() + outcome.val()
}

fn calc_score_from(them: &Move, outcome: &Outcome) -> usize {
    let our_move: Move = (them, outcome).into();

    our_move.val() + outcome.val()
}



#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"A Y
B X
C Z";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "15");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "12");
}
