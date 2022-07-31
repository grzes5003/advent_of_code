use std::cmp::min;
use std::collections::HashMap;
use std::env::var;

type Point = (usize, usize);


fn _task01(board: &Vec<Vec<u8>>, point: Point, cost: u32, cache: &mut HashMap<Point, u32>) -> u32 {
    let dims = board.len();

    if point == (dims - 1, dims - 1) {
        return cost;
    }

    // down
    let down = if point.1 < dims - 1 {
        if cache.contains_key(&(point.0, point.1 + 1)) {
            Some(cache.get(&(point.0, point.1 + 1)).unwrap().to_owned())
        } else {
            let res = task01(board, (point.0, point.1 + 1), cost + board[point.0][point.1] as u32, cache);
            // cache.insert((point.0, point.1 + 1), res);
            Some(res)
        }
    } else { None };
    // right
    let right = if point.0 < dims - 1 {
        if cache.contains_key(&(point.0 + 1, point.1)) {
            Some(cache.get(&(point.0 + 1, point.1)).unwrap().to_owned())
        } else {
            let res = task01(board, (point.0 + 1, point.1), cost + board[point.0][point.1] as u32, cache);
            // cache.insert((point.0 + 1, point.1), res);
            Some(res)
        }
    } else { None };

    if down.unwrap_or(u32::MAX) <= right.unwrap_or(u32::MAX) {
        cache.entry((point.0 + 1, point.1)).or_insert(down.unwrap_or(u32::MAX));
        down.unwrap_or(u32::MAX)
    } else {
        cache.entry((point.0, point.1 + 1)).or_insert(right.unwrap_or(u32::MAX));
        right.unwrap_or(u32::MAX)
    }
}


fn task01(board: &Vec<Vec<u8>>, point: Point, cost: u32, cache: &mut HashMap<Point, u32>) -> u32 {
    let dims = board.len() - 1;
    if point == (dims, dims) {
        println!("ret cost {}", cost);
        return cost
    };
    println!("point: {:?}", point);

    let mut vec = Vec::new();
    if point.0 < dims {vec.push((point.0 + 1, point.1));}
    if point.1 < dims {vec.push((point.0, point.1 + 1));}

    let result = vec.into_iter().map(|_point| {
        let a = if cache.contains_key(&_point) {
            *cache.get(&_point).unwrap()
        } else {
            task01(&board, _point, cost + board[point.0][point.1] as u32, cache)
        };
        println!("{}", a);
        a
    }).min().unwrap();
    if cache.contains_key(&point) {
        println!("has: {:?}", point);
    }
    cache.insert(point, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<Vec<u8>> {
        let parsed = parse("resources/day15.in");
        parsed.into_iter()
            .map(|line|
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as u8)
                    .collect()
            )
            .collect()
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let mut map = HashMap::new();
        let result = task01(&vec, (0, 0), 0, &mut map);
        println!("task01: {}", result)
    }

    // #[test]
    // fn task02_test() {
    //     let vec = input_data();
    //     let result = task02(vec.0, vec.1);
    //     println!("task02: {}", result)
    // }
}