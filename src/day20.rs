use std::fmt::{Display, Formatter};

type Point = (isize, isize);
type Res = u32;

#[derive(Debug)]
struct Pic(Vec<Vec<u8>>, bool, bool);

impl Display for Pic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-------------\n")?;
        for line in self.0.iter() {
            for item in line {
                write!(f, "{}", match item {
                    1 => "#",
                    0 => ".",
                    _ => panic!("Cannot display")
                })?;
            }
            write!(f, "\n")?;
        };
        write!(f, "-------------\n")?;
        Ok(())
    }
}

impl Pic {
    fn _new_point(&self) -> u8 {
        if self.1 && self.2 {
            1
        } else {
            0
        }
    }

    fn bit2num(bits: Vec<u8>) -> usize {
        bits.into_iter().rev().enumerate()
            .fold(0 as Res, |acc, (idx, val)|
                acc + if val == 1 { u32::from(2 as u32).pow(idx as u32) } else { 0 }) as usize
    }

    fn get_num(&self, point: Point) -> usize {
        let (x, y) = point;
        let idxs = vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ];

        let result = idxs.into_iter()
            .map(|(x, y)| {
                if x < 0 || y < 0 {
                    return self._new_point();
                };
                match self.0.get(x as usize) {
                    Some(val_x) => match val_x.get(y as usize) {
                        Some(val_y) => *val_y,
                        None => self._new_point()
                    },
                    None => self._new_point()
                }
            }).collect::<Vec<u8>>();
        Pic::bit2num(result)
    }

    fn padding(&self) -> Pic {
        let dims = self.0.len();
        let mut res = self.0.iter().map(|line| {
            let mut line = line.clone();
            line.insert(0, self._new_point());
            line.push(self._new_point());
            line
        }).collect::<Vec<Vec<u8>>>();
        res.insert(0, vec![self._new_point(); dims + 2]);
        res.push(vec![self._new_point(); dims + 2]);
        Pic(res, self.1, self.2)
    }

    fn augment(&mut self, transform_matrix: &Vec<u8>) {
        let offset: isize = 1;
        let mut result = self.padding();
        for idx in 0..result.0.len() {
            for idy in 0..result.0.get(0).unwrap().len() {
                let (_idx, _idy) = (idx as isize - offset, idy as isize - offset);
                *result.0.get_mut(idx).unwrap().get_mut(idy).unwrap() =
                    *transform_matrix.get(self.get_num((_idx, _idy))).unwrap();
            }
        }
        self.2 = !self.2;
        self.0 = result.0;
    }

    fn count_bright(&self) -> Res {
        self.0.iter().map(|line|
            line.iter().fold(0 as Res, |acc, val| acc + *val as Res))
            .fold(0 as Res, |acc, val| acc + val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn str2vec(line: String) -> Vec<u8> {
        line.chars().map(|ch| match ch {
            '#' => 1,
            '.' => 0,
            ch => panic!("Cannot parse {}", ch)
        }).collect()
    }

    fn input_data() -> (Pic, Vec<u8>) {
        let parsed = parse("resources/day20_2.in");
        let ref_matrix = str2vec(parsed.get(0).unwrap().to_owned());
        let arr = parsed[2..].into_iter()
            .map(|line| str2vec(line.to_owned()))
            .collect();
        (Pic(arr,
             if *ref_matrix.get(0).unwrap() == 0 { false } else { true }, false),
         ref_matrix)
    }

    #[test]
    fn task01() {
        let (mut pic, ref_matrix) = input_data();
        pic.augment(&ref_matrix);
        pic.augment(&ref_matrix);
        println!("{}", pic.count_bright());
    }

    #[test]
    fn task02() {
        let (mut pic, ref_matrix) = input_data();
        (0..50).for_each(|_| pic.augment(&ref_matrix));
        println!("{}", pic.count_bright());
    }
}