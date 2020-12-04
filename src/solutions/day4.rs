use std::{fmt, str, collections, cmp};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CREDENTIAL_PATTERN: Regex = Regex::new(r"(\w{3}):(#??\w+)").unwrap();
    static ref HEIGHT_PATTERN: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    static ref HAIR_PATTERN: Regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
    static ref EYE_PATTERN: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    static ref PASSPORT_PATTERN: Regex = Regex::new(r"\d{9}").unwrap();
}

#[derive(fmt::Debug,cmp::PartialEq, cmp::Eq)]
struct NorthPoleCredentials {
    pub birth_year: String,
    pub issue_year: String,
    pub expiration_year: String,
    pub height: String,
    pub hair_color: String,
    pub eye_color: String,
    pub passport_id: String,
    pub country_id: Option<String>
}
struct ParseNorthPoleCredentialsError;
impl str::FromStr for NorthPoleCredentials {
    type Err = ParseNorthPoleCredentialsError; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut properties: collections::HashMap<String, String> = collections::HashMap::new(); 
        for cap in CREDENTIAL_PATTERN.captures_iter(s) {
            properties.insert(String::from(&cap[1]), String::from(&cap[2]));
        }
        Ok(NorthPoleCredentials{
            birth_year: properties.get("byr").map(Into::into)
                .ok_or(ParseNorthPoleCredentialsError)?, 
            issue_year: properties.get("iyr").map(Into::into)
                .ok_or(ParseNorthPoleCredentialsError)?,
            expiration_year: properties.get("eyr").map(Into::into)
                .ok_or(ParseNorthPoleCredentialsError)?,
            height: properties.get("hgt").map(Into::into)
                .ok_or(ParseNorthPoleCredentialsError)?,
            hair_color: properties.get("hcl").map(Into::into)
                .ok_or(ParseNorthPoleCredentialsError)?,
            eye_color: properties.get("ecl").map(Into::into)
                .ok_or(ParseNorthPoleCredentialsError)?,
            passport_id: properties.get("pid").map(Into::into)
                .ok_or(ParseNorthPoleCredentialsError)?,
            country_id: properties.get("cid").map(Into::into) 
        })
    }
}

impl NorthPoleCredentials {
    fn is_valid(self: &Self) -> bool {
        if !in_num_range(&self.birth_year, 1920, 2002) ||
           !in_num_range(&self.issue_year, 2010, 2020) ||
           !in_num_range(&self.expiration_year, 2020, 2030) ||
           !valid_height(&self.height) ||
           !valid_hair_color(&self.hair_color) ||
           !valid_eye_color(&self.eye_color) ||
           !valid_passport_id(&self.passport_id) {
            return false;
        }
        return true;
    }
}

pub fn run(input: String) {
    let credentials = parse_credentials(input);

    let part_1: Vec<&NorthPoleCredentials> = credentials.iter()
        .collect();
    println!("Part 1: {}", part_1.len());

    let part_2: Vec<&NorthPoleCredentials> = credentials.iter()
        .filter(|credential| credential.is_valid())
        .collect(); 
    println!("Part 2: {}", part_2.len());
}

fn parse_credentials(input: String) -> Vec<NorthPoleCredentials> {
    input.split("\n\n")
        .map(|credentials_data| credentials_data.parse::<NorthPoleCredentials>())
        .filter_map(Result::ok)
        .collect()
}

fn in_num_range(str: &String, lower: u32, upper: u32) -> bool {
    let numeric_value: u32 = match str.parse() {
        Ok(value) => value,
        Err(_e) => return false
    };
    if numeric_value < lower || numeric_value > upper {
        return false;
    }
    return true;
}

fn valid_height(str: &String) -> bool {
    match HEIGHT_PATTERN.captures(str) {
        Some(captures) => {
            let unit = &captures[2];
            let length: u32 = match &captures[1].parse::<u32>() {
                Ok(value) => *value,
                Err(_e) => return false
            };
            if unit == "cm" && (length < 150 || length > 193) {
                return false;
            } 
            if unit == "in" && (length < 59 || length > 76) {
                return false;
            }
            return true;
        },
        None => return false
    }
}

fn valid_hair_color(str: &String) -> bool {
    HAIR_PATTERN.is_match(str)
}

fn valid_eye_color(str: &String) -> bool {
    EYE_PATTERN.is_match(str)
}

fn valid_passport_id(str: &String) -> bool {
    PASSPORT_PATTERN.is_match(str) && str.len() == 9
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_credential() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm".to_string();
        let creds = parse_credentials(input);
        assert_eq!(creds.len(), 1);
        assert_eq!(*creds.get(0).unwrap(), NorthPoleCredentials{
            birth_year: String::from("1937"),
            issue_year: String::from("2017"),
            expiration_year: String::from("2020"),
            height: String::from("183cm"),
            hair_color: String::from("#fffffd"),
            eye_color: String::from("gry"),
            passport_id: String::from("860033327"),
            country_id: Some(String::from("147")),
        })
    }

    #[test]
    fn parse_invalid_credential() {
        let creds = parse_credentials(String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"));
        assert_eq!(creds.len(), 0);
    }

    #[test]
    fn invalid_passports() {
        let creds = parse_credentials(String::from("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"));
        assert_eq!(creds.get(0).unwrap().is_valid(), false);
        let creds = parse_credentials(String::from("iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946"));
        assert_eq!(creds.get(0).unwrap().is_valid(), false);
        let creds = parse_credentials(String::from("hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"));
        assert_eq!(creds.get(0).unwrap().is_valid(), false);
        let creds = parse_credentials(String::from("hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"));
        assert_eq!(creds.get(0).unwrap().is_valid(), false);
    }

    #[test]
    fn valid_passports() {
        let creds = parse_credentials(String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"));
        assert_eq!(creds.get(0).unwrap().is_valid(), true);
        let creds = parse_credentials(String::from("eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"));
        assert_eq!(creds.get(0).unwrap().is_valid(), true);
        let creds = parse_credentials(String::from("hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022"));
        assert_eq!(creds.get(0).unwrap().is_valid(), true);
        let creds = parse_credentials(String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"));
        assert_eq!(creds.get(0).unwrap().is_valid(), true);
    }

    #[test]
    fn birth_year() {
        assert_eq!(in_num_range(&String::from("2002"), 1920, 2002), true);
        assert_eq!(in_num_range(&String::from("2003"), 1920, 2002), false);
    }

    #[test]
    fn height() {
        assert_eq!(valid_height(&String::from("60in")), true);
        assert_eq!(valid_height(&String::from("190cm")), true);
        assert_eq!(valid_height(&String::from("190in")), false);
        assert_eq!(valid_height(&String::from("190")), false);
    }

    #[test]
    fn hair_color() {
        assert_eq!(valid_hair_color(&String::from("#123abc")), true);
        assert_eq!(valid_hair_color(&String::from("#123abz")), false);
        assert_eq!(valid_hair_color(&String::from("123abc")), false);
    }

    #[test]
    fn eye_color() {
        assert_eq!(valid_eye_color(&String::from("brn")), true);
        assert_eq!(valid_eye_color(&String::from("wat")), false);
    }
    
    #[test]
    fn passport_id() {
        assert_eq!(valid_passport_id(&String::from("000000001")), true);
        assert_eq!(valid_passport_id(&String::from("0123456789")), false);
    }
}