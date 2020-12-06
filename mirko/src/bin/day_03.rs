fn main() {
    let map: Map = include_str!("../../inputs/03.txt").parse().unwrap();

    let counts = vec![
        map.check_slope(1, 1),
        map.check_slope(3, 1),
        map.check_slope(5, 1),
        map.check_slope(7, 1),
        map.check_slope(1, 2),
    ];

    let tree_product: usize = counts.iter().product();

    for (i, count) in counts.iter().enumerate() {
        println!("slope {}: {}", i + 1, count);
    }

    println!("For a total product of {}", tree_product);
}

#[derive(std::cmp::PartialEq)]
enum Vegetation {
    Tree,
    Open,
}

struct Map {
    grid: Vec<Vegetation>,
    width: usize,
    height: usize,
}

impl Map {
    fn check_slope(&self, right: usize, down: usize) -> usize {
        let mut col = 0;
        let mut count = 0;

        for row in (0..self.height).step_by(down) {
            if self.grid[col + row * self.width] == Vegetation::Tree {
                count += 1;
            }

            col += right;
            if col >= self.width {
                col -= self.width;
            }
        }

        count
    }
}

impl std::str::FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = 0;

        let grid = s
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Vegetation::Open),
                '#' => Some(Vegetation::Tree),
                '\n' => {
                    height += 1;
                    None
                }
                '\r' => None,
                _ => panic!("cannot map {}", c as u32),
            })
            .collect::<Vec<Vegetation>>();

        let width = grid.len() / height;

        Ok(Map {
            grid,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let uut: Map = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"
        .parse()
        .unwrap();

        assert_eq!(uut.check_slope(1, 1), 2);
        assert_eq!(uut.check_slope(3, 1), 7);
        assert_eq!(uut.check_slope(5, 1), 3);
        assert_eq!(uut.check_slope(7, 1), 4);
        assert_eq!(uut.check_slope(1, 2), 2);
    }
}
