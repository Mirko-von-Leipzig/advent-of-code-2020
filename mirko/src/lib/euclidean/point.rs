#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn move_in_direction(&mut self, direction: super::direction::Direction, amount: u32) {
        use super::direction::Direction::*;

        match direction {
            Up => self.y += amount as i64,
            Down => self.y -= amount as i64,
            Right => self.x += amount as i64,
            Left => self.x -= amount as i64,
        }
    }

    pub fn rotate(&mut self, turn: super::direction::Turn) {
        use super::direction::Turn::*;

        let (x, y) = (self.x, self.y);
        match turn {
            Left => {
                self.x = -y;
                self.y = x;
            }
            Right => {
                self.x = y;
                self.y = -x;
            }
            Around => {
                self.x = -x;
                self.y = -y;
            }
        }
    }

    pub fn manhattan_distance(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }

    pub fn scale_by(&self, factor: u32) -> Self {
        Self::new(self.x * factor as i64, self.y * factor as i64)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let mut a = Point::new(0, 10);
        let b = Point::new(10, 10);

        let c = a + b;
        a += b;

        assert_eq!(c.x, 10, "x addition failed");
        assert_eq!(c.y, 20, "y addition failed");
        assert_eq!(c, a, "+= failed");
    }

    #[test]
    fn scaling() {
        let a = Point::new(-4, 13);

        assert_eq!(a.scale_by(3), Point::new(-12, 39));
    }

    #[test]
    fn moving() {
        let mut a = Point::origin();

        a.move_in_direction(super::super::Direction::Left, 10);
        a.move_in_direction(super::super::Direction::Up, 10);

        assert_eq!(a, Point::new(-10, 10));
    }

    #[test]
    fn rotation() {
        use super::super::Turn::*;
        let mut a = Point::new(1, 2);
        a.rotate(Left);
        assert_eq!(a, Point::new(-2, 1));
        a.rotate(Around);
        assert_eq!(a, Point::new(2, -1));
    }
}
