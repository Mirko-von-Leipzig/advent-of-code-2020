fn main() {
    println!("Day X");
    let _input_string = include_str!("../inputs/XX.txt");
    puzzle_1(_input_string);
    puzzle_2(_input_string);
}

fn puzzle_1(_input: &str) {
    println!("Running Puzzle 1\n{}", _input);
}

fn puzzle_2(_input: &str) {
    println!("Running Puzzle 2\n{}", _input);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_puzzle_1() {}

    #[test]
    fn test_puzzle_2() {}
}
