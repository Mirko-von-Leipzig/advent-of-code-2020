
// Can define enumarations that represent things other than isize
// using repr
#[repr(u8)]
enum Terrain {
    OPEN = b'.',
    TREE = b'#'
}

// Specify a Direction as simple enum
#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}
// A Vehicle holds the logic for how it moves
struct Vehicle {
    move_seqeunce: Vec<Direction>,
}

struct Map {
    rank: u32,
    file: u32
}

trait Traverse {
    fn traverse(&self, direction: &Direction) -> &Terrain;
}

fn main() {
    println!("Day 3 Puzzle");
    let input_string = include_str!("../inputs/03.txt");
    // let cheap_toboggan = Vehicle { move_seqeunce: vec![ Direction::RIGHT, Direction::RIGHT, Direction::RIGHT, Direction::DOWN ] };
    // let map = Map {file: 0, rank: 0 };
    // let driver = Driver::new(cheap_toboggan, map);
    let cheap_toboggan = vec![ Direction::RIGHT, Direction::RIGHT, Direction::RIGHT, Direction::DOWN ];
    let mut it = cheap_toboggan.iter().cycle();

    loop {
        let a = it.next().expect("OOPS");
    }
}