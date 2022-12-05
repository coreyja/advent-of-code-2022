#[derive(Debug)]
struct Round {
    opponent_move: Move,
    your_move: Move,
}

impl Round {
    fn parse(input: &str) -> Self {
        let mut moves = input.split(' ').map(Move::parse);

        let opponent_move = moves.next().unwrap();
        let your_move = moves.next().unwrap();

        Round {
            opponent_move,
            your_move,
        }
    }

    fn outcome_score(&self) -> u64 {
        self.your_move.outcome_score(&self.opponent_move)
    }

    fn score(&self) -> u64 {
        self.outcome_score() + self.your_move.score()
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse(input: &str) -> Self {
        match input {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Unknown move character"),
        }
    }

    fn score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome_score(&self, other: &Self) -> u64 {
        match (self, other) {
            (Move::Rock, Move::Scissors) => 6,
            (Move::Paper, Move::Rock) => 6,
            (Move::Scissors, Move::Paper) => 6,
            (Move::Rock, Move::Paper) => 0,
            (Move::Paper, Move::Scissors) => 0,
            (Move::Scissors, Move::Rock) => 0,
            (Move::Scissors, Move::Scissors) => 3,
            (Move::Rock, Move::Rock) => 3,
            (Move::Paper, Move::Paper) => 3,
        }
    }
}

fn part_1(input: &str) -> u64 {
    let rounds: Vec<Round> = input.lines().map(Round::parse).collect();

    rounds.iter().map(|r| r.score()).sum()
}
fn main() {
    let example_input = include_str!("example.input");
    let my_input = include_str!("my.input");

    let example_part1_ans = part_1(example_input);
    dbg!(example_part1_ans);

    let my_part1_ans = part_1(my_input);
    dbg!(my_part1_ans);
}
