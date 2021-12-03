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
}