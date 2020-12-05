fn main() {
    let list = include_str!("../../inputs/02.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(':');

            let policy = split
                .next()
                .expect("line: policy")
                .parse::<Policy>()
                .expect("format must match policy");

            let password = split.next().expect("line must contain a password").trim();

            (policy, password)
        })
        .collect::<Vec<_>>();

    // part 1
    let part_1 = list
        .iter()
        .filter(|(policy, password)| policy.validate_sled_shop(password))
        .count();

    // part 2
    let part_2 = list
        .iter()
        .filter(|(policy, password)| policy.validate_official(password))
        .count();

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

struct Policy {
    arg_1: usize,
    arg_2: usize,
    symbol: char,
}

impl Policy {
    fn new(arg_1: usize, arg_2: usize, symbol: char) -> Self {
        Self {
            arg_1,
            arg_2,
            symbol,
        }
    }

    fn validate_sled_shop(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.symbol).count();

        count <= self.arg_2 && count >= self.arg_1
    }

    fn validate_official(&self, password: &str) -> bool {
        let first_matches = password.chars().nth(self.arg_1 - 1) == Some(self.symbol);
        let second_matches = password.chars().nth(self.arg_2 - 1) == Some(self.symbol);

        (first_matches && !second_matches) || (second_matches && !first_matches)
    }
}

impl std::str::FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"(\d+)\-(\d+) ([[:alpha:]])").unwrap();

        let matches = re.captures(s).unwrap();

        let minimum = matches[1].parse::<usize>().unwrap();
        let maximum = matches[2].parse::<usize>().unwrap();

        let symbol = matches[3].chars().next().unwrap();

        Ok(Policy::new(minimum, maximum, symbol))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sled_shop_example_1() {
        let uut = Policy::new(1, 3, 'a');
        assert!(uut.validate_sled_shop("abcde"));
    }

    #[test]
    fn sled_shop_example_2() {
        let uut = Policy::new(2, 9, 'c');
        assert!(uut.validate_sled_shop("ccccccccc"));
    }

    #[test]
    fn sled_shop_example_3() {
        let uut = Policy::new(1, 3, 'b');
        assert!(!uut.validate_sled_shop("cdefg"));
    }

    #[test]
    fn official_example_1() {
        let uut = Policy::new(1, 3, 'a');
        assert!(uut.validate_official("abcde"));
    }

    #[test]
    fn official_example_2() {
        let uut = Policy::new(2, 9, 'c');
        assert!(!uut.validate_official("ccccccccc"));
    }

    #[test]
    fn official_example_3() {
        let uut = Policy::new(1, 3, 'b');
        assert!(!uut.validate_official("cdefg"));
    }

    #[test]
    fn solution() {
        let list = include_str!("../../inputs/02.txt")
            .lines()
            .map(|line| {
                let mut split = line.split(':');

                let policy = split
                    .next()
                    .expect("line: policy")
                    .parse::<Policy>()
                    .expect("format must match policy");

                let password = split.next().expect("line must contain a password").trim();

                (policy, password)
            })
            .collect::<Vec<_>>();

        // part 1
        let part_1 = list
            .iter()
            .filter(|(policy, password)| policy.validate_sled_shop(password))
            .count();

        // part 2
        let part_2 = list
            .iter()
            .filter(|(policy, password)| policy.validate_official(password))
            .count();

        assert_eq!(part_1, 580);
        assert_eq!(part_2, 611);
    }
}
