use std::process;

pub fn part_one(input: &str) -> String {
    let numbers = parse_input(input);
    let mut val: Option<(i32, i32)> = None;
    for n in numbers.iter() {
        for m in numbers.iter() {
            if n + m == 2020 {
                val = Some((*n, *m))
            }
        }
    }
    match val {
        None => {
            println!("No value found!");
            process::exit(1)
        }
        Some((n, m)) => {
            (n * m).to_string()
        }
    }
}

pub fn part_two(input: &str) -> String {
    let numbers = parse_input(input);
    let mut val: Option<(i32, i32, i32)> = None;
    for n in numbers.iter() {
        for m in numbers.iter() {
            for l in numbers.iter() {
                if n + m + l == 2020 {
                    val = Some((*n, *m, *l))
                }
            }
        }
    }
    match val {
        None => {
            println!("No value found!");
            process::exit(1)
        }
        Some((n, m, l)) => {
            (n * m * l).to_string()
        }
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split("\n").map(|s| s.parse::<i32>().unwrap_or_else(|err| {
        println!("error parsing input: {}", err);
        process::exit(1)
    })).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let input = "1721\n979\n366\n299\n675\n1456";
        let numbers = parse_input(input);
        assert_eq!(numbers, vec![1721, 979, 366, 299, 675, 1456])
    }

    #[test]
    fn test_part_one() {
        let input = "1721\n979\n366\n299\n675\n1456";
        let output = part_one(input);
        assert_eq!(output, "514579")
    }

    #[test]
    fn test_part_two() {
        let input = "1721\n979\n366\n299\n675\n1456";
        let output = part_two(input);
        assert_eq!(output, "241861950")
    }
}