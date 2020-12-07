// By Omar and Gautam.
use anyhow::{bail, ensure, Context};
use std::collections::{HashMap, HashSet};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("./src/day5/input.txt").context("Cannot read input")?;

    let dirs: Vec<(Vec<Direction>, Vec<Direction>)> = input
        .lines()
        .map(|line| {
            let seat: &[char] = &line.chars().collect::<Vec<char>>()[0..7];
            let lr: &[char] = &line.chars().collect::<Vec<char>>()[7..];

            let dir = seat
                .iter()
                .map(|c| match c {
                    'F' => Direction::F,
                    'B' => Direction::B,
                    _ => panic!("cannot parse"),
                })
                .collect();

            let seat: Vec<_> = lr
                .iter()
                .map(|c| match c {
                    'L' => Direction::F,
                    'R' => Direction::B,
                    _ => panic!("cannot parse"),
                })
                .collect();

            (dir, seat)
        })
        .collect();

    let mut seat_ids = dirs
        .iter()
        .map(|d| part1(&d.0, 127) as usize * 8 + part1(&d.1, 7) as usize)
        .collect::<Vec<usize>>();

    let max = seat_ids.iter().max().unwrap();
    let mut hs = HashSet::new();
    dbg!(&max);
    for e in seat_ids.iter() {
        hs.insert(e);
    }

    for seat in 0..*max {
        if !hs.contains(&seat) {
            println!("{} Not found", seat);
        }
    }

    Ok(())
}

enum Direction {
    F,
    B,
}

fn part1(v: &[Direction], max: u8) -> u8 {
    let s = v
        .iter()
        .map(|e| match e {
            Direction::F => '0',
            Direction::B => '1',
        })
        .collect::<String>();

    return u8::from_str_radix(&s, 2).unwrap();
}

#[cfg(test)]
mod test {
    use crate::part1;
    use crate::Direction::*;

    #[test]
    fn t1() {
        assert_eq!(part1(&[F, B, F, B, B, F, F], 127), 44);
        assert_eq!(part1(&[B, F, F, F, B, B, F], 127), 70);
        assert_eq!(part1(&[F, F, F, B, B, B, F], 127), 14);
        assert_eq!(part1(&[B, B, F, F, B, B, F], 127), 102);

        assert_eq!(part1(&[B, B, B], 7), 7);
        assert_eq!(part1(&[B, F, F], 4), 4);
    }
}
