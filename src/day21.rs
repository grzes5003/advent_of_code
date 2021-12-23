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
        println!("task01: {:?}", result)
    }
}