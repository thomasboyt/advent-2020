use regex::Regex;

fn is_valid_one(policy: &Policy) -> bool {
    let appearances = policy.password.chars().fold(0, |acc, c|
        if c == policy.character { acc + 1 } else { acc }
    );
    appearances <= policy.second_num && appearances >= policy.first_num
}

pub fn part_one(input: &str) -> String {
    let policies = parse_input(input);
    policies.iter().fold(0, |acc, policy| {
        if is_valid_one(policy) {
            acc + 1
        } else {
            acc
        }
    }).to_string()
}

fn is_valid_two(policy: &Policy) -> bool {
    let chars: Vec<char> = policy.password.chars().collect();
    let first_pos = policy.first_num - 1;
    let second_pos = policy.second_num - 1;
    return
        (chars[first_pos as usize] == policy.character && chars[second_pos as usize] != policy.character) ||
        (chars[first_pos as usize] != policy.character && chars[second_pos as usize] == policy.character)
}

pub fn part_two(input: &str) -> String {
    let policies = parse_input(input);
    policies.iter().fold(0, |acc, policy| {
        if is_valid_two(policy) {
            acc + 1
        } else {
            acc
        }
    }).to_string()
}

fn parse_input(input: &str) -> Vec<Policy> {
    let lines = input.split("\n");
    let policies: Vec<Policy> = lines.map(|line| {
        let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)").unwrap();
        let captures = re.captures(line).unwrap();
        Policy {
            first_num: captures[1].parse().unwrap(),
            second_num: captures[2].parse().unwrap(),
            character: captures[3].parse().unwrap(),
            password: captures[4].to_string()
        }
    }).collect();
    return policies
}

#[derive(PartialEq, Eq, Debug)]
struct Policy {
    first_num: i32,
    second_num: i32,
    character: char,
    password: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d02_parsing_works() {
        fn make_policy(first_num: i32, second_num: i32, character: char, password: &str) -> Policy {
            Policy { first_num, second_num, character, password: password.to_string() }
        };

        let input = "1-3 a: abcde\n\
            1-3 b: cdefg\n\
            2-9 c: ccccccccc";

        let parsed = parse_input(input);

        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0], make_policy(1, 3, 'a', "abcde"));
        assert_eq!(parsed[1], make_policy(1, 3, 'b', "cdefg"));
        assert_eq!(parsed[2], make_policy(2, 9, 'c', "ccccccccc"));
    }

    #[test]
    fn d02_test_part_one() {
        let input = "1-3 a: abcde\n\
            1-3 b: cdefg\n\
            2-9 c: ccccccccc";
        let output = part_one(input);
        assert_eq!(output, "2")
    }

    #[test]
    fn d02_test_part_two() {
        let input = "1-3 a: abcde\n\
            1-3 b: cdefg\n\
            2-9 c: ccccccccc";
        let output = part_two(input);
        assert_eq!(output, "1")
    }
}