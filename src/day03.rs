use std::ops::BitXor;
use std::os::raw::c_int;

fn task01(input: Vec<String>) -> u32 {
    let len = (input.len() / 2) as u32;

    let res = input.iter()
        .map(|val|
            val.chars().map(|ch| ch.to_digit(10).unwrap()).collect::<Vec<u32>>()
        )
        .reduce(|accum, item|
            accum.iter()
                .zip(&item)
                .map(|(val1, val2)| val1 + val2)
                .collect()
        ).unwrap()
        .iter().map(|val| if *val > len { 1 } else { 0 }).collect::<Vec<u32>>();

    let res2 = res.iter()
        .map(|val| if *val == 1 { "0".to_string() } else { "1".to_string() })
        .collect::<Vec<_>>().join("");

    (isize::from_str_radix(
        res2.as_str(), 2)
        .unwrap() *
        isize::from_str_radix(
            res.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>().join("").as_str(), 2)
            .unwrap()) as u32
}


fn _task02(input: Vec<Vec<u8>>, idx: usize, version: bool) -> Vec<String> {
    let len = (input.len() as f32 )/ 2 as f32;
    if input.len() == 1 {
        return input.get(0).unwrap().into_iter().map(|val| val.to_string()).collect();
    }

    let find_more = match input.iter()
        .map(|val| {
            *val.get(idx).unwrap() as u16
        }
        )
        .reduce(|accum, item| accum + item).unwrap() as f32{
        val if val >= len => version as u8,
        val if val < len => !version as u8,
        _ => panic!("")
    };

    _task02(input.into_iter()
                .filter(|vec| *vec.get(idx).unwrap() != find_more)
                .collect(),
            idx + 1,
            version,
    )
}

fn task02(input: Vec<String>) -> u32 {
    let v_input = input.iter()
        .map(|val| val.chars().map(|ch| ch.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();

    let val1 = _task02(v_input.clone(), 0, true).join("");

    let val2 = _task02(v_input, 0, false).join("");

    (isize::from_str_radix(val1.as_str(), 2).unwrap()
        * isize::from_str_radix(val2.as_str(), 2).unwrap()) as u32
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day03.in")
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("task01: {}", result)
    }

    #[test]
    fn task02_example_test() {
        let vec = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        let result = task02(vec);
        println!("task02_example: {}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data();
        let result = task02(vec);
        println!("task02: {}", result)
    }
}