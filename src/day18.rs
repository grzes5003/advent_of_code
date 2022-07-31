use std::{error::Error, fmt};
use std::fmt::Debug;
use std::ops::Index;

pub trait Numeric {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}

enum Pair {
    L,
    R
}

#[derive(Debug)]
struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse the token")
    }
}

type Result<T> = std::result::Result<T, ParseError>;

trait Exp<T>: Debug {
    fn value(self) -> T;
    fn single(&self) -> bool;
    fn left(self) -> Box<dyn Exp<T>>;
    fn right(self) -> Box<dyn Exp<T>>;
    fn add_left(&mut self, value: T) -> &Self;
    fn add_right(&mut self, value: T) -> &Self;
}

struct Num<T: Numeric + std::fmt::Debug> {
    _1: T
}

impl<T: Numeric + std::fmt::Debug> Exp<T> for Num<T> {
    fn value(self) -> T { self._1 }

    fn single(&self) -> bool {
        true
    }

    fn left(self) -> Box<dyn Exp<T>> {
        unimplemented!()
    }

    fn right(self) -> Box<dyn Exp<T>> {
        unimplemented!()
    }

    fn add_left(&mut self, value: T) -> &Self {
        self._1 += value;
        self
    }

    fn add_right(&mut self, value: T) -> &Self {
        self._1 += value;
        self
    }
}

// impl<T: Numeric + std::fmt::Debug> Index<Pair> for Num<T> {
//     type Output = T;
//
//     fn index(&self, pair: Pair) -> &Self::Output {
//         match pair {
//             Pair::L => &self._1,
//             Pair::R => &self._1,
//         }
//     }
// }

impl<T: Numeric + std::fmt::Debug> fmt::Debug for Num<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self._1)
        // f.debug_tuple("")
        //     .field(&self._1)
        //     .finish()
    }
}

struct Token<T> {
    _1: Box<dyn Exp<T>>,
    _2: Box<dyn Exp<T>>
}

// impl<T> Index<Pair> for Token<T> {
//     type Output = Box<dyn Exp<T, Output=T>>;
//
//     fn index(&self, pair: Pair) -> &Self::Output {
//         match pair {
//             Pair::L => &self._1,
//             Pair::R => &self._2,
//         }
//     }
// }

impl<T> Exp<T> for Token<T> {
    fn value(self) -> T {
        todo!()
    }

    fn single(&self) -> bool {
        false
    }

    fn left(self) -> Box<dyn Exp<T>> {
        self._1
    }

    fn right(self) -> Box<dyn Exp<T>> {
        self._2
    }

    fn add_left(&mut self, value: T) -> &Self {
        self._2.add_left(value)
    }

    fn add_right(&mut self, value: T) -> &Self {
        self._2.add_right(value)
    }
}

impl<T> fmt::Debug for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self._1)
            .field(&self._2)
            .finish()
    }
}

fn parse_line<T: 'static + Numeric>(line: String) -> Result<Box<dyn Exp<T>>>
    where Num<u32>: Exp<T>
{
    let mut input = line.chars();
    let mut ops: Vec<char> = Vec::new();
    let mut nums: Vec<Box<dyn Exp<T>>> = Vec::new();

    loop {
        match input.next() {
            Some('[') => ops.push('['),
            Some(']') => {
                let val1 = nums.pop().unwrap();
                let val2 = nums.pop().unwrap();
                nums.push(Box::new(
                    Token {
                        _1: val2,
                        _2: val1
                    }
                ))
            },
            Some(val) if val.is_digit(10) => nums.push(Box::new(
                Num {
                    _1: val.to_digit(10).unwrap()
                }
            )),
            Some(',') => (),
            None => break,
            _ => panic!("Unexpected token")
        }
    }


    // if !line.starts_with('[') || !line.ends_with(']') {
    //     println!("err {:?}", line);
    //     return Err(ParseError)
    // }
    //
    // if let [_1, _2] = line
    //     .strip_prefix('[').unwrap()
    //     .strip_suffix(']').unwrap()
    //     .split(',').map(|s| s.to_string())
    //     .collect::<Vec<String>>().as_slice() {
    //     println!("{:?} {:?}", _1, _2)
    // }
    println!("inp: {}", line);
    println!("{:?}", nums);

    Err(ParseError)
}

fn reduce<T: Numeric>(val: &mut Box<dyn Exp<T>>) -> Box<dyn Exp<T>> {
    let mut current = val;
    let mut depth = 0u8;
    while !current.single() {

    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> () {
        println!("1");
        let res = parse("resources/day18_test.in")
            .into_iter()
            .map(|line| parse_line::<u32>(line))
            .collect::<Vec<Result<Box<dyn Exp<u32>>>>>();
    }

    #[test]
    fn task01_test() {
        let e = Token {
            _1: Box::new(Num {
                _1: 2u32
            }),
            _2: Box::new(Num {
                _1: 2u32
            })
        };
        let vec = input_data();
        // let result = task01(vec);
        // println!("task01: {:?}", result)
    }
}