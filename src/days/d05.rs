fn decode_boarding_pass(pass: &str) -> (i32, i32) {
    // here as a monument to my folly
    // let mut row_cur = 0;
    // let mut row_gap = 128;
    // let mut col_cur = 0;
    // let mut col_gap = 8;
    // let chars = pass.chars();
    // for char in chars {
    //     match char {
    //         'F' => { row_gap = row_gap / 2; }
    //         'B' => {
    //             row_gap = row_gap / 2;
    //             row_cur += row_gap;
    //         }
    //         'L' => { col_gap = col_gap / 2; }
    //         'R' => {
    //             col_gap = col_gap / 2;
    //             col_cur += col_gap;
    //         }
    //         _ => panic!("Unrecognized character {}", char),
    //     }
    // };
    let row_bin = pass.chars().collect::<Vec<char>>()[..7]
        .iter().map(|&c| if c == 'F' { '0' } else { '1' }).collect::<String>();
    let col_bin = pass.chars().collect::<Vec<char>>()[7..]
        .iter().map(|&c| if c == 'L' { '0' } else { '1' }).collect::<String>();
    let row = i32::from_str_radix(&row_bin, 2).unwrap();
    let col = i32::from_str_radix(&col_bin, 2).unwrap();
    return (row, col);
}

pub fn part_one(input: &str) -> String {
    input.split_whitespace().map(|pass| {
        let (row, col) = decode_boarding_pass(pass);
        return row * 8 + col;
    }).max().unwrap().to_string()
}

pub fn part_two(input: &str) -> String {
    let mut id_list: Vec<i32> = input.split_whitespace().map(|pass| {
        let (row, col) = decode_boarding_pass(pass);
        return row * 8 + col;
    }).collect();
    id_list.sort();
    for (pos, el) in id_list.iter().enumerate() {
        if pos == id_list.len() - 1 {
            panic!("no seat found");
        }
        if id_list[pos + 1] == el + 2 {
            return (el + 1).to_string();
        }
    }
    panic!("no seat found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d05_decode_checks() {
        assert_eq!(decode_boarding_pass("FBFBBFFRLR"), (44, 5));
        assert_eq!(decode_boarding_pass("BFFFBBFRRR"), (70, 7));
        assert_eq!(decode_boarding_pass("FFFBBBFRRR"), (14, 7));
        assert_eq!(decode_boarding_pass("BBFFBBFRLL"), (102, 4));
    }
}