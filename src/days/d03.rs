fn count_trees_hit(map: &Vec<Vec<i32>>, sx: usize, sy: usize) -> i64 {
    let mut x: usize = 0;
    map.iter().step_by(sy).fold(0, |acc, row| {
        let idx = x % row.len();
        let item = row[idx];
        x += sx;
        return acc + (item as i64);
    })
}

pub fn part_one(input: &str) -> String {
    let map = parse_input(input);
    let result = count_trees_hit(&map, 3, 1);
    return result.to_string()
}

pub fn part_two(input: &str) -> String {
    let map = parse_input(input);
    let slopes: [[usize; 2]; 5] = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let result = slopes.iter().fold(1, |acc, [sx, sy]| acc * count_trees_hit(&map, *sx, *sy));
    return result.to_string()
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let lines = input.split_whitespace();
    let output: Vec<Vec<i32>> = lines
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Invalid charcter in input {:?}", c),
                })
                .collect()
        })
        .collect();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d03_parsing_works() {
        let input = "..##.......\n\
            #...#...#..\n\
            .#....#..#.";
        let expected: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0],
        ];
        let parsed = parse_input(input);
        assert_eq!(parsed, expected)
    }

    #[test]
    fn d03_test_part_one() {
        let input = "..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#";
        let output = part_one(input);
        assert_eq!(output, "7")
    }

    #[test]
    fn d03_test_part_two() {
        let input = "..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#";
        let output = part_two(input);
        assert_eq!(output, "336")
    }
}
