fn main() {
    let raw_passes = include_str!("../../inputs/05.txt");

    let mut seat_ids: Vec<u16> = raw_passes
        .lines()
        .map(|line| {
            let boarding_pass: BoardingPass = line.parse().unwrap();

            boarding_pass.id
        })
        .collect();

    seat_ids.sort_unstable();

    let max_id = seat_ids.last().unwrap();

    println!("maximum seat ID: {}", max_id);

    let i1 = seat_ids.iter();
    let i2 = i1.clone().skip(1);

    let (id1, id2) = i1
        .zip(i2)
        .find(|(&first, &second)| first + 2 == second)
        .unwrap();

    println!(
        "hopefully between seats {} and {} i.e. {}",
        id1,
        id2,
        id2 - 1
    );
}

#[allow(dead_code)]
struct BoardingPass {
    row: u16,
    col: u16,
    id: u16,
}

impl std::str::FromStr for BoardingPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row_str, col_str) = s.split_at(7);

        let mut row = 0;
        let mut n = 64;

        for r in row_str.chars() {
            if r == 'B' {
                row += n;
            }

            n /= 2;
        }

        let mut col = 0;
        let mut n = 4;

        for c in col_str.chars() {
            if c == 'R' {
                col += n;
            }

            n /= 2;
        }

        let id = row * 8 + col;

        Ok(BoardingPass { row, col, id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let uut: BoardingPass = "BFFFBBFRRR".parse().unwrap();
        assert_eq!(uut.row, 70);
        assert_eq!(uut.col, 7);
        assert_eq!(uut.id, 567);
    }

    #[test]
    fn example_2() {
        let uut: BoardingPass = "FFFBBBFRRR".parse().unwrap();
        assert_eq!(uut.row, 14);
        assert_eq!(uut.col, 7);
        assert_eq!(uut.id, 119);
    }

    #[test]
    fn example_3() {
        let uut: BoardingPass = "BBFFBBFRLL".parse().unwrap();
        assert_eq!(uut.row, 102);
        assert_eq!(uut.col, 4);
        assert_eq!(uut.id, 820);
    }
}
