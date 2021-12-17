use std::collections::HashMap;
use std::iter;

fn extender(init: String, dict: HashMap<String, String>, limit: u32) -> String {
    if limit == 0 {
        return init;
    }
    let res = init.chars().zip(init.chars().skip(1))
        .map(|pair| format!("{}{}", pair.0, dict.get(format!("{}{}", pair.0, pair.1).as_str()).unwrap()))
        .chain(iter::once(init.chars().last().unwrap().to_string()))
        .collect::<String>();
    extender(res, dict, limit - 1)
}

fn task01(init: String, dict: HashMap<String, String>) -> u32 {
    let mut occurrences = HashMap::new();
    let res = extender(init, dict, 10);
    res.chars().for_each(
        |ch| { *occurrences.entry(ch).or_insert(0) += 1 }
    );
    occurrences.iter().map(|(_, val)| val).max().unwrap() -
    occurrences.iter().map(|(_, val)| val).min().unwrap()
}

fn fast_extender(input: &mut HashMap<String, u64>, dict: HashMap<String, String>, limit: u32) -> &mut HashMap<String, u64> {
    if limit == 0 {
        return input;
    }
    input.clone().into_iter().filter(|(_, val)| *val != 0).for_each(
        |(key, val)| {
            let add = dict.get(&key).unwrap();
            *input.get_mut(&key).unwrap() -= val;
            *input.entry(format!("{}{}", key.chars().next().unwrap(), add)).or_insert(0) += val;
            *input.entry(format!("{}{}", add, key.chars().last().unwrap())).or_insert(0) += val;
        });
    fast_extender(input, dict, limit - 1)
}


fn task02(init: String, dict: HashMap<String, String>) -> u64 {
    let mut res: HashMap<String, u64> = init.chars().zip(init.chars().skip(1)).map(|pair| (format!("{}{}", pair.0, pair.1), 1)).collect();
    fast_extender(&mut res, dict,40);

    let mut occurrences: HashMap<char, u64> = HashMap::new();
    res.into_iter().for_each(|(key, val)|
        *occurrences.entry(key.chars().next().unwrap()).or_default() += val
    );
    *occurrences.entry(init.chars().last().unwrap()).or_default() += 1;

    occurrences.iter().map(|(_, val)| val).max().unwrap() -
    occurrences.iter().map(|(_, val)| val).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> (String, HashMap<String, String>) {
        let parsed = parse("resources/day14.in");
        let init = parsed[0].clone();
        let dict = parsed.into_iter().skip(2).map(|line|
            if let [pair, res] = line.split(" -> ").collect::<Vec<&str>>().as_slice() {
                (pair.to_string(), res.to_string())
            } else { panic!() })
            .collect::<HashMap<String, String>>();
        (init, dict)
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec.0, vec.1);
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data();
        let result = task02(vec.0, vec.1);
        println!("task02: {}", result)
    }
}