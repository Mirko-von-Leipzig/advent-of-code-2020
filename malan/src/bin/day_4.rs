/* POSSIBLE PASSPORT FIELDS
   byr (Birth Year)            REQUIRED
   iyr (Issue Year)            REQUIRED
   eyr (Expiration Year)       REQUIRED
   hgt (Height)                REQUIRED
   hcl (Hair Color)            REQUIRED
   ecl (Eye Color)             REQUIRED
   pid (Passport ID)           REQUIRED
   cid (Country ID) (optional) OPTIONAL
*/

use malan::d4;
fn main() {
    println!("Day 4");
    let _input_string = include_str!("../inputs/04.txt");
    puzzle_1(_input_string);
    puzzle_2(_input_string);
}

fn puzzle_1(_input: &str) {
    println!("Running Puzzle 1");
    let mut num_correct: u32 = 0;
    let passports = d4::seperate_passports(_input);
    for (index, passport) in passports.into_iter().enumerate() {
        match d4::parse_passport_1(passport) {
            Ok(passport) => {
                println!("{}: OKAY! - {:?}", index, passport);
                num_correct += 1;
            }
            Err(err) => println!("{}: {}", index, err),
        }
    }
    println!("Correct number of passports: {}", num_correct);
    // answer is 242
}

fn puzzle_2(_input: &str) {
    println!("Running Puzzle 2");
    let mut num_correct: u32 = 0;
    let passports = d4::seperate_passports(_input);
    for (index, passport) in passports.into_iter().enumerate() {
        match d4::parse_passport_2(passport) {
            Ok(passport) => {
                println!("{}: OKAY! - {:?}", index, passport);
                num_correct += 1;
            }
            Err(err) => println!("{}: {}", index, err),
        }
    }
    println!("Correct number of passports: {}", num_correct);
    // answer is 186
}

#[cfg(test)]
mod tests {

    use malan::d4;

    #[test]
    fn test_puzzle_1() {
        let test_input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let mut num_correct: u32 = 0;
        let passports = d4::seperate_passports(test_input);

        for passport in passports {
            match d4::parse_passport_1(passport) {
                Ok(passport) => num_correct += 1,
                Err(err) => println!("{}", err),
            }
        }

        assert_eq!(num_correct, 2);
    }

    #[test]
    fn test_puzzle_2() {}
}
