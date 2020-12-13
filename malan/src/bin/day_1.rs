use std::num::ParseIntError;

const YEAR: u32 = 2020;

fn main() {
    println!("Day 1");
    let _input_string = include_str!("../inputs/01.txt");
    puzzle_1(_input_string);
    puzzle_2(_input_string);
}

fn puzzle_1(input: &str) {

    println!("Running Puzzle 1");

    // Split string into lines, and map each substring to u32. Collect into vec
    let entries = input
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .expect("string parsing error");

    if let Some(pair) = find_pair(&entries, YEAR) {
        println!("{}", pair.0 * pair.1);
    } else {
        println!("No matching entries found")
    };
}

fn puzzle_2(input: &str) {
    println!("Running Puzzle 2");

        // Split string into lines, and map each substring to u32. Collect into vec
    let entries = input
    .lines()
    .map(|line| line.parse::<u32>())
    .collect::<Result<Vec<u32>, ParseIntError>>()
    .expect("string parsing error");

    if let Some(answer) = find_triple_answer(&entries, YEAR) {
        println!("{}", answer);
    } else {
        println!("No answer found for part 2.");
    }
}

#[cfg(test)]
mod tests {
    use crate::YEAR;

    #[test]
    fn test_puzzle_1() {
        let input = vec![1721, 979 ,366, 299, 675, 1456];
        let expected = 514579;

        let response = super::find_pair(&input, YEAR).expect("Function failed");
        assert!(expected == response.0 * response.1);
    }

    #[test]
    fn test_puzzle_2() {
        let input = vec![1721, 979 ,366, 299, 675, 1456];
        let expected = 241861950;

        let response = super::find_triple_answer(&input, YEAR).expect("Function failed");
        assert!(expected == response);
    }
}

// From Mirko
fn find_pair(entries: &[u32], target: u32) -> Option<(u32, u32)> {
    for entry in entries {
        if *entry <= target && entries.contains(&(target - entry)) {
            return Some((*entry, target - entry));
        }
    }
    None
}

// From Mirko
fn find_triple_answer(entries: &[u32], target: u32) -> Option<u32> {
    entries
        .iter()
        .enumerate()
        .filter(|(i, &v)| v <= target && i + 2 < entries.len())
        .find_map(|(i, &v)| find_pair(&entries[i + 1..], target - v).map(|(a, b)| a * b * v))
}
