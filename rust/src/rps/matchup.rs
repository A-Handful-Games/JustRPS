use std::cmp::Ordering::{self, *};
<<<<<<< HEAD
use godot::builtin::meta::{ConvertError, GodotConvert, ToGodot, FromGodot};
=======
>>>>>>> 18f365ea5c4862e329cf309168e9d4722880eb6c

#[derive(Debug, PartialEq)]
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

impl GodotConvert for Sign {
    type Via = i8;
}

impl ToGodot for Sign {
    fn to_godot(&self) -> Self::Via {
        match self {
            Self::Rock => 0,
            Self::Paper => 1,
            Self::Scissors => 2,
            _ => -1
        }
    }
}

impl FromGodot for Sign {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        match via {
            0 => Ok(Self::Rock),
            1 => Ok(Self::Paper),
            2 => Ok(Self::Scissors),
            _ => Err(ConvertError::new("Failed to create sign from Godot"))
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