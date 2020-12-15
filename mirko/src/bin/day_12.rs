use mirko::euclidean::*;
use std::convert::TryFrom;

fn main() {
    let mut ship_1 = Ship::new();
    let mut ship_2 = WaypointShip::new();

    include_str!("../../inputs/12.txt")
        .lines()
        .map(|line| Instruction::try_from(line).unwrap())
        .for_each(|instruction| {
            ship_1.handle_instruction(instruction);
            ship_2.handle_instruction(instruction);
        });

    println!("part 1: {}", ship_1.manhattan_distance());
    println!("part 2: {}", ship_2.manhattan_distance());
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward(u32),
    Move(Direction, u32),
    Turn(Turn),
}

#[derive(Debug, Clone, PartialEq)]
struct Ship {
    position: Point,
    facing: Direction,
}

#[derive(Debug, Clone, PartialEq)]
struct WaypointShip {
    ship: Point,
    waypoint: Point,
}

impl WaypointShip {
    fn new() -> Self {
        Self {
            ship: Point::origin(),
            waypoint: Point::new(10, 1),
        }
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(distance) => self.ship += self.waypoint.scale_by(distance),
            Instruction::Move(direction, distance) => {
                self.waypoint.move_in_direction(direction, distance)
            }
            Instruction::Turn(direction) => self.waypoint.rotate(direction),
        }
    }

    fn manhattan_distance(&self) -> u64 {
        self.ship.manhattan_distance()
    }
}

impl Ship {
    fn new() -> Self {
        Self {
            position: Point::origin(),
            facing: Direction::Right,
        }
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(distance) => {
                self.position.move_in_direction(self.facing, distance)
            }
            Instruction::Move(direction, distance) => {
                self.position.move_in_direction(direction, distance)
            }
            Instruction::Turn(turn) => self.facing.turn(turn),
        }
    }

    fn manhattan_distance(&self) -> u64 {
        self.position.manhattan_distance()
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let letter = chars.next().unwrap();
        let amount = chars.as_str().parse::<u32>().unwrap();

        match letter {
            'F' => Instruction::Forward(amount),
            'N' => Instruction::Move(Direction::Up, amount),
            'S' => Instruction::Move(Direction::Down, amount),
            'E' => Instruction::Move(Direction::Right, amount),
            'W' => Instruction::Move(Direction::Left, amount),
            'R' => match amount {
                90 => Instruction::Turn(Turn::Right),
                180 => Instruction::Turn(Turn::Around),
                270 => Instruction::Turn(Turn::Left),
                _ => panic!("unknown right amount: {}", amount),
            },
            _ => match amount {
                90 => Instruction::Turn(Turn::Left),
                180 => Instruction::Turn(Turn::Around),
                270 => Instruction::Turn(Turn::Right),
                _ => panic!("unknown left amount: {}", amount),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ship() {
        let mut ship = Ship::new();

        ship.handle_instruction("F10".into());
        assert_eq!(
            ship,
            Ship {
                position: Point::new(10, 0),
                facing: Direction::Right,
            }
        );

        ship.handle_instruction("N3".into());
        assert_eq!(
            ship,
            Ship {
                position: Point::new(10, 3),
                facing: Direction::Right,
            }
        );

        ship.handle_instruction("F7".into());
        assert_eq!(
            ship,
            Ship {
                position: Point::new(17, 3),
                facing: Direction::Right,
            }
        );

        ship.handle_instruction("R90".into());
        assert_eq!(
            ship,
            Ship {
                position: Point::new(17, 3),
                facing: Direction::Down,
            }
        );

        ship.handle_instruction("F11".into());
        assert_eq!(
            ship,
            Ship {
                position: Point::new(17, -8),
                facing: Direction::Down,
            }
        );

        assert_eq!(ship.manhattan_distance(), 25);
    }

    #[test]
    fn example_waypoint() {
        let mut ship = WaypointShip::new();

        ship.handle_instruction("F10".into());
        assert_eq!(
            ship,
            WaypointShip {
                ship: Point::new(100, 10),
                waypoint: Point::new(10, 1),
            }
        );

        ship.handle_instruction("N3".into());
        assert_eq!(
            ship,
            WaypointShip {
                ship: Point::new(100, 10),
                waypoint: Point::new(10, 4),
            }
        );

        ship.handle_instruction("F7".into());
        assert_eq!(
            ship,
            WaypointShip {
                ship: Point::new(170, 38),
                waypoint: Point::new(10, 4),
            }
        );

        ship.handle_instruction("R90".into());
        assert_eq!(
            ship,
            WaypointShip {
                ship: Point::new(170, 38),
                waypoint: Point::new(4, -10),
            }
        );

        ship.handle_instruction("F11".into());
        assert_eq!(
            ship,
            WaypointShip {
                ship: Point::new(214, -72),
                waypoint: Point::new(4, -10),
            }
        );

        assert_eq!(ship.manhattan_distance(), 286);
    }
}
