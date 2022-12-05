#[derive(Debug)]
struct Round {
    opponent_move: Move,
    your_move: Move,
}

impl Round {
    fn parse_part1(input: &str) -> Self {
        let mut moves = input.split(' ').map(Move::parse_part1);

        let opponent_move = moves.next().unwrap();
        let your_move = moves.next().unwrap();

        Round {
            opponent_move,
            your_move,
        }
    }

    fn parse_part2(input: &str) -> Self {
        let mut split = input.split(' ');

        let opponent_move = split.next().unwrap();
        let opponent_move = Move::parse_part1(opponent_move);

        let end_result = split.next().unwrap();
        let end_result = Result::parse(end_result);

        let your_move = opponent_move.your_move_for_result(end_result);

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

enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn parse(input: &str) -> Self {
        match input {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Unknown result character"),
        }
    }
}

impl Move {
    fn parse_part1(input: &str) -> Self {
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

    /// This method takes in the intended result, and returns the move you
    /// have to play to get the Result in the round
    fn your_move_for_result(&self, intended_result: Result) -> Self {
        match (self, intended_result) {
            (Move::Rock, Result::Draw) => Move::Rock,
            (Move::Rock, Result::Win) => Move::Paper,
            (Move::Rock, Result::Lose) => Move::Scissors,

            (Move::Paper, Result::Draw) => Move::Paper,
            (Move::Paper, Result::Win) => Move::Scissors,
            (Move::Paper, Result::Lose) => Move::Rock,

            (Move::Scissors, Result::Draw) => Move::Scissors,
            (Move::Scissors, Result::Win) => Move::Rock,
            (Move::Scissors, Result::Lose) => Move::Paper,
        }
    }
}

fn part_1(input: &str) -> u64 {
    let rounds: Vec<Round> = input.lines().map(Round::parse_part1).collect();

    rounds.iter().map(|r| r.score()).sum()
}

fn part_2(input: &str) -> u64 {
    let rounds: Vec<Round> = input.lines().map(Round::parse_part2).collect();

    rounds.iter().map(|r| r.score()).sum()
}
fn main() {
    let example_input = include_str!("example.input");
    let my_input = include_str!("my.input");

    let example_part1_ans = part_1(example_input);
    let example_part2_ans = part_2(example_input);
    dbg!(example_part1_ans, example_part2_ans);

    let my_part1_ans = part_1(my_input);
    let my_part2_ans = part_2(my_input);
    dbg!(my_part1_ans, my_part2_ans);
}
