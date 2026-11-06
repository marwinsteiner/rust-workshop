#[derive(Clone, Copy, Debug, PartialEq)]
enum Move { Rock, Paper, Scissors, Lizard }
use Move::*;

const ALL: [Move; 4] = [Rock, Paper, Scissors, Lizard];

/// The game rules: does `a` beat `b`?
fn beats(a: Move, b: Move) -> bool {
    matches!(
        (a, b),
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper)
            | (Lizard, Paper) | (Lizard, Rock) | (Scissors, Lizard)
    )
}

/// A move that beats `m` (the first one in `ALL`).
fn counter(m: Move) -> Move {
    *ALL.iter().find(|&&c| beats(c, m)).expect("every move must be beatable")
}

/// `m` moved `k` places along `ALL` (wrapping around).
fn shift(m: Move, k: usize) -> Move {
    let i = ALL.iter().position(|&x| x == m).unwrap();
    ALL[(i + k) % ALL.len()]
}

/// +1 if the player wins, -1 if the bot wins, 0 for a draw.
fn round_score(player: Move, bot: Move) -> i32 {
    if beats(player, bot) {
        1
    } else if beats(bot, player) {
        -1
    } else {
        0
    }
}

fn parse_move(input: &str) -> Option<Move> {
    match input.trim().to_lowercase().as_str() {
        "r" | "rock" => Some(Rock),
        "p" | "paper" => Some(Paper),
        "s" | "scissors" => Some(Scissors),
        "l" | "lizard" => Some(Lizard),
        _ => None,
    }
}

/// A guess about the human's next move, based only on previous rounds.
#[derive(Clone, Copy, Debug)]
enum Predictor {
    /// Human plays their own last move shifted by k (k = 0: always the same move).
    ShiftOwn(usize),
    /// Human plays what beats their own last move.
    CounterOwn,
    /// Human plays the bot's last move shifted by k (k = 0: copies the bot).
    ShiftBot(usize),
    /// Human plays what beats the bot's last move.
    CounterBot,
    /// Human repeats a winning move, otherwise plays what beats the bot's last move.
    WinStayLoseShift,
    /// Human repeats the move from p rounds ago (a pattern of length p).
    Period(usize),
}

impl Predictor {
    fn predict(self, human: &[Move], bot: &[Move]) -> Option<Move> {
        let (h, b) = (human.last().copied(), bot.last().copied());
        match self {
            Predictor::ShiftOwn(k) => h.map(|m| shift(m, k)),
            Predictor::CounterOwn => h.map(counter),
            Predictor::ShiftBot(k) => b.map(|m| shift(m, k)),
            Predictor::CounterBot => b.map(counter),
            Predictor::WinStayLoseShift => {
                let (h, b) = (h?, b?);
                Some(if beats(h, b) { h } else { counter(b) })
            }
            Predictor::Period(p) => human.len().checked_sub(p).map(|i| human[i]),
        }
    }
}

/// How many recent rounds the bot learns from.
const WINDOW: usize = 10;
/// Longest repeating pattern the bot looks for.
const MAX_PERIOD: usize = 6;
/// The bot's moves in rounds 1-3. They don't need to win: they are chosen so the
/// human's answers reveal their pattern before round 4. Found by the `retrain` test.
const OPENING: [Move; 3] = [Rock, Scissors, Scissors];

struct Bot {
    human_moves: Vec<Move>,
    bot_moves: Vec<Move>,
    predictors: Vec<Predictor>,
    opening: Vec<Move>,
}

impl Bot {
    fn new() -> Self {
        Self::with_opening(&OPENING)
    }

    fn with_opening(opening: &[Move]) -> Self {
        let mut predictors = Vec::new();
        predictors.extend((0..ALL.len()).map(Predictor::ShiftOwn));
        predictors.push(Predictor::CounterOwn);
        predictors.extend((0..ALL.len()).map(Predictor::ShiftBot));
        predictors.push(Predictor::CounterBot);
        predictors.push(Predictor::WinStayLoseShift);
        predictors.extend((2..=MAX_PERIOD).map(Predictor::Period));
        Bot {
            human_moves: Vec::new(),
            bot_moves: Vec::new(),
            predictors,
            opening: opening.to_vec(),
        }
    }

    /// Picks the bot's move. It only sees previous rounds, never the current input.
    ///
    /// Training: every predictor is replayed over the recent rounds and scored.
    /// Predictors with the fewest wrong guesses vote for the move they expect,
    /// weighted by how many guesses they got right. The bot plays the move that
    /// beats the most votes.
    fn choose(&self) -> Move {
        let n = self.human_moves.len();
        if n < self.opening.len() {
            return self.opening[n];
        }
        let scored: Vec<(usize, usize, Move)> = self
            .predictors
            .iter()
            .filter_map(|p| {
                let guess = p.predict(&self.human_moves, &self.bot_moves)?;
                let (mut right, mut wrong) = (0, 0);
                for i in n.saturating_sub(WINDOW)..n {
                    match p.predict(&self.human_moves[..i], &self.bot_moves[..i]) {
                        Some(g) if g == self.human_moves[i] => right += 1,
                        Some(_) => wrong += 1,
                        None => {}
                    }
                }
                Some((wrong, right, guess))
            })
            .collect();
        let Some(fewest_wrong) = scored.iter().map(|&(w, _, _)| w).min() else {
            return Paper; // no history: many people open with Rock
        };
        let votes = |c: Move| -> usize {
            scored
                .iter()
                .filter(|&&(w, _, g)| w == fewest_wrong && beats(c, g))
                .map(|&(_, r, _)| r)
                .sum()
        };
        // max_by_key keeps the last maximum, so iterate in reverse to prefer ALL's order on ties
        *ALL.iter().rev().max_by_key(|&&c| votes(c)).unwrap()
    }

