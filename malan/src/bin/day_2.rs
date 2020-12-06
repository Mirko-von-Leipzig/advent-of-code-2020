use  regex::Regex;

struct Policy {
    required_char: char,
    min_count: usize,
    max_count: usize,
}

// Should not really be an Option, but learning
fn extract_policy(line: &str) -> Option<Policy> {
    // use a raw string to avoid escape characters
    let pattern = Regex::new(r"^(\d+)-(\d+)\s*(\w):").expect("Invalid regular expression");
    // build op the Option<Policy> return
    if let Some(caps) = pattern.captures(line) {
        Some(Policy {
            min_count: caps.get(1).unwrap().as_str().parse().unwrap(), // parse knows what type to return because struct defines the types
            max_count: caps.get(2).unwrap().as_str().parse().unwrap(),
            required_char: caps.get(3).unwrap().as_str().parse().unwrap(),
        })
    } else {
        None
    }
}

fn extract_password(line: &str) -> &str {
    let pattern = Regex::new(r":\s*(\w*)").expect("Invalid regular expression");

    pattern.captures(line).unwrap().get(1).unwrap().as_str()
}

fn verify_password_1(password: &str, policy: &Policy) -> bool {
    let count = password.chars().filter(|c| c == &policy.required_char).count();
    count >= policy.min_count && count <= policy.max_count
}

fn verify_password_2(password: &str, policy: &Policy) -> bool {
    let mut initial = false;
    for (ind, ch) in password.char_indices() {
        initial = initial^((ind+1 == policy.min_count || ind+1 == policy.max_count) && (ch == policy.required_char));
    }
    initial
}


fn main() {
    let mut correct_1: u32 = 0;
    let mut correct_2: u32 = 0;
    let input_string = include_str!("../inputs/02.txt");

    // loop over the lines of the input file
    for line in input_string.lines() {
        if let Some(policy) = extract_policy(line) {
            let password = extract_password(line);
            correct_1 = match verify_password_1(&password, &policy) {
                true => correct_1 + 1,
                false => correct_1,
            };
            correct_2 = match verify_password_2(&password, &policy) {
                true => correct_2 + 1,
                false => correct_2,
            }
        } else {
            println!("Could not extract policy from string line!");
        }
    }

    println!("Ammount of correct passwords (policy_1): {}", correct_1);
    println!("Ammount of correct passwords (policy_2): {}", correct_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_1() {
        let policy1 = Policy {required_char: 'a', min_count: 1, max_count: 3};
        let policy2 = Policy {required_char: 'b', min_count: 1, max_count: 3};
        let policy3 = Policy {required_char: 'c', min_count: 2, max_count: 9};

        assert!(verify_password_1("abcde", &policy1));
        assert!(!verify_password_1("cdefg", &policy2));
        assert!(verify_password_1("ccccccccc", &policy3));
    }

    #[test]
    fn test_policy_2() {
        let policy1 = Policy {required_char: 'a', min_count: 1, max_count: 3};
        let policy2 = Policy {required_char: 'b', min_count: 1, max_count: 3};
        let policy3 = Policy {required_char: 'c', min_count: 2, max_count: 9};

        assert!(verify_password_2("abcde", &policy1));
        assert!(!verify_password_2("cdefg", &policy2));
        assert!(!verify_password_2("ccccccccc", &policy3));
    }
}