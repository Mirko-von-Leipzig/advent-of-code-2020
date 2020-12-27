pub mod d4 {
    use regex::Regex;
    use std::convert::TryFrom;
    #[derive(Debug)]
    pub struct Passport {
        pub byr: String,         // birth year
        pub iyr: String,         // issue year
        pub eyr: String,         // expiration year
        pub hgt: String,         // height
        pub hcl: String,         // hair color
        pub ecl: String,         // eye color
        pub pid: String,         // passport id
        pub cid: Option<String>, // coutry id
    }

    impl TryFrom<&str> for Passport {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let byr = Regex::new(r"byr:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value)
                .ok_or("No birthyear!")?;
            let iyr = Regex::new(r"iyr:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value)
                .ok_or("No issued year!")?;
            let eyr = Regex::new(r"eyr:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value)
                .ok_or("No expiration year!")?;
            let hgt = Regex::new(r"hgt:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value)
                .ok_or("No height!")?;
            let hcl = Regex::new(r"hcl:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value)
                .ok_or("No hair color!")?;
            let ecl = Regex::new(r"ecl:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value)
                .ok_or("No eye color!")?;
            let pid = Regex::new(r"pid:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value)
                .ok_or("No PID!")?;
            let cid = Regex::new(r"cid:(#?[0-9a-z]*)[\s\n]?")
                .unwrap()
                .captures(value);

            Ok(Passport {
                byr: String::from(byr.get(1).unwrap().as_str()),
                iyr: String::from(iyr.get(1).unwrap().as_str()),
                eyr: String::from(eyr.get(1).unwrap().as_str()),
                hgt: String::from(hgt.get(1).unwrap().as_str()),
                hcl: String::from(hcl.get(1).unwrap().as_str()),
                ecl: String::from(ecl.get(1).unwrap().as_str()),
                pid: String::from(pid.get(1).unwrap().as_str()),
                cid: match cid {
                    Some(cap) => Some(String::from(cap.get(1).unwrap().as_str())),
                    None => None,
                },
            })
        }
    }

    pub fn parse_passport_1(str: &str) -> Result<Passport, &'static str> {
        Passport::try_from(str)
    }

    pub fn parse_passport_2(value: &str) -> Result<Passport, &'static str> {
        let byr = Regex::new(r"byr:(\d\d\d\d)[\s\n]?")
            .unwrap()
            .captures(value)
            .ok_or("No birthyear!")?; // 1920 - 2002
        let iyr = Regex::new(r"iyr:(\d\d\d\d)[\s\n]?")
            .unwrap()
            .captures(value)
            .ok_or("No issued year!")?; // 2010 - 2020
        let eyr = Regex::new(r"eyr:(\d\d\d\d)[\s\n]?")
            .unwrap()
            .captures(value)
            .ok_or("No expiration year!")?; // 2020 - 2030
        let hgt = Regex::new(r"hgt:(\d*)(cm|in)[\s\n]?")
            .unwrap()
            .captures(value)
            .ok_or("No height!")?; // cm, 150 - 193: in, 59 - 76
        let hcl = Regex::new(r"hcl:(#[0-9a-fA-F]{6})[\s\n]?")
            .unwrap()
            .captures(value)
            .ok_or("No hair color!")?;
        let ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)[\s\n]?")
            .unwrap()
            .captures(value)
            .ok_or("No eye color!")?;
        let pid = Regex::new(r"pid:(\d{9})[\s\n]?")
            .unwrap()
            .captures(value)
            .ok_or("No PID!")?;
        let cid = Regex::new(r"cid:(#?[0-9a-z]*)[\s\n]")
            .unwrap()
            .captures(value);

        if !validate_range(1920, 2002, byr.get(1).unwrap().as_str().parse().unwrap()) {
            return Err("Invalid birth year");
        };

        if !validate_range(2010, 2020, iyr.get(1).unwrap().as_str().parse().unwrap()) {
            return Err("Invalid issue year");
        };

        if !validate_range(2020, 2030, eyr.get(1).unwrap().as_str().parse().unwrap()) {
            return Err("Invalid issue year");
        };

        match hgt.get(2).unwrap().as_str() {
            "in" => {
                if !validate_range(59, 76, hgt.get(1).unwrap().as_str().parse().unwrap()) {
                    return Err("INVALID HEIGHT INCHES");
                };
            }
            "cm" => {
                if !validate_range(150, 193, hgt.get(1).unwrap().as_str().parse().unwrap()) {
                    return Err("INVALID HEIGHT CENTIMETERS");
                };
            }
            _ => {
                return Err("UKNOWN HEIGHT UNIT");
            }
        };

        Ok(Passport {
            byr: String::from(byr.get(1).unwrap().as_str()),
            iyr: String::from(iyr.get(1).unwrap().as_str()),
            eyr: String::from(eyr.get(1).unwrap().as_str()),
            hgt: String::from(hgt.get(1).unwrap().as_str()),
            hcl: String::from(hcl.get(1).unwrap().as_str()),
            ecl: String::from(ecl.get(1).unwrap().as_str()),
            pid: String::from(pid.get(1).unwrap().as_str()),
            cid: match cid {
                Some(cap) => Some(String::from(cap.get(1).unwrap().as_str())),
                None => None,
            },
        })
    }

    fn validate_range(min: i32, max: i32, value: i32) -> bool {
        let range = min..(max + 1);
        range.contains(&value)
    }

    pub fn seperate_passports(str: &str) -> Vec<&str> {
        str.split("\n\n").collect()
    }
}
