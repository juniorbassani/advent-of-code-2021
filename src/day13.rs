use core::panic;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| fs::read_to_string(INPUT_PATH).unwrap());
static COORDINATES: Lazy<Vec<(usize, usize)>> = Lazy::new(|| {
    INPUT
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
});
static FOLD_INSTRUCTIONS: Lazy<Vec<(u8, usize)>> = Lazy::new(|| {
    INPUT
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let coordinate = line.trim_start_matches("fold along ");
            (
                coordinate.bytes().next().unwrap(),
                coordinate[2..].parse().unwrap(),
            )
        })
        .collect()
});
const INPUT_PATH: &str = "input/day13";

fn visible_dots_after_folding(
    coordinates: &[(usize, usize)],
    fold_instructions: &[(u8, usize)],
) -> (HashSet<(usize, usize)>, usize) {
    let mut coordinates: HashSet<_> = coordinates.iter().copied().collect();

    for &(direction, split) in fold_instructions {
        let folded: Vec<_> = coordinates
            .iter()
            .filter(|&&entry| beyond_split(direction, split, entry))
            .map(|&entry| fold(direction, split, entry))
            .collect();

        coordinates.retain(|&entry| !beyond_split(direction, split, entry));
        coordinates.extend(folded);
    }

    let len = coordinates.len();

    (coordinates, len)
}

fn beyond_split(direction: u8, split: usize, (x, y): (usize, usize)) -> bool {
    if direction == b'x' {
        x > split
    } else {
        y > split
    }
}

fn fold(direction: u8, split: usize, (x, y): (usize, usize)) -> (usize, usize) {
    if direction == b'x' {
        (2 * split - x, y)
    } else {
        (x, 2 * split - y)
    }
}

pub fn part1() -> usize {
    visible_dots_after_folding(&*COORDINATES, &FOLD_INSTRUCTIONS[..1]).1
}

pub fn part2() {
    let (coords, _) = visible_dots_after_folding(&*COORDINATES, &*FOLD_INSTRUCTIONS);
    let width = coords.iter().map(|&(x, _)| x).max().unwrap();
    let height = coords.iter().map(|&(_, y)| y).max().unwrap();
    let mut map = vec![vec![b'.'; width + 1]; height + 1];
    for (x, y) in coords {
        map[y][x] = b'#';
    }
    for row in map {
        for entry in row {
            print!("{}", char::from(entry));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COORDINATES: &[(usize, usize)] = &[
        (6, 10),
        (0, 14),
        (9, 10),
        (0, 3),
        (10, 4),
        (4, 11),
        (6, 0),
        (6, 12),
        (4, 1),
        (0, 13),
        (10, 12),
        (3, 4),
        (3, 0),
        (8, 4),
        (1, 10),
        (2, 14),
        (8, 10),
        (9, 0),
    ];
    const FOLD_INSTRUCTIONS: &[(u8, usize)] = &[(b'y', 7), (b'x', 5)];

    #[test]
    fn part1_example() {
        assert_eq!(
            visible_dots_after_folding(COORDINATES, &FOLD_INSTRUCTIONS[..1]).1,
            17
        );
        assert_eq!(
            visible_dots_after_folding(COORDINATES, FOLD_INSTRUCTIONS).1,
            16
        );
    }
}
