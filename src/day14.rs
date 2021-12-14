use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::utils;

#[allow(clippy::type_complexity)]
static INPUT: Lazy<(Vec<u8>, Vec<((u8, u8), u8)>)> = Lazy::new(|| {
    utils::split_map(INPUT_PATH, "\n\n", std::convert::identity, |line| {
        let (pattern, replacement) = line.split_once(" -> ").unwrap();
        let pattern = pattern.as_bytes();
        (
            (pattern[0], pattern[1]),
            replacement.bytes().next().unwrap(),
        )
    })
});
const INPUT_PATH: &str = "input/day14";

fn elements_difference_after_n_steps(
    polymer_template: &[u8],
    pair_insertion: &[((u8, u8), u8)],
    n: usize,
) -> usize {
    let mut pair_occurrences = HashMap::<(u8, u8), usize>::new();
    let mut element_occurrences = HashMap::<u8, usize>::new();
    let rules: HashMap<(u8, u8), u8> = pair_insertion.iter().copied().collect();

    for pair in polymer_template.windows(2) {
        if let [lhs, rhs] = *pair {
            *pair_occurrences.entry((lhs, rhs)).or_default() += 1;
        }
    }

    for &polymer in polymer_template {
        *element_occurrences.entry(polymer).or_default() += 1;
    }

    for _ in 0..n {
        let keys: Vec<((u8, u8), usize)> = pair_occurrences
            .iter()
            .filter(|(_, &count)| count > 0)
            .map(|(&(lhs, rhs), &count)| ((lhs, rhs), count))
            .collect();

        for ((lhs, rhs), count) in keys {
            let replacement = rules[&(lhs, rhs)];

            *pair_occurrences.entry((lhs, replacement)).or_default() += count;
            *pair_occurrences.entry((replacement, rhs)).or_default() += count;
            *pair_occurrences.entry((lhs, rhs)).or_default() -= count;
            *element_occurrences.entry(replacement).or_default() += count;
        }
    }

    let max = element_occurrences
        .iter()
        .map(|(_, &count)| count)
        .max()
        .unwrap();
    let min = element_occurrences
        .iter()
        .map(|(_, &count)| count)
        .min()
        .unwrap();

    max - min
}

pub fn part1() -> usize {
    let (polymer_template, pair_insertion) = &*INPUT;
    elements_difference_after_n_steps(polymer_template, pair_insertion, 10)
}

pub fn part2() -> usize {
    let (polymer_template, pair_insertion) = &*INPUT;
    elements_difference_after_n_steps(polymer_template, pair_insertion, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const POLYMER_TEMPLATE: &[u8] = &[b'N', b'N', b'C', b'B'];
    const PAIR_INSERTION: &[((u8, u8), u8)] = &[
        ((b'C', b'H'), b'B'),
        ((b'H', b'H'), b'N'),
        ((b'C', b'B'), b'H'),
        ((b'N', b'H'), b'C'),
        ((b'H', b'B'), b'C'),
        ((b'H', b'C'), b'B'),
        ((b'H', b'N'), b'C'),
        ((b'N', b'N'), b'C'),
        ((b'B', b'H'), b'H'),
        ((b'N', b'C'), b'B'),
        ((b'N', b'B'), b'B'),
        ((b'B', b'N'), b'B'),
        ((b'B', b'B'), b'N'),
        ((b'B', b'C'), b'B'),
        ((b'C', b'C'), b'N'),
        ((b'C', b'N'), b'C'),
    ];

    #[test]
    fn part1_example() {
        assert_eq!(
            elements_difference_after_n_steps(POLYMER_TEMPLATE, PAIR_INSERTION, 10),
            1588
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            elements_difference_after_n_steps(POLYMER_TEMPLATE, PAIR_INSERTION, 40),
            2188189693529
        );
    }
}
