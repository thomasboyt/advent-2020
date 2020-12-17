use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn map_contains_keys(map: &HashMap<String, String>, keys: &[&str]) -> bool {
    keys.iter().all(|&s| map.contains_key(s))
}

pub fn part_one(input: &str) -> String {
    let required_fields = ["ecl", "eyr", "pid", "hcl", "byr", "iyr", "hgt"];
    let passports = parse_input(input);
    let result = passports.iter().fold(0, |acc, passport| {
        if map_contains_keys(passport, &required_fields) {
            acc + 1
        } else {
            acc
        }
    });
    result.to_string()
}

fn validate_byr(val: Option<&String>) -> bool {
    match val {
        None => false,
        Some(byr) => {
            if byr.len() < 4 { return false; }
            let byr_int = byr.parse::<i32>().unwrap();
            byr_int >= 1920 && byr_int <= 2002
        }
    }
}

fn validate_iyr(val: Option<&String>) -> bool {
    match val {
        None => false,
        Some(iyr) => {
            if iyr.len() < 4 { return false; }
            let iyr_int = iyr.parse::<i32>().unwrap();
            iyr_int >= 2010 && iyr_int <= 2020
        }
    }
}

fn validate_eyr(val: Option<&String>) -> bool {
    match val {
        None => false,
        Some(eyr) => {
            if eyr.len() < 4 { return false; }
            let eyr_int = eyr.parse::<i32>().unwrap();
            eyr_int >= 2020 && eyr_int <= 2030
        }
    }
}

fn validate_hgt(val: Option<&String>) -> bool {
    match val {
        None => false,
        Some(hgt) => {
            let re = Regex::new(r"^(\d+)(cm|in)").unwrap();
            let captures = match re.captures(hgt) {
                Some(captures) => captures,
                None => return false
            };
            let num = captures[1].parse::<i32>().unwrap();
            let unit = &captures[2];
            if unit == "cm" {
                num >= 150 && num <= 193
            } else {
                num >= 59 && num <= 76
            }
        }
    }
}

fn validate_hcl(val: Option<&String>) -> bool {
    let hcl = match val {
        None => return false,
        Some(hcl) => hcl
    };
    let re = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
    re.is_match(hcl)
}

fn validate_ecl(val: Option<&String>) -> bool {
    let valid_colors: HashSet<_> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
    let ecl = match val {
        None => return false,
        Some(ecl) => ecl
    };
    valid_colors.contains(&ecl[..])
}

fn validate_pid(val: Option<&String>) -> bool {
    let pid = match val {
        None => return false,
        Some(pid) => pid
    };
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(pid)
}

pub fn part_two(input: &str) -> String {
    let passports = parse_input(input);
    let result = passports
        .iter()
        .map(|passport| {
            if !validate_byr(passport.get("byr")) {
                return false;
            }
            if !validate_iyr(passport.get("iyr")) {
                return false;
            }
            if !validate_eyr(passport.get("eyr")) {
                return false;
            }
            if !validate_hgt(passport.get("hgt")) {
                return false;
            }
            if !validate_hcl(passport.get("hcl")) {
                return false;
            }
            if !validate_ecl(passport.get("ecl")) {
                return false;
            }
            if !validate_pid(passport.get("pid")) {
                return false;
            }
            true
        })
        .fold(0, |acc, is_valid| if is_valid { acc + 1 } else { acc });
    result.to_string()
}

pub fn parse_input(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|passport| {
            let pairs: Vec<Vec<&str>> = passport
                .split_whitespace()
                .map(|pair| pair.split(":").collect())
                .collect();
            pairs
                .iter()
                .map(|key_value| (key_value[0].to_owned(), key_value[1].to_owned()))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm\n\
        \n\
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929\n\
        \n\
        hcl:#ae17e1 iyr:2013\n\
        eyr:2024\n\
        ecl:brn pid:760753108 byr:1931\n\
        hgt:179cm\n\
        \n\
        hcl:#cfa07d eyr:2025 pid:166559648\n\
        iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn d04_parsing_works() {
        let parsed = parse_input(TEST_INPUT);
        assert_eq!(
            map_contains_keys(
                &parsed[0],
                &["ecl", "eyr", "pid", "hcl", "byr", "iyr", "cid", "hgt"]
            ),
            true
        );
        assert_eq!(
            map_contains_keys(
                &parsed[1],
                &["ecl", "eyr", "pid", "hcl", "byr", "iyr", "cid"]
            ),
            true
        );
        assert_eq!(
            map_contains_keys(
                &parsed[2],
                &["ecl", "eyr", "pid", "hcl", "byr", "iyr", "hgt"]
            ),
            true
        );
        assert_eq!(
            map_contains_keys(&parsed[3], &["ecl", "eyr", "pid", "hcl", "iyr", "hgt"]),
            true
        );
    }

    #[test]
    fn d04_test_part_one() {
        let output = part_one(TEST_INPUT);
        assert_eq!(output, "2")
    }

    #[test]
    fn d04_test_part_two() {
        let invalid_passports = "eyr:1972 cid:100\n\
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
            \n\
            iyr:2019\n\
            hcl:#602927 eyr:1967 hgt:170cm\n\
            ecl:grn pid:012533040 byr:1946\n\
            \n\
            hcl:dab227 iyr:2012\n\
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
            \n\
            hgt:59cm ecl:zzz\n\
            eyr:2038 hcl:74454a iyr:2023\n\
            pid:3556412378 byr:2007";
        let invalid_output = part_two(invalid_passports);
        assert_eq!(invalid_output, "0");

        let valid_passports = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
            hcl:#623a2f\n\
            \n\
            eyr:2029 ecl:blu cid:129 byr:1989\n\
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
            \n\
            hcl:#888785\n\
            hgt:164cm byr:2001 iyr:2015 cid:88\n\
            pid:545766238 ecl:hzl\n\
            eyr:2022\n\
            \n\
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let valid_output = part_two(valid_passports);
        assert_eq!(valid_output, "4");
    }
}
