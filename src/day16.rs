use std::cmp::Ordering;
use std::fmt;
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

trait Payload: fmt::Debug {
    fn get_value(&self) -> u32;
}

#[derive(Debug)]
struct Value {
    val: u32
}

impl Payload for Value {
    fn get_value(&self) -> u32 {
        println!("Value: {:?}", self);
        self.val
    }
}

#[derive(Debug)]
struct Values {
    type_id: u8,
    packets: Vec<Packet>
}

impl Payload for Values {
    fn get_value(&self) -> u32 {
        println!("values: {:?}",  self.type_id);
        println!("packetsss: {:?}",  self.packets.iter().map(|pckt| pckt.get_value()).collect::<Vec<u32>>());
        match self.type_id {
            0 => {
                self.packets.iter().map(|pckt| pckt.get_value()).sum()
            }
            1 => {
                self.packets.iter().map(|pckt| pckt.get_value()).reduce(|accum, val| accum * val).unwrap()
            },
            2 => {
                self.packets.iter().map(|pckt| pckt.get_value()).min().unwrap()
            },
            3 => {
                self.packets.iter().map(|pckt| pckt.get_value()).max().unwrap()
            },
            5 => {
                (self.packets.get(0).unwrap().get_value()
                    .cmp(&self.packets.get(1).unwrap().get_value()) == Ordering::Greater) as u32
            }
            6 => {
                (self.packets.get(0).unwrap().get_value()
                    .cmp(&self.packets.get(1).unwrap().get_value()) == Ordering::Less) as u32
            }
            7 => {
                (self.packets.get(0).unwrap().get_value()
                    .cmp(&self.packets.get(1).unwrap().get_value()) == Ordering::Equal) as u32
            }
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Header {
    version: u8,
    type_id: u8,
}

#[derive(Debug)]
struct Packet {
    header: Header,
    // value | vec<>
    body: Box<dyn Payload>,
}

impl Payload for Packet {
    fn get_value(&self) -> u32 {
        // println!("packet: {:?}", self);
        self.body.get_value()
    }
}

fn reader<I: Iterator<Item=u8>>(iter: &mut Peekable<I>, result: &mut Vec<Packet>) -> ()
{
    println!("{:?}", result);
    let version = get_val(iter, 3);
    let type_id = get_val(iter, 3);
    let header = Header{
        version: version as u8,
        type_id: type_id as u8
    };

    match type_id {
        // literal
        4 => {
            let mut chunks: Vec<u32> = Vec::new();
            while let guard = get_val(iter, 1) {
                chunks.push(get_val(iter, 4));
                // println!("literal {:b}:{:b}", guard, chunks.iter().last().unwrap());
                if guard == 0 {break}
            }
            println!("chunks: {:?}", chunks);
            let res = chunks.into_iter()
                .rev()
                .enumerate()
                .fold(0u32, |acc, (idx, val)| acc | (val<<(idx*4)));
            // println!("res: {} {:b}", res, res);
            result.push(Packet {
                header,
                body: Box::new(Value {val: res})
            })
        },
        // operator
        _ => {
            let i = get_val(iter, 1);
            let mut res: Vec<Packet> = Vec::new();
            // println!("operator I={}", i);
            if i == 1 {
                let num = get_val(iter, 11);
                (0..num).for_each(|_| reader(iter, &mut res))
            } else {
                let len = get_val(iter, 15) as usize;
                let mut it = iter.take(len).collect::<Vec<u8>>().into_iter().peekable();
                while let Some(_) = it.peek() {
                    reader(&mut it, &mut res)
                }
            };
            println!("push {:?}",res);
            result.push(Packet {
                body: Box::new(Values{ type_id: header.type_id, packets: res }),
                header,
            })
        }
    }
}

fn task01(mut input: Vec<u8>) -> u32 {
    // i in 0..4 of 1 << i
    let mut res = Vec::new();
    reader(&mut input.into_iter().peekable(), &mut res);
    println!("{:?}", res);
    res.into_iter().map(|pckt| pckt.header.version as u32).sum()
}

fn task02(mut input: Vec<u8>) -> u32 {
    let mut res = Vec::new();
    reader(&mut input.into_iter().peekable(), &mut res);
    println!("=== {:?}", res);
    res.into_iter().map(|pckt| pckt.get_value()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<u8> {
        parse("resources/day16.in")
            .iter()
            .next()
            .unwrap()
            .chars()
            .map(|ch| {
                let val = to_binary(ch);
                (0..4u8).rev().map(|n| (val & 1 << n != 0) as u8).collect::<Vec<u8>>()
            }).flat_map(|vec| vec)
            .collect::<Vec<u8>>()
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01( vec);
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data();
        let result = task02(vec);
        println!("task02: {}", result)
    }
}