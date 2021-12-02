use super::command::{Command, Direction};

#[derive(Debug, Default, Copy, Clone)]
pub struct PositionPart1 {
    depth: usize,
    horizontal: usize,
}

impl Position for PositionPart1 {
    fn apply(&mut self, cmd: Command) {
        match cmd.direction {
            Direction::Forward => self.horizontal += cmd.value,
            Direction::Down => self.depth += cmd.value,
            Direction::Up => self.depth -= cmd.value,
        }
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn horizontal(&self) -> usize {
        self.horizontal
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct PositionPart2 {
    depth: usize,
    horizontal: usize,
    aim: usize,
}

impl Position for PositionPart2 {
    fn apply(&mut self, cmd: Command) {
        match cmd.direction {
            Direction::Forward => {
                self.horizontal += cmd.value;
                self.depth += self.aim * cmd.value;
            }
            Direction::Down => self.aim += cmd.value,
            Direction::Up => self.aim -= cmd.value,
        }
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn horizontal(&self) -> usize {
        self.horizontal
    }
}

pub trait Position {
    fn apply(&mut self, cmd: Command);

    fn depth(&self) -> usize;
    fn horizontal(&self) -> usize;

    fn applyf<P: Position>(mut pos: P, cmd: Command) -> P {
        pos.apply(cmd);
        pos
    }

    fn multiplied_value(&self) -> usize {
        self.depth() * self.horizontal()
    }
}
