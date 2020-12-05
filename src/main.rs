use std::env;
use std::process;
use std::fs;
mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Missing required argument: please call with number of day to run & part 1 or 2");
        process::exit(1)
    }
    let num_day: usize = args[1].clone().parse::<usize>().unwrap_or_else(|err| {
        println!("Could not parse day to int: {}", err);
        process::exit(1)
    });
    let num_part: usize = args[2].clone().parse::<usize>().unwrap_or_else(|err| {
        println!("Could not parse part to int: {}", err);
        process::exit(1)
    });

    run(num_day, num_part)
}

fn run(num_day: usize, num_part: usize) {
    let days = [
        [days::d01::part_one, days::d01::part_two],
        [days::d02::part_one, days::d02::part_two]
    ];
    if num_day > days.len() {
        println!("Day out of range: {}", num_day);
        process::exit(1)
    }
    if num_part > 2 {
        println!("Part out of range: {}", num_part);
        process::exit(1)
    }
    let input = read_input(num_day);
    let result = days[num_day - 1][num_part - 1](&input);
    println!("{}", result)
}

fn read_input(num_day: usize) -> String {
    let file_name = format!("input/{}.txt", num_day);
    let msg = format!("Could not read input {}", file_name);
    fs::read_to_string(file_name).expect(&msg)
}