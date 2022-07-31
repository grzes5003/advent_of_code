use std::collections::HashSet;

const BOARD_LEN: u8 = 10;

#[derive(Debug)]
struct Player {
    position: u8,
    score: u64,
}

fn task01(positions: (u8, u8)) -> u64 {
    let mut player1 = Player {
        position: positions.0,
        score: 0,
    };
    let mut player2 = Player {
        position: positions.1,
        score: 0,
    };

    let mut turns = 0..;
    while let Some(turn) = turns.next() {
        let player = match turn % 2 {
            0 => (&mut player1, &player2),
            _ => (&mut player2, &player1)
        };
        if player.1.score >= 1000 {
            return player.0.score * turn as u64 * 3;
        }
        let score = (player.0.position as u64
            + (turn * 3..).take(3).map(|val| val % 100 + 1).sum::<u64>())
            % BOARD_LEN as u64;
        player.0.score += if score == 0 { 10 } else { score };
        player.0.position = score as u8;
    }
    panic!()
}

fn rec(boards: &mut Vec<(Player, Player)>, score: (u64,u64), perm: &Vec<i32>) -> u64 {
    let _boards: Vec<(Player,Player)> = Vec::new();
    boards.into_iter()
        .map(|(p1, p2)| {
            perm.iter().map(|dice| {
                // Player {
                //     position: p1.position + ,
                //     score: 0
                // }
                // _boards.push({})
            })
        });
    1
}

fn perm() -> Vec<(i32,i32)> {
    const LIMIT: i32 = 3;
    let stuff = (1..=LIMIT)
        .flat_map(|num| std::iter::repeat(num).zip(0..LIMIT))
        .flat_map(|num| std::iter::repeat(num).zip(0..LIMIT))
        .map(|item| item.0.0 + item.0.1 + item.1)
        // .collect::<HashSet<i32>>()
        // .into_iter()
        .collect::<Vec<i32>>();
    stuff.iter()
        .flat_map(|num| std::iter::repeat(*num).zip(stuff.clone()))
        .collect()
}

fn task02(positions: (u8, u8), ) -> u64 {
    let perm = perm();
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> (u8, u8) {
        let res = parse("resources/day21.in")
            .into_iter()
            .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
            .collect::<Vec<u8>>();
        (res[0], res[1])
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("perm {:?}", perm());
        println!("task01: {:?}", result)
    }
}