#[derive(Clone, Copy, Debug, PartialEq)]
enum Move { Rock, Paper, Scissors, }
use Move::*;

fn counter(m: Move) -> Move {
    match m {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
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
        "r" | "rock" => Some(Rock),
        "p" | "paper" => Some(Paper),
        "s" | "scissors" => Some(Scissors),
        _ => None,
    }
}

fn choose_bot(history: &[Move]) -> Move {
    if history.is_empty() {
        return Rock;
    }
    let mut counts = [0; 3];
    for m in history {
        match m {
            Rock => counts[0] += 1,
            Paper => counts[1] += 1,
            Scissors => counts[2] += 1,
        }
    }
    let predicted = if counts[0] >= counts[1] && counts[0] >= counts[2] {
        Rock
    } else if counts[1] >= counts[2] {
        Paper
    } else {
        Scissors
    };
    counter(predicted)
}

fn main() -> std::io::Result<()> {
    let mut score = 0;
    let mut history = vec![Rock; 10];
    loop {
        let bot = choose_bot(&history);
        println!("r / p / s, or q to quit:");
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
                println!("Please enter rock, paper or scissors.");
                continue;
            }
        };
        history.push(player);
        if history.len() > 10 {
            history.remove(0);
        }
        let result = round_score(player, bot);
        score += result;
        println!("You: {player:?}; bot: {bot:?}; round: {result:+}; score: {score}");
    }
    println!("Final score: {score}");
    Ok(())
}