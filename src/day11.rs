
#[derive(Debug, Clone)]
struct Board {
    values: Vec<u8>,
}

fn tmp_printer(input: &Vec<Vec<u8>>) -> String {
    input.iter().map(|line|
        format!("{}\n", line.iter().map(|ch| ch.to_string()).collect::<String>())
    ).collect()
}


fn task01(input: &mut Vec<Vec<u8>>, limit: u8) -> u32 {
    let mut flashes = 0u32;
    for _ in 0..limit {
        for idy in 1..input.len() - 1 {
            for idx in 1..input.get(0).unwrap().len() - 1 {
                input[idy][idx] += 1;
            }
        };
        loop {
            let delta_flashes = flashes;
            for idy in 1..input.len() - 1 {
                for idx in 1..input.get(0).unwrap().len() - 1 {
                    if input[idy][idx] > 9 {
                        flashes += 1;
                        input[idy][idx] = 0;
                        input[(idy - 1)..=(idy + 1)]
                            .iter_mut()
                            .for_each(
                                |line| {
                                    line[(idx - 1)..=(idx + 1)].iter_mut().for_each(|val|
                                        if *val != 0 { *val += 1 }
                                    )
                                });
                    }
                }
            }
            if delta_flashes == flashes {
                break;
            }
        }
    }
    flashes
}


fn task02(input: &mut Vec<Vec<u8>>) -> u32 {
    let mut iter = 1u32;
    loop {
        let mut flashes = 0u32;
        for idy in 1..input.len() - 1 {
            for idx in 1..input.get(0).unwrap().len() - 1 {
                input[idy][idx] += 1;
            }
        };
        loop {
            let delta_flashes = flashes;
            for idy in 1..input.len() - 1 {
                for idx in 1..input.get(0).unwrap().len() - 1 {
                    if input[idy][idx] > 9 {
                        flashes += 1;
                        input[idy][idx] = 0;
                        input[(idy - 1)..=(idy + 1)]
                            .iter_mut()
                            .for_each(
                                |line| {
                                    line[(idx - 1)..=(idx + 1)].iter_mut().for_each(|val|
                                        if *val != 0 { *val += 1 }
                                    )
                                });
                    }
                }
            }
            if delta_flashes == flashes {
                break;
            }
        }
        if flashes as usize == (input.len() - 2) * (input[0].len() - 2) {
            return iter
        }
        iter += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<Vec<u8>> {
        let parsed = parse("resources/day11.in");
        let len = parsed.get(0).unwrap().len();
        let mut res = parsed
            .into_iter()
            .map(|line| {
                [
                    vec![0],
                    line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect::<Vec<u8>>(),
                    vec![0]
                ].concat()
            })
            .collect::<Vec<Vec<u8>>>();
        res.insert(0, vec![0; len + 2]);
        res.push(vec![0; len + 2]);
        res
    }

    #[test]
    fn task01_test() {
        let mut vec = input_data();
        let result = task01(&mut vec, 100);
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let mut vec = input_data();
        let result = task02(&mut vec);
        println!("task02: {}", result)
    }
}