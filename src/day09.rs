

fn task01(input: Vec<Vec<u8>>) -> u32 {
    let mut res: u32 = 0;
    for idy in 1..input.len()-1 {
        for idx in 1..input.get(0).unwrap().len() {
            if
                input[idy][idx] < input[idy+1][idx] &&
                input[idy][idx] < input[idy][idx+1] &&
                input[idy][idx] < input[idy-1][idx] &&
                input[idy][idx] < input[idy][idx-1] {
                res += 1 + input[idy][idx] as u32;
            }
        }
    }
    res
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
        res.insert(0, vec![9; len+2]);
        res.push(vec![9; len+2]);
        res
    }

    #[test]
    fn task01_test() {
        let result = task01(input_data());
        println!("task01: {}", result)
    }
}