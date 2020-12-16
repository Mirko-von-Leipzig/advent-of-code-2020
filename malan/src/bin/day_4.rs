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
 use std::convert::TryFrom;

fn main() {
    println!("Day 4");
    let _input_string = include_str!("../inputs/04.txt");
    puzzle_1(_input_string);
    puzzle_2(_input_string);
    let p = d4::Passport::try_from(_input_string).unwrap();
    println!("{:?}", p);
}

fn puzzle_1(_input: &str) {
    println!("Running Puzzle 1")

}

fn puzzle_2(_input: &str) {
    println!("Running Puzzle 2")
}

#[cfg(test)]
mod tests {

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
    }

    #[test]
    fn test_puzzle_2() {}
}
