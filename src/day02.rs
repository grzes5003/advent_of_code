

fn task01(input: Vec<String>) -> i32 {
    let mut x = 0;
    let mut y = 0;

    input.into_iter().for_each(
        |line| {
            let s = line.split(' ').into_iter().map(|word| word.trim().to_string()).collect::<Vec<String>>();
            match s.get(0) {
                Some(dir) => match dir.as_str() {
                    "forward" => x += s.get(1).unwrap().parse::<i32>().unwrap(),
                    "up" => y -= s.get(1).unwrap().parse::<i32>().unwrap(),
                    "down" => y += s.get(1).unwrap().parse::<i32>().unwrap(),
                    _ => ()
                },
                None => ()
            }
        }
    );
    x * y
}

fn task02(input: Vec<String>) -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    input.into_iter().for_each(
        |line| {
            let s = line.split(' ').into_iter().map(|word| word.trim().to_string()).collect::<Vec<String>>();
            match s.get(0) {
                Some(dir) => match dir.as_str() {
                    "forward" => {
                        x += s.get(1).unwrap().parse::<i32>().unwrap();
                        y += aim * s.get(1).unwrap().parse::<i32>().unwrap();
                    },
                    "up" => aim -= s.get(1).unwrap().parse::<i32>().unwrap(),
                    "down" => aim += s.get(1).unwrap().parse::<i32>().unwrap(),
                    _ => ()
                },
                None => ()
            }
        }
    );
    x * y
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day02.txt")
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data();
        let result = task02(vec);
        println!("task01: {}", result)
    }
}
