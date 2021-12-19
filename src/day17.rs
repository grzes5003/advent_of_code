use std::iter::Repeat;

#[derive(Debug, PartialEq)]
enum Status {
    InScope = 0,
    HIT = 1,
    OutOfScope = 2,
}

#[derive(Debug)]
struct Ocean {
    probe: (i32, i32),
    target: (i32, i32, i32, i32),
    vel: (i32, i32)
}

impl Ocean {
    pub fn step(&mut self) {
        self.probe.0 += self.vel.0;
        self.probe.1 += self.vel.1;

        self.vel.0 = (self.vel.0.abs() - 1) * self.vel.0.signum();
        self.vel.1 = self.vel.1 - 1;
    }

    pub fn check_status(&self) -> Status {
        match self.probe {
            (x,y) if self.target.0 <= x && self.target.1 >= x && self.target.2 <= y && self.target.3 >= y
            => Status::HIT,
            (x,y) if self.target.1 >= x && i32::min(self.target.2, 0) <= y
            => Status::InScope,
            _ => Status::OutOfScope
        }
    }

    pub fn reset(&mut self, vel: (i32,i32)) {
        self.probe = (0,0);
        self.vel = vel;
    }
}


fn task01(coords: (i32, i32, i32, i32)) -> i32 {
    let mut ocean = Ocean {
        probe: (0,0),
        target: coords,
        vel: (1,1)
    };
    let limit = 150;
    let val = (0..limit)
        .flat_map(|num| std::iter::repeat(num).zip((0..limit)));
    let mut glob_max = 0;
    for xy in val {
        ocean.reset(xy);
        let mut max = 0;
        while let Status::InScope = ocean.check_status() {
            ocean.step();
            max = i32::max(max, ocean.probe.1);
        };
        if ocean.check_status() == Status::HIT {
            glob_max = i32::max(max, glob_max);
        }
    }
    glob_max
}

fn task02(coords: (i32, i32, i32, i32)) -> i32 {
    let mut ocean = Ocean {
        probe: (0,0),
        target: coords,
        vel: (1,1)
    };
    let limit = 1000;
    let val = (0..limit)
        .flat_map(|num| std::iter::repeat(num).zip((-limit..limit)));
    let mut counter = 0;
    for xy in val {
        ocean.reset(xy);
        while let Status::InScope = ocean.check_status() {
            ocean.step();
        };
        if ocean.check_status() == Status::HIT {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> (i32, i32, i32, i32) {
        let res = parse("resources/day17.in")
            .iter()
            .next().unwrap()
            .strip_prefix("target area: ").unwrap()
            .split(", ")
            .map(|s| {
                if let [_, nums] = s.split("=")
                    .collect::<Vec<&str>>()
                    .as_slice()
                {
                    nums.split("..")
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<i32>>()
                } else {panic!()}
            }).flat_map(|vals| vals)
            .collect::<Vec<i32>>();

        (res[0], res[1], res[2], res[3])
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("task01: {:?}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data();
        let result = task02(vec);
        println!("task02: {:?}", result)
    }
}