use std::collections::HashMap;

fn main() {
    let raw_passports = include_str!("../../inputs/04.txt");
    let passports = parse_batch_file(raw_passports);

    let required_fields_count = passports
        .iter()
        .filter(|passport| passport.has_required_fields())
        .count();

    let valid_count = passports
        .iter()
        .filter(|passport| passport.has_valid_fields())
        .count();

    println!("passports with required fields: {}", required_fields_count);
    println!("passports with valid fields: {}", valid_count);
}

#[derive(Default)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn has_required_fields(&self) -> bool {
        self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid")
        // && self.fields.contains_key("cid")
    }

    fn has_valid_fields(&self) -> bool {
        self.has_required_fields()
            && self.has_valid_byr()
            && self.has_valid_ecl()
            && self.has_valid_eyr()
            && self.has_valid_hcl()
            && self.has_valid_hgt()
            && self.has_valid_iyr()
            && self.has_valid_pid()
    }

    fn has_valid_year(&self, year_key: &str, min: u16, max: u16) -> bool {
        let year = self.fields.get(year_key).unwrap().parse::<u16>().unwrap();

        year >= min && year <= max
    }

    fn has_valid_byr(&self) -> bool {
        self.has_valid_year("byr", 1920, 2002)
    }

    fn has_valid_iyr(&self) -> bool {
        self.has_valid_year("iyr", 2010, 2020)
    }

    fn has_valid_eyr(&self) -> bool {
        self.has_valid_year("eyr", 2020, 2030)
    }

    fn has_valid_ecl(&self) -> bool {
        let str = self.fields.get("ecl").unwrap_or(&"0");

        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(str)
    }

    fn has_valid_pid(&self) -> bool {
        let str = self.fields.get("pid").unwrap();

        str.len() == 9 && str.chars().all(|c| c.is_ascii_digit())
    }

    fn has_valid_hcl(&self) -> bool {
        let str = self.fields.get("hcl").unwrap();

        let re = regex::Regex::new(r"#[0-9a-f]{6}").unwrap();

        re.is_match(str)
    }

    fn has_valid_hgt(&self) -> bool {
        let str = self.fields.get("hgt").unwrap();

        if str.ends_with("cm") {
            let cm: u16 = str[0..str.len() - 2].parse::<u16>().unwrap_or(0);

            cm >= 150 && cm <= 193
        } else if str.ends_with("in") {
            let inches: u16 = str[0..str.len() - 2].parse::<u16>().unwrap_or(0);

            inches >= 59 && inches <= 76
        } else {
            false
        }
    }
}

fn parse_batch_file(contents: &str) -> Vec<Passport> {
    let passports = contents
        .split("\r\n\r\n")
        .flat_map(|line| line.split("\n\n"))
        .map(|raw_passport| {
            let mut passport = Passport::default();
            for (field, value) in raw_passport.split_whitespace().flat_map(|word| {
                let mut parts = word.split(':');
                Some((parts.next().unwrap(), parts.next().unwrap()))
            }) {
                passport.fields.insert(field, value);
            }
            passport
        })
        .collect::<Vec<Passport>>();

    passports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let batch = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let passports = parse_batch_file(&batch);

        assert_eq!(passports.len(), 4);
        assert!(passports[0].has_required_fields());
        assert!(!passports[1].has_required_fields());
        assert!(passports[2].has_required_fields());
        assert!(!passports[3].has_required_fields());
    }

    #[test]
    fn invalid_fields() {
        let batch = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let passports = parse_batch_file(&batch);
        assert_eq!(passports.len(), 4);

        assert!(!passports[0].has_valid_fields());
        assert!(!passports[1].has_valid_fields());
        assert!(!passports[2].has_valid_fields());
        assert!(!passports[3].has_valid_fields());
    }

    #[test]
    fn valid_fields() {
        let batch = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = parse_batch_file(&batch);
        assert_eq!(passports.len(), 4);

        assert!(passports[0].has_valid_fields());
        assert!(passports[1].has_valid_fields());
        assert!(passports[2].has_valid_fields());
        assert!(passports[3].has_valid_fields());
    }
}
