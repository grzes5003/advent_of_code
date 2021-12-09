#[derive(Debug)]
struct Screen {
    dict: Vec<String>,
    vals: Vec<String>,
}


fn task01(input: Vec<Screen>) -> u32 {
    println!("{:?}", input);

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
    fn task01_test() {
        let result = task01(input_data());
        println!("task01: {}", result)
    }
}