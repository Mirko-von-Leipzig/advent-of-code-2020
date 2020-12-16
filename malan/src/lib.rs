use std::convert::TryFrom;

pub mod d4 {
    #[derive(Debug)]
    pub struct Passport {
        byr: String,    // birth year
        iyr: String,    // issue year
        eyr: String,    // expiration year
        hgt: String,    // height
        hcl: String,    // hair color
        ecl: String,    // eye color
        pid: String,    // passport id
        cid: String,    // coutry id
    }

    impl super::TryFrom<&str> for Passport {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            println!("{}", value);
            Ok(Passport {
                byr: "test".to_string(),
                iyr: "test".to_string(),
                eyr: "test".to_string(),
                hgt: "test".to_string(),
                hcl: "test".to_string(),
                ecl: "test".to_string(),
                pid: "test".to_string(),
                cid: "test".to_string(),
            })
        }

    }

    pub fn parse_passport(str: &str) -> bool {
        true
    }
}