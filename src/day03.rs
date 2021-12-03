#![allow(clippy::from_over_into)]

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<i32>> = Lazy::new(|| {
    utils::get_input_as_vec_with(INPUT_PATH, |line| i32::from_str_radix(line, 2).unwrap())
});
const INPUT_PATH: &str = "input/day03";

enum ZeroOrOne {
    Zero,
    One,
}

impl Into<usize> for ZeroOrOne {
    fn into(self) -> usize {
        match self {
            ZeroOrOne::Zero => 0,
            ZeroOrOne::One => 1,
        }
    }
}

fn power_consumption(input: &[i32], nbits: usize) -> usize {
    let (gamma, epsilon) = gamma_and_epsilon_rate(input, nbits);
    gamma * epsilon
}

fn life_support(input: &[i32], nbits: usize) -> usize {
    let o2 = oxygen_generator_rating(input, nbits);
    let co2 = co2_scrubber_rating(input, nbits);
    o2 * co2
}

fn oxygen_generator_rating(input: &[i32], nbits: usize) -> usize {
    use ZeroOrOne::*;
    match_criteria(input, nbits, (One, One, Zero))
}

fn co2_scrubber_rating(input: &[i32], nbits: usize) -> usize {
    use ZeroOrOne::*;
    match_criteria(input, nbits, (Zero, Zero, One))
}

fn match_criteria(
    input: &[i32],
    nbits: usize,
    (equal, one, zero): (ZeroOrOne, ZeroOrOne, ZeroOrOne),
) -> usize {
    let mut input = input.to_vec();
    let (equal, one, zero) = (equal.into(), one.into(), zero.into());

    for i in 0..nbits {
        let mut bit_pattern = get_bit_pattern(&input, nbits, true);
        bit_pattern.reverse();

        let pred = if bit_pattern[i] == 0 {
            equal
        } else if bit_pattern[i].is_positive() {
            one
        } else {
            zero
        };

        input = input
            .iter()
            .filter(|&&param| ((param >> (nbits - 1 - i)) & 1) as usize == pred)
            .copied()
            .collect::<Vec<i32>>();

        if input.len() == 1 {
            break;
        }
    }

    input[0] as usize
}

fn gamma_and_epsilon_rate(input: &[i32], nbits: usize) -> (usize, usize) {
    let bit_pattern = get_bit_pattern(input, nbits, false);
    let mut gamma = 0;

    for (i, bit) in bit_pattern.iter().enumerate() {
        gamma |= bit << i;
    }

    let mask = (1 << nbits) - 1;
    let epsilon = !gamma & mask;

    (gamma as usize, epsilon as usize)
}

fn get_bit_pattern(input: &[i32], nbits: usize, raw: bool) -> Vec<i32> {
    let mut bits = vec![0; nbits];

    for item in input {
        for (j, bit) in bits.iter_mut().enumerate() {
            let mut b = (item >> j) & 1;
            if b == 0 {
                b = -1;
            }

            *bit += b;
        }
    }

    if !raw {
        for bit in &mut bits {
            if bit.is_positive() {
                *bit = 1;
            } else {
                *bit = 0;
            }
        }
    }

    bits
}

pub fn part1() -> usize {
    power_consumption(&*INPUT, 12)
}

pub fn part2() -> usize {
    life_support(&*INPUT, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[i32] = &[
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    #[test]
    fn part1_example() {
        assert_eq!(power_consumption(INPUT, 5), 198);
    }

    #[test]
    fn part2_example() {
        assert_eq!(life_support(INPUT, 5), 230);
    }
}
