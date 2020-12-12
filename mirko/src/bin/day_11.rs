// Not terribly proud of this one -- bit of a mess, but it works.

fn main() {
    let mut grid: Grid = include_str!("../../inputs/11.txt").into();

    let mut grid1 = grid.clone();
    grid1.musical_chairs_full();

    grid.musical_chairs_full_impaired();

    let part_1 = grid1.count_occupied();
    let part_2 = grid.count_occupied();

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

#[derive(Clone, PartialEq, Copy, Debug)]
enum GridState {
    Empty,
    Floor,
    Occupied,
}

#[derive(PartialEq, Clone)]
struct Grid {
    seats: Vec<GridState>,
    rows: usize,
    cols: usize,
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.at((r as isize, c as isize)).unwrap() {
                    GridState::Empty => f.write_str("L")?,
                    GridState::Floor => f.write_str(".")?,
                    GridState::Occupied => f.write_str("#")?,
                };
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Grid {
    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|&&state| state == GridState::Occupied)
            .count()
    }

    fn musical_chairs_full(&mut self) {
        while self.musical_chairs_iteration() != 0 {}
    }

    fn musical_chairs_full_impaired(&mut self) {
        while self.musical_chairs_iteration_impaired() != 0 {}
    }

    fn musical_chairs_iteration(&mut self) -> usize {
        // create new seating arrangement
        // we need to clone :( at the changes happen instantaneously
        let mut new_seats = Vec::with_capacity(self.seats.len());
        for row in 0..self.rows {
            for col in 0..self.cols {
                new_seats.push(self.next_state((row as isize, col as isize)).unwrap());
            }
        }

        // count diffs (we could do this faster by tracking changes on the fly, but this is easier to read)
        let diff = new_seats
            .iter()
            .zip(self.seats.iter())
            .filter(|(a, b)| a != b)
            .count();

        // update self
        self.seats = new_seats;

        diff
    }

    fn musical_chairs_iteration_impaired(&mut self) -> usize {
        // create new seating arrangement
        // we need to clone :( at the changes happen instantaneously
        let mut new_seats = Vec::with_capacity(self.seats.len());
        for row in 0..self.rows {
            for col in 0..self.cols {
                new_seats.push(
                    self.next_state_visibly_impaired((row as isize, col as isize))
                        .unwrap(),
                );
            }
        }

        // count diffs (we could do this faster by tracking changes on the fly, but this is easier to read)
        let diff = new_seats
            .iter()
            .zip(self.seats.iter())
            .filter(|(a, b)| a != b)
            .count();

        // update self
        self.seats = new_seats;

        diff
    }

    fn next_state(&self, position: (isize, isize)) -> Option<GridState> {
        if let Some(&current_state) = self.at(position) {
            let adjacent_occupied_count = self
                .get_adjacent(position)
                .iter()
                .filter(|&&state| state == GridState::Occupied)
                .count();

            if current_state == GridState::Empty && adjacent_occupied_count == 0 {
                Some(GridState::Occupied)
            } else if current_state == GridState::Occupied && adjacent_occupied_count >= 4 {
                Some(GridState::Empty)
            } else {
                Some(current_state)
            }
        } else {
            None
        }
    }

    fn is_visibly_occupied(&self, position: (isize, isize), direction: (isize, isize)) -> bool {
        let mut p = (
            position.0 as isize + direction.0,
            position.1 as isize + direction.1,
        );
        while let Some(&state) = self.at(p) {
            match state {
                GridState::Occupied => return true,
                GridState::Empty => return false,
                _ => {
                    p.0 += direction.0;
                    p.1 += direction.1;
                }
            }
        }

        false
    }

    fn get_adjacent(&self, position: (isize, isize)) -> Vec<GridState> {
        vec![
            (position.0 as isize - 1, position.1 as isize - 1),
            (position.0 as isize - 1, position.1 as isize),
            (position.0 as isize - 1, position.1 as isize + 1),
            (position.0 as isize, position.1 as isize - 1),
            (position.0 as isize, position.1 as isize + 1),
            (position.0 as isize + 1, position.1 as isize - 1),
            (position.0 as isize + 1, position.1 as isize),
            (position.0 as isize + 1, position.1 as isize + 1),
        ]
        .iter()
        .filter(|(r, c)| r >= &0 && c >= &0)
        .filter_map(|&p| self.at(p).copied())
        .collect()
    }

    fn visibly_occupied_count(&self, position: (isize, isize)) -> usize {
        vec![
            (0isize, 1isize), // right
            (0, -1),          // left
            (1, 0),           // up
            (-1, 0),          // down
            (1, 1),           // up-right
            (-1, -1),         // down-left
            (1, -1),          // up-left
            (-1, 1),          // down-right
        ]
        .iter()
        .filter(|&&direction| self.is_visibly_occupied(position, direction))
        .count()
    }

    fn at(&self, position: (isize, isize)) -> Option<&GridState> {
        if position.0 >= 0
            && position.0 < self.rows as isize
            && position.1 >= 0
            && position.1 < self.cols as isize
        {
            Some(&self.seats[position.1 as usize + position.0 as usize * self.cols])
        } else {
            None
        }
    }

    fn next_state_visibly_impaired(&self, position: (isize, isize)) -> Option<GridState> {
        if let Some(&current_state) = self.at(position) {
            let adjacent_occupied_count = self.visibly_occupied_count(position);

            if current_state == GridState::Empty && adjacent_occupied_count == 0 {
                Some(GridState::Occupied)
            } else if current_state == GridState::Occupied && adjacent_occupied_count >= 5 {
                Some(GridState::Empty)
            } else {
                Some(current_state)
            }
        } else {
            None
        }
    }
}

impl From<char> for GridState {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Occupied,
            'L' => Self::Empty,
            _ => Self::Floor,
        }
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let seats: Vec<GridState> = s
            .lines()
            .flat_map(|line| line.chars().map(GridState::from))
            .collect();

        let cols = s.lines().next().unwrap().chars().count();

        let rows = seats.len() / cols;

        Self { seats, rows, cols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let mut grid: Grid = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .into();

        let expect_grid: Grid = r"#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"
            .into();

        grid.musical_chairs_full();

        assert_eq!(grid, expect_grid);
        assert_eq!(grid.count_occupied(), 37);
    }

    #[test]
    fn part2_1() {
        let grid: Grid = r".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."
            .into();

        assert_eq!(grid.visibly_occupied_count((4, 3)), 8);
    }

    #[test]
    fn example_part2() {
        let mut grid: Grid = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .into();

        let expect_grid: Grid = r"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
            .into();

        grid.musical_chairs_full_impaired();
        assert_eq!(grid, expect_grid);
        assert_eq!(grid.count_occupied(), 26);
    }
}
