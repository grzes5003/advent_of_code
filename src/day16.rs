use std::iter::{Iterator, Peekable};
use std::ops::{Add, Shl, Sub};

fn to_binary(c: char) -> u8 {
    match c {
        '0' => 0b0000u8,
        '1' => 0b0001u8,
        '2' => 0b0010u8,
        '3' => 0b0011u8,
        '4' => 0b0100u8,
        '5' => 0b0101u8,
        '6' => 0b0110u8,
        '7' => 0b0111u8,
        '8' => 0b1000u8,
        '9' => 0b1001u8,
        'A' => 0b1010u8,
        'B' => 0b1011u8,
        'C' => 0b1100u8,
        'D' => 0b1101u8,
        'E' => 0b1110u8,
        'F' => 0b1111u8,
        _ => panic!(),
    }
}

fn get_bits(num: u8, _1: u8, _2: u8) -> u8 {
    ((1 << _1) - 1) & (num >> (_2 - 1))
}

fn get_val<I: Iterator<Item=u8>>(iter: &mut Peekable<I>, n: usize) -> u32 {
    iter.take(n)
        .enumerate()
        // .map(|tmp| {println!("{:?}", tmp); tmp})
        .fold(0u32,|acc, (pos,val)|
            if val == 1 {acc | (1 << (n-pos-1))} else {acc & !(1<<(n-pos-1))}
        )
}

enum Version {
    BITSv4 = 4,
    BITS,
}

enum TypeID {
    LITERAL = 4,
    OPERATOR,
}

#[derive(Debug)]
struct Header {
    version: u8,
    type_id: u8,
}

struct Packet {
    header: Header,
    body: Vec<char>,
}

fn reader<I: Iterator<Item=u8>>(iter: &mut Peekable<I>, result: &mut Vec<Header>, n: usize) -> ()
{
    let version = get_val(iter, 3);
    let type_id = get_val(iter, 3);
    println!("{:b} {:b}", version, type_id);
    result.push(Header{
        version: version as u8,
        type_id: type_id as u8
    });

    match type_id {
        // literal
        4 => {
            while let guard = get_val(iter, 1) {
                let lit = get_val(iter, 4);
                println!("literal {:b}:{:b}", guard, lit);
                if guard == 0 {break}
            }
        },
        // operator
        _ => {
            let i = get_val(iter, 1);
            println!("operator I={}", i);
            if i == 1 {
                let num = get_val(iter, 11);
                (0..num).for_each(|_| reader(iter, result, 10))
            } else {
                let len = get_val(iter, 15) as usize;
                let mut it = iter.take(len).collect::<Vec<u8>>().into_iter().peekable();
                while let Some(_) = it.peek() {
                    reader(&mut it, result, 10)
                }
            };
        }
    }

    // iter.take()
    ()
}

fn task01(input: Vec<String>) -> u32 {
    // i in 0..4 of 1 << i
    let mut res = vec![];
    reader(&mut input.iter().next()
        .unwrap()
        .chars()
        .map(|ch| {
            let val = to_binary(ch);
            (0..4u8).rev().map(|n| (val & 1 << n != 0) as u8).collect::<Vec<u8>>()
        }).flat_map(|vec| vec)
        .peekable(),
           &mut res,
        10
    );
    res.into_iter().map(|header| header.version as u32).sum()
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