fn main() {
    let mut data: Vec<u64> = include_str!("../../inputs/09.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let invalid = find_first_invalid(&data, 25).unwrap();
    let sequence = find_sequence(data.as_mut_slice(), invalid).unwrap();

    sequence.sort_unstable();

    let weakness = sequence.first().unwrap() + sequence.last().unwrap();

    println!("part 1: {:?}", invalid);
    println!("part 2: {:?}", weakness);
}

fn find_first_invalid(data: &[u64], preamble_length: usize) -> Option<u64> {
    data.windows(preamble_length)
        .zip(data.iter().skip(preamble_length))
        .find(|(window, &target)| {
            !window
                .iter()
                .any(|&value| target > value && window.contains(&(target - value)))
        })
        .map(|(_, value)| *value)
}

fn find_sequence(data: &mut [u64], target: u64) -> Option<&mut [u64]> {
    let mut current_sum = 0;

    let mut i_start = 0;
    let mut i_end = 0;

    while i_end < data.len() {
        if current_sum < target {
            current_sum += data[i_end];

            i_end += 1;
        }

        if current_sum > target {
            current_sum -= data[i_start];
            i_start += 1;
        }

        if current_sum == target && i_start != i_end {
            return Some(&mut data[i_start..i_end]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut data = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_first_invalid(&data, 5), Some(127));
        assert_eq!(
            find_sequence(&mut data, 127),
            Some(vec![15u64, 25, 47, 40].as_mut_slice())
        );
    }
}
