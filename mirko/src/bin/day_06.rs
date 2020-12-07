use std::collections::HashSet;

fn main() {
    let groups = include_str!("../../inputs/06.txt")
        .split("\n\n")
        .flat_map(|segment| segment.split("\r\n\r\n"));

    let part_1: usize = groups
        .clone()
        .map(|segment| count_unique_chars(segment))
        .sum();

    let part_2: usize = groups.map(|group| count_all_yes(group)).sum();

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

fn count_unique_chars(s: &str) -> usize {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<HashSet<_>>()
        .len()
}

fn count_all_yes(s: &str) -> usize {
    let r = s
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<_>>()
        })
        .fold(('a'..='z').collect(), |accum, x| &accum & &x);

    r.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(count_unique_chars("abc"), 3);
        assert_eq!(count_unique_chars("a\nb\nc"), 3);
        assert_eq!(count_unique_chars("ab\nac"), 3);
        assert_eq!(count_unique_chars("a\na\na\na"), 1);
        assert_eq!(count_unique_chars("b"), 1);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_all_yes("abc"), 3);
        assert_eq!(count_all_yes("a\nb\nc"), 0);
        assert_eq!(count_all_yes("ab\nac"), 1);
        assert_eq!(count_all_yes("a\na\na\na"), 1);
        assert_eq!(count_all_yes("b"), 1);
    }
}