    fn record(&mut self, human: Move, bot: Move) {
        self.human_moves.push(human);
        self.bot_moves.push(bot);
    }
}

fn main() -> std::io::Result<()> {
    let mut score = 0;
    let mut bot = Bot::new();
    loop {
        // The bot commits to its move before reading the human's input.
        let bot_move = bot.choose();
        println!("r / p / s / l, or q to quit: ");
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
                println!("Please enter rock, paper, scissors, or lizard.");
                continue;
            }
        };
        bot.record(player, bot_move);
        let result = round_score(player, bot_move);
        score += result;
        let outcome = match result {
            1 => "You win",
            -1 => "Bot wins",
            _ => "Draw",
        };
        println!("Bot played {bot_move:?}. {outcome}. Score: {score}");
    }
    println!("Final score: {score}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    type Human = Box<dyn Fn(&[Move], &[Move]) -> Move>;

    /// Scripted "humans" that follow a pattern, each tried with every starting move.
    fn humans() -> Vec<(String, Human)> {
        let mut hs: Vec<(String, Human)> = Vec::new();
        for &m in &ALL {
            for k in 0..ALL.len() {
                hs.push((
                    format!("own last +{k}, start {m:?}"),
                    Box::new(move |h: &[Move], _: &[Move]| h.last().map_or(m, |&x| shift(x, k))),
                ));
                hs.push((
                    format!("bot last +{k}, start {m:?}"),
                    Box::new(move |_: &[Move], b: &[Move]| b.last().map_or(m, |&x| shift(x, k))),
                ));
            }
            hs.push((
                format!("beat own last, start {m:?}"),
                Box::new(move |h: &[Move], _: &[Move]| h.last().map_or(m, |&x| counter(x))),
            ));
            hs.push((
                format!("beat bot last, start {m:?}"),
                Box::new(move |_: &[Move], b: &[Move]| b.last().map_or(m, |&x| counter(x))),
            ));
            hs.push((
                format!("win-stay lose-shift, start {m:?}"),
                Box::new(move |h: &[Move], b: &[Move]| match (h.last(), b.last()) {
                    (Some(&x), Some(&y)) if beats(x, y) => x,
                    (Some(_), Some(&y)) => counter(y),
                    _ => m,
                }),
            ));
            for &o in &ALL {
                if o != m {
                    hs.push((
                        format!("alternate {m:?}/{o:?}"),
                        Box::new(move |h: &[Move], _: &[Move]| if h.len() % 2 == 0 { m } else { o }),
                    ));
                }
            }
        }
        hs
    }

    #[test]
    fn every_move_can_be_beaten() {
        for &m in &ALL {
            assert!(beats(counter(m), m), "{m:?}");
        }
    }

    /// Plays the bot against every pattern; returns the patterns it fails to beat
    /// in every round from round 4 on.
    fn failures(opening: &[Move], hs: &[(String, Human)]) -> Vec<String> {
        let mut failures = Vec::new();
        for (name, human) in hs {
            let mut bot = Bot::with_opening(opening);
            for round in 1..=50 {
                let b = bot.choose();
                let h = human(&bot.human_moves, &bot.bot_moves);
                if round >= 4 && !beats(b, h) {
                    failures.push(format!("{name}: round {round}, human {h:?}, bot {b:?}"));
                    break;
                }
                bot.record(h, b);
            }
        }
        failures
    }

    /// Retraining: tries every possible opening and lists the ones that beat
    /// all patterns from round 4. Run with:
    /// cargo test retrain -- --ignored --nocapture
    #[test]
    #[ignore]
    fn retrain() {
        let hs = humans();
        let mut best = (usize::MAX, Vec::new());
        for &a in &ALL {
            for &b in &ALL {
                for &c in &ALL {
                    let opening = [a, b, c];
                    let fl = failures(&opening, &hs);
                    let f = fl.len();
                    if f <= 3 { println!("{opening:?}: {f} -> {fl:?}"); }
                    if f == 0 {
                        println!("perfect opening: {opening:?}");
                    }
                    if f < best.0 {
                        best = (f, opening.to_vec());
                    }
                }
            }
        }
        println!("best opening {:?} with {} failures", best.1, best.0);
    }

    #[test]
    fn bot_always_wins_from_round_4() {
        let hs = humans();
        let failures = failures(&OPENING, &hs);
        assert!(
            failures.is_empty(),
            "{} of {} patterns not beaten from round 4:\n{}",
            failures.len(),
            hs.len(),
            failures.join("\n")
        );
    }
}
