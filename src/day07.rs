use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;


fn task01(input: Vec<u32>) -> u32 {

    let mut results: Vec<u32> = vec![];
    for x in 0..*input.iter().max().unwrap() {
        let val = results.push(input.iter()
            .map(|p| (x as i32 - *p as i32).abs() as u32)
            .sum::<u32>());
        if x >= 3 {
            if let [_1, _2, _3] = &results[(x as usize) - 2..] {
                if *_1 > *_2 && *_2 < *_3 {
                    return *_2 as u32;
                }
            }
        }
    }
    0
}


fn task02(input: Vec<u32>) -> u32 {

    let mut results: Vec<u32> = vec![];
    for x in 0..*input.iter().max().unwrap() {
        let val = results.push(input.iter()
            .map(|p| (1..=(x as i32 - *p as i32).abs() as u32).sum::<u32>())
            .sum::<u32>());
        if x >= 3 {
            if let [_1, _2, _3] = &results[(x as usize) - 2..] {
                if *_1 > *_2 && *_2 < *_3 {
                    return *_2 as u32;
                }
            }
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<u32> {
        parse("resources/day07.in")
            .get(0).unwrap().to_owned()
            .split(",")
            .map(|ch| ch.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()

    }

    #[test]
    fn task01_test() {
        let result = task01(input_data());
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let result = task02(input_data());
        println!("task02: {}", result)
    }
}