use std::str::FromStr;

use miette::Diagnostic;
use thiserror::Error;

use super::parser::line;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum DirectionParseError {
    #[error("Unknown direction: {0}")]
    #[diagnostic(code(day02::direction::unknown))]
    UnknownDirection(String),
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(DirectionParseError::UnknownDirection(s.to_owned())),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub direction: Direction,
    pub value: usize,
}

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum CommandParseError {
    #[error("Remaining chars in line: {0}")]
    #[diagnostic(code(day02::command::formatting))]
    InvalidFormatting(String),
    #[error("Well, fuck...")]
    #[diagnostic(code(day02::command::fuck))]
    Other,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match line(s) {
            Ok((rem, _)) if !rem.is_empty() => {
                Err(CommandParseError::InvalidFormatting(rem.to_owned()))
            }
            Ok((_, (direction, value))) => Ok(Command { direction, value }),
            Err(_) => Err(CommandParseError::Other),
        }
    }
}
