use std::cmp;
use std::collections::HashSet;

#[derive(Debug,PartialEq)]
enum Action {
    Coord(u32, u32),
    Fold(u32, bool)
}

fn tmp_printer(input: &HashSet<(u32,u32)>) -> String {
    let res = input.iter().fold((0u32,0u32), |accum, next|
        (cmp::max(accum.0, next.0) as u32, cmp::max(accum.1, next.1) as u32)
    );
    println!("{:?}", res);
    (0..=res.1).into_iter().map(|y| {
        format!("{}\n", (0..=res.0).into_iter().map(|x| if input.contains(&(x,y)) {"#"} else {"."}).collect::<String>())
    }).collect()
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn folder(input: HashSet<(u32,u32)>, actions: Vec<(u32, bool)>) -> HashSet<(u32, u32)> {
    if actions.is_empty() {
        return input;
    }
    let action = actions[0];
    let (tab1, mut tab2): (_, HashSet<_>) = input.into_iter()
        .partition(|(x,y)| if action.1 {*y > action.0} else {*x > action.0});

    let res = tab1.into_iter().map(|(x,y)| {
        if action.1 {
            (x, action.0 - i32::abs(action.0 as i32 - y as i32) as u32)
        } else {
            (action.0 - i32::abs(action.0 as i32 - x as i32) as u32, y)
        }
    }).collect::<HashSet<(u32,u32)>>();
    res.into_iter().for_each(|x| {tab2.insert(x);});
    folder(tab2, actions[1..].to_vec())
}


fn task01(input: Vec<Action>) -> usize {
    let actions: Vec<(u32, bool)> = vec![input.iter()
        .filter(|&action| variant_eq(action, &Action::Fold(1,false)) )
        .map(|act| if let Action::Fold(val, dir) = act {(*val, *dir)} else {panic!()}).collect::<Vec<(u32, bool)>>()[0]];
    folder(input.iter()
               .filter(|&action| variant_eq(action, &Action::Coord(1,1)) )
               .map(|act| if let Action::Coord(x, y) = act {(*x,*y)} else {panic!()}).collect(),
            actions
    ).len()
}

fn task02(input: Vec<Action>) -> String {
    let res = folder(input.iter()
               .filter(|&action| variant_eq(action, &Action::Coord(1,1)) )
               .map(|act| if let Action::Coord(x, y) = act {(*x,*y)} else {panic!()}).collect(),
           input.iter()
               .filter(|&action| variant_eq(action, &Action::Fold(1,false)) )
               .map(|act| if let Action::Fold(val, dir) = act {(*val, *dir)} else {panic!()}).collect::<Vec<(u32, bool)>>()
    );
    tmp_printer(&res)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<Action> {
        let parsed = parse("resources/day13.in");
        parsed.into_iter().filter(|line| !line.is_empty()).map(|line| {
            if line.starts_with("fold") {
                if let [dir, wh] = line.split("=")
                    .map(|val| val.parse().unwrap_or_else(|_| if val.contains('y') {1} else {0}) )
                    .collect::<Vec<u32>>().as_slice() {
                    Action::Fold(*wh, *dir != 0)
                } else {panic!()}
            } else {
                if let [x, y] = line.split(",")
                    .map(|val| val.parse().unwrap())
                    .collect::<Vec<u32>>().as_slice() {
                    Action::Coord(*x, *y)
                } else {panic!()}
            }
        }).collect::<Vec<Action>>()
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
        println!("task01: \n{}", result)
    }
}