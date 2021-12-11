fn task01(input: Vec<u8>, width: usize) -> u32 {
    println!("{:?}", input);

    1
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<u8> {
        let parsed = parse("resources/day09_2.in");
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
        res.insert(0, vec![9; parsed.get(0).unwrap().len()]);
        res.push(vec![9; parsed.get(0).unwrap().len()]);

        [
            vec![9; parsed.get(0).unwrap().len()],
            parse("resources/day09_2.in")
                .into_iter()
                .map(|line| {
                    [
                        vec![9],
                        line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect::<Vec<u8>>(),
                        vec![9]
                    ].concat()
                })
                .collect::<Vec<Vec<u8>>>(),
            vec![9; parsed.get(0).unwrap().len()]
        ].concat()
    }

    #[test]
    fn task01_test() {
        let result = task01(input_data(), 10);
        println!("task01: {}", result)
    }
}