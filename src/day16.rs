use std::iter::{Iterator,Peekable};

enum Version {
    BITSv4 = 4,
    BITSv6 = 6,
}

enum TypeID {
    LITERAL = 4,
    OPERATOR = 6,
}

struct Packet {
    version: Version,
    type_id: TypeID,
    body: Vec<char>
}

fn reader<I: Iterator<Item=char>>(iter: Peekable<I>, result: String) -> ()
{
    // iter.take()
    ()
}

fn task01(input: Vec<String>) -> String {
    reader(input.iter().next().unwrap().chars().peekable(), "".to_string());
    "".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day16.in")
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("task01: {}", result)
    }
}