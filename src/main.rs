#[derive(Clone, Copy, Debug, PartialEq)]

enum Move { Rock, Paper, Scissors, Lizard}
use Move::*;

fn counter(m: Move) -> Move {
    match m {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
        _ => {}
    }
}

fn round_score(player: Move, bot: Move) -> i32 {
    if player == bot {
        0
    } else if player == counter(bot) {
        1
    } else {
        -1
    }
}

fn parse_move(input: &str) -> Option<Move> {
    match input.trim().to_lowercase().as_str() {
        "r" | "rock" => return Some(Rock),
        "p" | "paper" => return Some(Paper),
        "s" | "scissors" => return Some(Scissors),
        _ => None
    }
}

fn main() -> std::io::Result<()> {
    let mut score = 0;
    loop {
        let bot = Rock;
        println!("r / p / s, or q to quit: ");
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        if input.trim().eq_ignore_ascii_case("q") {
            break;
        }
        let player = match parse_move(&input) {
            Some(m) => m,
            None => {
                println!("Please enter rock, paper, or scissors.");
                continue;
            }
        };
        let result = round_score(player, bot);
        score += result;
        println!("Score: {}", score);
    }
    println!("Final score: {score}");
}