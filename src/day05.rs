use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::{Range, RangeInclusive};

#[derive(Debug)]
struct Vector {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Vector {
    pub fn from_vec(input: Vec<u32>) -> Vector {
        Vector {
            x1: *input.get(0).unwrap(),
            y1: *input.get(1).unwrap(),
            x2: *input.get(2).unwrap(),
            y2: *input.get(3).unwrap(),
        }
    }

    fn vert_or_hor(&self) -> bool {
        &self.x1 == &self.x2 || &self.y1 == &self.y2
    }

    fn is_diag(&self) -> bool {
        (self.x2 as i32 - self.x1 as i32).abs() == (self.y2 as i32 - self.y1 as i32).abs()
    }

    fn get_line(&self) -> Vec<Point> {
        match (&self.x1 == &self.x2, &self.y1 == &self.y2) {
            (true, _) => ord_range(self.y1, self.y2).map(|val| Point {
                x: self.x1,
                y: val,
            }).collect(),
            (_, true) => ord_range(self.x1, self.x2).map(|val| Point {
                x: val,
                y: self.y1,
            }).collect(),
            _ => ord_range(self.x1, self.x2)
                .zip(ord_range(self.y1, self.y2))
                .map(|(x, y)| Point { x, y })
                .collect()
        }
    }
}

fn ord_range(val1: u32, val2: u32) -> Box<dyn Iterator<Item=u32>> {
    match val1 < val2 {
        true => Box::new(val1..=val2),
        false => Box::new((val2..=val1).rev()),
    }
}

fn parse_vec(input: Vec<String>) -> Vec<Vector> {
    input.into_iter()
        .map(|line|
            line.split(" -> ")
                .map(|xy| xy.split(","))
                .flat_map(|val| val)
                .map(|item| item.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        ).map(|items| Vector::from_vec(items))
        .collect::<Vec<Vector>>()
}

fn solution(input: Vec<Vector>) -> u32 {
    let mut h_points: HashMap<Point, bool> = HashMap::new();
    input.into_iter()
        .map(|vec| vec.get_line())
        .flat_map(|val| val)
        .for_each(|point| {
            match h_points.contains_key(&point) {
                true => *h_points.get_mut(&point).unwrap() = true,
                false => { h_points.insert(point, false); }
            }
        });
    h_points.iter()
        .fold(0, |accum, entry| if *entry.1 { accum + 1 } else { accum })
}

fn task01(input: Vec<Vector>) -> u32 {
    let _input: Vec<Vector> = input.into_iter()
        .filter(|vec| vec.vert_or_hor())
        .collect();
    solution(_input)
}

fn task02(input: Vec<Vector>) -> u32 {
    let _input: Vec<Vector> = input.into_iter()
        .filter(|vec| vec.is_diag() || vec.vert_or_hor())
        .collect();
    solution(_input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day05.in")
    }

    #[test]
    fn parse_test() {
        let vec = parse_vec(input_data());
        println!("vec: {:?}", vec)
    }

    #[test]
    fn task01_test() {
        let vec = parse_vec(input_data());
        let result = task01(vec);
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let vec = parse_vec(input_data());
        let result = task02(vec);
        println!("task02: {}", result)
    }
}