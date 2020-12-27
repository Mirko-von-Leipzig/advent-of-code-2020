use core::panic;

// Can define enumarations that represent things other than isize using repr.
// Here we define the representation as u8, which allows as to use
// 1 byte ASCII characters as the enum
#[repr(u8)]
#[derive(Debug)]
enum Terrain {
    Open = b'.',
    Tree = b'#',
}

// Specify a Direction as simple enum
#[derive(Debug)]
enum Direction {
    DOWN,
    RIGHT,
}

struct GeographicalMap {
    layout: String,
    rank: usize, // current rank, or row
    file: usize, // current file, or column
}

impl GeographicalMap {
    fn new(layout: &str) -> GeographicalMap {
        GeographicalMap {
            layout: String::from(layout),
            rank: 0,
            file: 0,
        }
    }

    fn travel(&mut self, direction: &Direction) {
        match direction {
            Direction::DOWN => self.rank += 1,
            Direction::RIGHT => self.file += 1,
        }
    }

    fn reset(&mut self) {
        self.rank = 0;
        self.file = 0;
    }

    fn inspect_terrain(&self) -> Option<Terrain> {
        let rank = self.layout.lines().nth(self.rank)?;
        let terrain_char = rank.as_bytes().iter().cycle().nth(self.file)?;
        match *terrain_char {
            b'.' => Some(Terrain::Open),
            b'#' => Some(Terrain::Tree),
            _ => panic!("Unknown terrain type!"),
        }
    }
}

fn main() {
    println!("Day 3 Puzzle");
    let _input_string = include_str!("../inputs/03.txt");
    let mut map = GeographicalMap::new(_input_string);
    part_1(&mut map);
    map.reset();
    part_2(&mut map);
}

fn part_1(map: &mut GeographicalMap) {
    use Direction::*;

    let cheap_toboggan = vec![RIGHT, RIGHT, RIGHT, DOWN];

    let mut tree_count: usize = 0;

    loop {
        for direction in cheap_toboggan.iter() {
            map.travel(direction);
        }
        match map.inspect_terrain() {
            Some(terrain) => {
                match terrain {
                    Terrain::Tree => tree_count += 1,
                    Terrain::Open => (),
                };
            }
            None => break,
        }
    }
    println!("Finished. Tree count is: {}", tree_count);
}

fn part_2(map: &mut GeographicalMap) {
    use Direction::*;

    let slope1: Vec<Direction> = vec![RIGHT, DOWN];
    let slope2: Vec<Direction> = vec![RIGHT, RIGHT, RIGHT, DOWN];
    let slope3: Vec<Direction> = vec![RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, DOWN];
    let slope4: Vec<Direction> = vec![RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, DOWN];
    let slope5: Vec<Direction> = vec![RIGHT, DOWN, DOWN];

    let mut tree_counts = Vec::new();
    let vehicles = vec![slope1, slope2, slope3, slope4, slope5];

    for vehicle in vehicles.iter() {
        let mut tree_count: usize = 0;
        loop {
            for direction in vehicle.iter() {
                map.travel(direction);
            }
            match map.inspect_terrain() {
                Some(terrain) => {
                    match terrain {
                        Terrain::Tree => tree_count += 1,
                        Terrain::Open => (),
                    };
                }
                None => break,
            }
        }
        tree_counts.push(tree_count);
        map.reset();
    }

    println!("Finished. Tree counts are: {:?}", tree_counts);
    println!("Multiplied: {}", tree_counts.into_iter().product::<usize>());
}
