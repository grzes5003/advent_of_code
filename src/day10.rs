
enum Token {
    Open(char),
    Close(char),

}

fn score(token: char) -> u32 {
    match token {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!()
    }
}

fn valid(token1: Option<char>, token2: Option<char>) -> bool {
    match (token1, token2) {
        (Some('('), Some(')')) => true,
        (Some('{'), Some('}')) => true,
        (Some('['), Some(']')) => true,
        (Some('<'), Some('>')) => true,
        _ => false
    }
}

fn parser(chunk: String) -> u32 {
    let mut heap = vec![];

    let mut points = 0;
    for token in chunk.chars() {
        match token {
            token if "<[{(".contains(token) => heap.push(token),
            token if ">]})".contains(token) => {
                if !valid(heap.pop(), Some(token)) {
                    points = score(token);
                    break;
                }
            }
            _ => panic!()
        }
    }
    points
}

fn fixer(chunk: String) -> u64 {
    let mut heap = vec![];

    let mut points = 0u64;
    for token in chunk.chars() {
        match token {
            token if "<[{(".contains(token) => heap.push(token),
            token if ">]})".contains(token) => {heap.pop();},
            _ => panic!()
        }
    }
    heap.into_iter().rev().for_each(|token| {
        points *= 5;
        points = match token {
            '(' => points + 1,
            '[' => points + 2,
            '{' => points + 3,
            '<' => points + 4,
            _ => panic!()
        };
    });
    points
}

fn task01(input: Vec<String>) -> u32 {
    input.into_iter()
        .map(|expr| parser(expr))
        .reduce(|accum, score| accum + score).unwrap()
}

fn task02(input: Vec<String>) -> u64 {
    let mut res = input.into_iter()
        .filter(|expr| parser(expr.clone()) == 0)
        .map(|expr| fixer(expr))
        .collect::<Vec<u64>>();
    res.sort();
    res[res.len()/2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day10.in")
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