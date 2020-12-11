use std::collections::{HashMap, HashSet};

fn main() {
    let rules: Rules = include_str!("../../inputs/07.txt").into();

    let part_1 = rules.origin_count("shiny gold");
    let part_2 = rules.bags_contained("shiny gold");

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

#[derive(Debug, Default)]
struct Rules<'a> {
    contained_by: HashMap<&'a str, Vec<&'a str>>,
    contains: HashMap<&'a str, Vec<(u16, &'a str)>>,
}

impl<'a> Rules<'a> {
    fn origin_count(&self, key: &'a str) -> usize {
        let hash = self.graph_unique(key, HashSet::new());

        hash.len() - 1
    }

    fn bags_contained(&self, key: &'a str) -> u16 {
        let mut contains = 0;

        if let Some(contained) = self.contains.get(key) {
            for (amount, bag_key) in contained {
                let inner = self.bags_contained(bag_key);

                contains += amount * (inner + 1);
            }
        }

        contains
    }

    fn graph_unique(&self, key: &'a str, hash: HashSet<&'a str>) -> HashSet<&'a str> {
        let mut hash = hash;
        if hash.contains(key) {
            return hash;
        }

        hash.insert(key);
        if let Some(bags) = self.contained_by.get(key) {
            for bag in bags {
                hash = self.graph_unique(bag, hash);
            }
        }

        hash
    }
}

impl<'a> From<&'a str> for Rules<'a> {
    fn from(s: &'a str) -> Rules<'a> {
        let mut rules = Rules::default();

        // match for (<number>) (<adjective> <color>) bags
        let regex = regex::Regex::new(r"(\d+) ([a-zA-Z]+ [a-zA-Z]+) bag").unwrap();

        s.lines().for_each(|line| {
            // split at contain
            let mut split = line.split(" contain ");
            let first_segment = split.next().unwrap();
            let second_segment = split.next().unwrap();

            let container = &first_segment[0..first_segment.rfind(' ').unwrap()];

            let contained: Vec<(u16, &str)> = regex
                .captures_iter(second_segment)
                .map(|captures| {
                    let amount: u16 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let bag = captures.get(2).unwrap().as_str();

                    (amount, bag)
                })
                .collect();

            contained.iter().for_each(|(_, bag)| {
                let mut existing = rules.contained_by.remove(bag).unwrap_or_default();
                existing.push(container);
                rules.contained_by.insert(bag, existing);
            });
            rules.contains.insert(container, contained);
        });

        rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let rules: Rules = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .into();

        assert_eq!(rules.origin_count("shiny gold"), 4);
        assert_eq!(rules.bags_contained("shiny gold"), 32);
    }

    #[test]
    fn example_2() {
        let rules: Rules = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .into();

        assert_eq!(rules.bags_contained("shiny gold"), 126);
    }
}
