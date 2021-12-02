
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(EnumString)]
enum Foo {
    forward,
    down,
    up
}

fn task01(input: Vec<String>) -> u32 {

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("/Users/xgg/CLionProjects/advent_of_code/resources/day02_task01.txt")
    }

    #[test]
    fn it_works() {
        let vec = input_data();
        vec.into_iter().map()

        println!("{}", vec.len())
    }
}
