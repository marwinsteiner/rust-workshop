#[derive(Clone, Copy, Debug, PartialEq)]

enum Move { Rock, Paper, Scissors}
use Move::*;

fn main() {
    for m in [Rock, Paper, Scissors] {
        println!("{m:?}");
    }
}