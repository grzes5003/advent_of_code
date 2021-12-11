use std::cmp::Reverse;
use std::collections::HashSet;

fn task01(input: Vec<Vec<u8>>) -> u32 {
    let mut res: u32 = 0;
    for idy in 1..input.len() - 1 {
        for idx in 1..input.get(0).unwrap().len() {
            if
                input[idy][idx] < input[idy + 1][idx] &&
                input[idy][idx] < input[idy][idx + 1] &&
                input[idy][idx] < input[idy - 1][idx] &&
                input[idy][idx] < input[idy][idx - 1] {
                res += 1 + input[idy][idx] as u32;
            }
        }
    }
    res
}

fn rec(yx: (usize, usize), input: &Vec<Vec<u8>>, result: &mut HashSet<(usize,usize)>) {
    result.insert(yx);
    let curr = input[yx.0][yx.1];
    if curr < input[yx.0][yx.1+1] && input[yx.0][yx.1+1] != 9 {
        rec((yx.0, yx.1+1), &input, result)
    }
    if curr < input[yx.0+1][yx.1] && input[yx.0+1][yx.1] != 9 {
        rec((yx.0+1, yx.1), &input, result)
    }
    if curr < input[yx.0][yx.1-1] && input[yx.0][yx.1-1] != 9 {
        rec((yx.0, yx.1-1), &input, result)
    }
    if curr < input[yx.0-1][yx.1] && input[yx.0-1][yx.1] != 9 {
        rec((yx.0-1, yx.1), &input, result)
    }
}

fn task02(input: Vec<Vec<u8>>) -> usize {
    let mut yx: Vec<(usize, usize)> = vec![];

    for idy in 1..input.len() - 1 {
        for idx in 1..input.get(0).unwrap().len() {
            if
                input[idy][idx] < input[idy + 1][idx] &&
                input[idy][idx] < input[idy][idx + 1] &&
                input[idy][idx] < input[idy - 1][idx] &&
                input[idy][idx] < input[idy][idx - 1] {
                yx.push((idy, idx))
            }
        }
    }

    let mut r = yx.into_iter().map(|_yx| {
        let mut res = HashSet::new();
        rec(_yx, &input, &mut res);
        res.len()
    }).collect::<Vec<usize>>();
    r.sort_by_key(|w| Reverse(*w));
    r.into_iter().take(3).reduce(|acc, val| acc * val).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<Vec<u8>> {
        let parsed = parse("resources/day09.in");
        let len = parsed.get(0).unwrap().len();
        let mut res = parsed
            .into_iter()
            .map(|line| {
                [
                    vec![9],
                    line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect::<Vec<u8>>(),
                    vec![9]
                ].concat()
            })
            .collect::<Vec<Vec<u8>>>();
        res.insert(0, vec![9; len + 2]);
        res.push(vec![9; len + 2]);
        res
    }

    #[test]
    fn task01_test() {
        let result = task01(input_data());
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let result = task02(input_data());
        println!("task01: {}", result)
    }
}