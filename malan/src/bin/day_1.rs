use std::{num::ParseIntError, fs};

const YEAR: u32 = 2020;

fn main() {

    // Read in the file contents
    let file_string = fs::read_to_string("../inputs/01.txt").expect("file open failed");

    // Split string into lines, and map each substring to u32. Collect into vec
    let entries= file_string
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>().expect("string parsing error");

    if let Some(pair) = find_pair(&entries, YEAR) {
        println!("{}", pair.0*pair.1);
    } else {
        println!("No matching entries found")
    };

    if let Some(answer) = find_triple_answer(&entries, YEAR) {
        println!("{}", answer);
    } else {
        println!("No answer found for part 2.");
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
            .find_map(|(i, &v)| {
                find_pair(&entries[i + 1..], target - v).map(|(a, b)| a * b * v)
            })
}