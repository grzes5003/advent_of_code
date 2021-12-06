use std::collections::HashMap;

fn task01(input: Vec<String>) -> u32 {
    let _input: Vec<u8> = input.get(0).unwrap()
        .split(",")
        .map(|ch| ch.parse::<u8>().unwrap())
        .collect();
    let mut h_map: HashMap<u8, u32> = HashMap::new();

    // initial count
    (0..=8).for_each(|val| {h_map.insert(val, 0);});
    _input.into_iter()
        .for_each(|val|
            *h_map.entry(val).or_insert(0) += 1
        );

    for _ in 0..80 {
        let mut last = 0;
        (1..=8).rev().for_each(|age| {
            let current = *h_map.get(&age).unwrap();
            *h_map.get_mut(&age).unwrap() = last;
            last = current;
        });
        let current = *h_map.get(&0).unwrap();
        *h_map.get_mut(&6).unwrap() += current;
        *h_map.get_mut(&8).unwrap() = current;
        *h_map.get_mut(&0).unwrap() = last;
    }
    h_map.into_iter().map(|(_, val)| val).reduce(|accum, val| accum + val).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day06.in")
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("task01: {}", result)
    }
}