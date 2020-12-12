use std::collections::HashMap;

fn main() {
    let mut adapters: Vec<u64> = include_str!("../../inputs/10.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let diff_counts = chain_diffs_counts(adapters.as_mut_slice());

    let part_1 = diff_counts.get(&1).unwrap_or(&0) * diff_counts.get(&3).unwrap_or(&0);
    let part_2 = combinations(adapters.as_mut_slice());

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

fn chain_diffs_counts(adapters: &mut [u64]) -> HashMap<u64, u64> {
    adapters.sort_unstable();

    let mut diff_counts = HashMap::new();

    adapters.windows(2).for_each(|window| {
        let diff = window[1] - window[0];

        let current_count = diff_counts.entry(diff).or_insert(0);
        *current_count += 1;
    });

    // built-in adapter (we know this diff = 3)
    let current_count = diff_counts.entry(3).or_insert(0);
    *current_count += 1;

    // diff of first adapter to outlet (0 jolts)
    let current_count = diff_counts.entry(adapters[0]).or_insert(0);
    *current_count += 1;

    diff_counts
}

fn combinations(adapters: &mut [u64]) -> u64 {
    let mut counts = HashMap::<u64, u64>::new();

    // reverse sort
    adapters.sort_unstable_by(|a, b| b.cmp(a));

    // insert count for built-in adapter
    counts.insert(adapters[0] + 3, 1);

    adapters.iter().for_each(|this_jolt| {
        let count: u64 = (this_jolt + 1..=this_jolt + 3)
            .map(|connection| counts.get(&connection).unwrap_or(&0))
            .sum();

        counts.insert(*this_jolt, count);
    });

    (1..=3).map(|jolts| counts.get(&jolts).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let diff_counts = chain_diffs_counts(adapters.as_mut_slice());
        let diff_counts_iter = diff_counts.iter();

        let counts: Vec<(u64, u64)> = diff_counts_iter.map(|(&a, &b)| (a, b)).collect();

        assert_eq!(counts.len(), 2);
        assert!(counts.contains(&(1, 7)));
        assert!(counts.contains(&(3, 5)));

        assert_eq!(combinations(adapters.as_mut_slice()), 8);
    }

    #[test]
    fn example_2() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        let diff_counts = chain_diffs_counts(adapters.as_mut_slice());
        let diff_counts_iter = diff_counts.iter();

        let counts: Vec<(u64, u64)> = diff_counts_iter.map(|(&a, &b)| (a, b)).collect();

        assert_eq!(counts.len(), 2);
        assert!(counts.contains(&(1, 22)));
        assert!(counts.contains(&(3, 10)));

        assert_eq!(combinations(adapters.as_mut_slice()), 19208);
    }
}
