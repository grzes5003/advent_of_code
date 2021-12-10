//!
//! Encoding scheme:
//! position of bits
//!     0000
//!    1    2
//!    1    2
//!     3333
//!    4    5
//!    4    5
//!     6666
//! val |0,0,0,0,0,0,0|
//! idx  0,1,2,3,4,5,6
use std::collections::HashMap;

const DICT: [(u8, u8, u8); 10] = [
    (0, 6, (1 << 0) + (1 << 1) + (1 << 2) + (1 << 4) + (1 << 5) + (1 << 6)),
    (1, 2, (1 << 2) + (1 << 5)),
    (2, 5, (1 << 0) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 6)),
    (3, 5, (1 << 0) + (1 << 2) + (1 << 3) + (1 << 5) + (1 << 6)),
    (4, 4, (1 << 1) + (1 << 2) + (1 << 3) + (1 << 5)),
    (5, 5, (1 << 0) + (1 << 1) + (1 << 3) + (1 << 5) + (1 << 6)),
    (6, 6, (1 << 0) + (1 << 1) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6)),
    (7, 3, (1 << 0) + (1 << 2) + (1 << 5)),
    (8, 7, (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6)),
    (9, 6, (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 5) + (1 << 6)),
];

#[derive(Debug)]
struct Screen {
    dict: Vec<String>,
    vals: Vec<String>,
}


fn task01(input: Vec<Screen>) -> u32 {
    println!("{:?}", input);
    input.into_iter().map(
        |screen|
            screen.dict.into_iter()
    );
    1
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<Screen> {
        parse("resources/day08_2.in")
            .into_iter()
            .map(|line| {
                if let [dict, vals] =
                line.split("|")
                    .map(|part| part.split_whitespace()
                        .map(|s| s.to_string()).collect::<Vec<String>>()
                    )
                    .collect::<Vec<Vec<String>>>().as_slice() {
                    Screen {
                        dict: dict.to_vec(),
                        vals: vals.to_vec(),
                    }
                } else {
                    panic!("")
                }
            })
            .collect::<Vec<Screen>>()
    }

    #[test]
    fn task01_test2() {
        println!("task01: {:?}", DICT)
    }


    #[test]
    fn task01_test() {
        let result = task01(input_data());
        println!("task01: {}", result)
    }
}