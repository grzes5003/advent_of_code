use std::borrow::Borrow;
use std::convert::TryInto;
use std::env::var;

const SIZE: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    values: Vec<u16>,
}

fn transpose(input: &Vec<u16>) -> Vec<u16> {
    let mut arr: Vec<u16> = vec![];
    (0..SIZE).for_each(|start|
        arr.extend(input.into_iter().skip(start).step_by(SIZE))
    );
    arr
}

fn check(vec: &Vec<u16>, target: &[u16]) -> bool {
    vec.chunks(SIZE)
        .map(|val| val.iter().all(|item| target.contains(item)))
        .any(|val| val)
}

fn parse_game_input(input: Vec<String>) -> (Vec<u16>, Vec<Board>) {
    let start = input.get(0).unwrap().split(",").map(|ch| ch.parse::<u16>().unwrap()).collect();

    let boards: Vec<Board> = input
        .into_iter().skip(1)
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .chunks(SIZE)
        .map(|chunk|
            Board {
                values: chunk.into_iter()
                    .map(|s|
                        s.split_whitespace().map(|x|
                            x.parse::<u16>().unwrap()
                        ).collect::<Vec<u16>>()
                    ).flat_map(|x| x)
                    .collect::<Vec<u16>>()
            }
        ).collect();
    (start, boards)
}

fn task01(input: Vec<String>) -> u16 {
    let (mut start, boards) = parse_game_input(input);
    let mut res = 0;
    for idx in SIZE..start.len() {
        if let Some(winner) = boards.clone()
            .into_iter()
            .skip_while(|board| !check(&board.values, &start[..idx]) && !check(&transpose(&board.values), &start[..idx]))
            .collect::<Vec<Board>>().first()
        {
            res = winner.clone().values
                .into_iter()
                .filter(|item| !start[..idx].contains(item))
                .sum::<u16>() * start.get(idx - 1)
                .unwrap();
            break;
        }
    }
    res
}


fn task02(input: Vec<String>) -> u16 {
    let (mut start, boards) = parse_game_input(input);
    let mut res = 0;
    let mut last = Board {values: vec![]};

    for idx in SIZE..start.len() {
        let loosers = boards.clone()
            .into_iter()
            .filter(|board| !check(&board.values, &start[..idx]) && !check(&transpose(&board.values), &start[..idx]))
            .collect::<Vec<Board>>();
        if loosers.is_empty() {
            return last.clone().values
                .into_iter()
                .filter(|item| !start[..idx].contains(item))
                .sum::<u16>() * start.get(idx - 1)
                .unwrap()
        }
        match loosers.last() {
            Some(looser) => last = looser.to_owned(),
            None => ()
        }
    };
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day04.in")
    }

    #[test]
    fn transpose_test() {
        let board = Board {
            values: (0..25).collect::<Vec<u16>>()
        };
        println!("{:?}", transpose(&board.values))
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
        println!("task02: {}", result)
    }
}