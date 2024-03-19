use std::cmp::Ordering::{self, *};

#[derive(PartialEq)]
pub enum Sign {
    Rock,
    Paper,
    Scissors
}


impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other{
            return Some(Equal)
        } else {
            match (self, other) {
                (Self::Rock, Self::Paper) => Some(Less),
                (Self::Paper, Self::Scissors) => Some(Less),
                (Self::Scissors, Self::Rock) => Some(Less),
                (Self::Rock, Self::Scissors) => Some(Greater),
                (Self::Paper, Self::Rock) => Some(Greater),
                (Self::Scissors, Self::Paper) => Some(Greater),
                _ => None
            }
        }
    }
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Loss,
    Tie
}

impl From<Ordering> for Outcome {
    fn from(value: Ordering) -> Self{
        match value {
            Equal => Self::Tie,
            Less => Self::Loss,
            Greater => Self::Win,
        }
    }
}

pub fn fight(a: &Sign, b: &Sign) -> Outcome {
    Sign::partial_cmp(a, b).unwrap().into()
}