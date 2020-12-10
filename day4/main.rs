use std::fs;
use regex::Regex;

struct Credentials {
    birth_year: u16,
    issue_year: u16,
    expiration_year: u16,
    height: Height,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: bool
}

struct Height {
    value: u8,
    units: HeightUnits
}

#[derive(Debug)]
enum HeightUnits {
    In,
    Cm
}

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_4\\passport_processing\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let passport_defs: Vec<String> = get_passport_defs(contents);
    let count_valid: usize = passport_defs.iter()
                                          .map(|def| parse_credentials(def))
                                          .filter(|credentials| validate_credentials(credentials))
                                          .count();

    println!("Valid passport count: {}", count_valid);
}

fn get_passport_defs(data: String) -> Vec<String> {
    let mut passports: Vec<String> = Vec::with_capacity(data.split('\n').count());

    let mut passport: Vec<&str> = Vec::with_capacity(data.split('\n').count());
    for line in data.split('\n') {
        if line.trim().is_empty() {
            if passport.len() > 0 {
                passports.push(passport.join(" "));
            }
            passport.clear();
        }
        else {
            passport.push(line.trim());
        }
    }

    if passport.len() > 0 {
        passports.push(passport.join(" "));
    }
    
    return passports;
}

fn parse_credentials(def: &str) -> Credentials {
    let mut credentials = Credentials {
        birth_year: 0,
        issue_year: 0,
        expiration_year: 0,
        height: Height {
            value: 0,
            units: HeightUnits::Cm
        },
        hair_color: String::from(""),
        eye_color: String::from(""),
        passport_id: String::from(""),
        country_id: false
    };

    for field in def.split(' ') {
        let field_split: Vec<&str> = field.split(':').collect();
        let label: &str = field_split[0].trim();
        let val: &str = field_split[1].trim();
        
        match label {
            "byr" => credentials.birth_year = parse_year(val),
            "iyr" => credentials.issue_year = parse_year(val),
            "eyr" => credentials.expiration_year = parse_year(val),
            "hgt" => credentials.height = parse_height(field_split[1].trim()),
            "hcl" => credentials.hair_color = String::from(field_split[1].trim()),
            "ecl" => credentials.eye_color = String::from(field_split[1].trim()),
            "pid" => credentials.passport_id = String::from(field_split[1].trim()),
            "cid" => credentials.country_id = true,
            _ => println!("no known field: {}", label)
        }
    }

    return credentials;
}

fn parse_year(year_def: &str) -> u16 {
    let parsed_year = year_def.parse();
    if parsed_year.is_ok() {
        return parsed_year.unwrap();
    }
    return 0;
}

fn parse_height(height_def: &str) -> Height {
    let height_regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let mut value: u8 = 0;
    let mut units: HeightUnits = HeightUnits::Cm;
    for cap in height_regex.captures_iter(height_def) {
        let parsed_value = cap[1].parse();
        if parsed_value.is_ok() {
            value = parsed_value.unwrap();
        }

        if &cap[2] == "cm" {
            units = HeightUnits::Cm;
        }
        else {
            units = HeightUnits::In
        }
    }
    
    return Height {
        value: value,
        units: units
    }
}

fn validate_credentials(credentials: &Credentials) -> bool {
    if credentials.birth_year < 1920 || credentials.birth_year > 2002 {
        return false;
    }

    if credentials.issue_year < 2010 || credentials.issue_year > 2020 {
        return false;
    }

    if credentials.expiration_year < 2020 || credentials.expiration_year > 2030 {
        return false;
    }

    let valid_height: bool;
    match credentials.height.units {
        HeightUnits::In => valid_height = credentials.height.value >= 59 && credentials.height.value <= 76,
        HeightUnits::Cm => valid_height = credentials.height.value >= 150 && credentials.height.value <= 193
    }

    if !valid_height {
        return false;
    }

    let hair_color_regex = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
    if !hair_color_regex.is_match(&credentials.hair_color) {
        return false;
    }

    let eye_color_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    if !eye_color_regex.is_match(&credentials.eye_color) {
        return false;
    }

    let passport_id_regex = Regex::new(r"^\d{9}$").unwrap();
    if !passport_id_regex.is_match(&credentials.passport_id) {
        return false;
    }

    return true;
}