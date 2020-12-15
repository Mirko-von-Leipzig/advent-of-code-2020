#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Turn {
    Left,
    Right,
    Around,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn turn(&mut self, t: Turn) {
        *self = match self {
            Direction::Up => match t {
                Turn::Right => Self::Right,
                Turn::Left => Self::Left,
                Turn::Around => Self::Down,
            },
            Direction::Down => match t {
                Turn::Right => Self::Left,
                Turn::Left => Self::Right,
                Turn::Around => Self::Up,
            },
            Direction::Right => match t {
                Turn::Right => Self::Down,
                Turn::Left => Self::Up,
                Turn::Around => Self::Left,
            },
            Direction::Left => match t {
                Turn::Right => Self::Up,
                Turn::Left => Self::Down,
                Turn::Around => Self::Right,
            },
        }
    }
}
