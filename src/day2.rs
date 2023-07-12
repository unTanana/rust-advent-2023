use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;

#[derive(PartialEq, Eq)]
enum Sign {
    Rock(usize),
    Paper(usize),
    Scissors(usize),
}

trait Opposite {
    fn get_opposite(input: &Sign, outcome: &str) -> Result<Sign, anyhow::Error>;
}

impl FromStr for Sign {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Sign::Rock(1)),
            "B" => Ok(Sign::Paper(2)),
            "C" => Ok(Sign::Scissors(3)),
            "X" => Ok(Sign::Rock(1)),
            "Y" => Ok(Sign::Paper(2)),
            "Z" => Ok(Sign::Scissors(3)),
            _ => Err(anyhow!("Bad input")),
        }
    }
}

impl Opposite for Sign {
    fn get_opposite(input: &Sign, outcome: &str) -> Result<Sign, anyhow::Error> {
        match outcome {
            // lose
            "X" => match input {
                Sign::Rock(_) => Ok(Sign::Scissors(3)),
                Sign::Paper(_) => Ok(Sign::Rock(1)),
                Sign::Scissors(_) => Ok(Sign::Paper(2)),
            },
            // draw
            "Y" => match input {
                Sign::Rock(_) => Ok(Sign::Rock(1)),
                Sign::Paper(_) => Ok(Sign::Paper(2)),
                Sign::Scissors(_) => Ok(Sign::Scissors(3)),
            },
            // win
            "Z" => match input {
                Sign::Rock(_) => Ok(Sign::Paper(2)),
                Sign::Paper(_) => Ok(Sign::Scissors(3)),
                Sign::Scissors(_) => Ok(Sign::Rock(1)),
            },
            _ => Err(anyhow!("Bad input")),
        }
    }
}

struct Game {
    p1: Sign,
    p2: Sign,
}

trait Play {
    fn play(&self) -> usize;
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1_str, p2_str) = s.split_once(' ').expect("MUST WORK");

        let p1 = p1_str.parse()?;
        let p2 = Sign::get_opposite(&p1, p2_str)?;

        Ok(Game { p1, p2 })
    }
}
impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sign::Rock(_) => "R1",
                Sign::Paper(_) => "P2",
                Sign::Scissors(_) => "S3",
            }
        )
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} X {} = {}", self.p1, self.p2, self.play())
    }
}

const WIN: usize = 6;
const DRAW: usize = 3;
const LOSS: usize = 0;

impl Play for Game {
    fn play(&self) -> usize {
        match (&self.p1, &self.p2) {
            (Sign::Rock(_), Sign::Scissors(a)) => a + LOSS,
            (Sign::Paper(_), Sign::Rock(a)) => a + LOSS,
            (Sign::Scissors(_), Sign::Paper(a)) => a + LOSS,
            (Sign::Rock(_), Sign::Rock(a)) => a + DRAW,
            (Sign::Paper(_), Sign::Paper(a)) => a + DRAW,
            (Sign::Scissors(_), Sign::Scissors(a)) => a + DRAW,
            (Sign::Rock(_), Sign::Paper(a)) => a + WIN,
            (Sign::Paper(_), Sign::Scissors(a)) => a + WIN,
            (Sign::Scissors(_), Sign::Rock(a)) => a + WIN,
        }
    }
}

pub fn day2() {
    let file = std::fs::read_to_string("rock_paper_scissors").expect("File must exist");
    let total: usize = file
        .lines()
        .map(|line| {
            let game = line.parse::<Game>().expect("must parse game");
            game.play()
        })
        .sum();

    println!("{}", total);
}
