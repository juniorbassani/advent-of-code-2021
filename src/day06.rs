use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<u8>> = Lazy::new(|| utils::parse_line(INPUT_PATH));
const INPUT_PATH: &str = "input/day06";

fn number_of_fishes_after_n_days(fish_list: &[u8], days: usize) -> usize {
    let mut fishes = [0; 9];
    for &fish in fish_list {
        fishes[fish as usize] += 1;
    }

    for _ in 0..days {
        let to_procreate = fishes[0];

        for i in 0..fishes.len() - 1 {
            fishes[i] = fishes[i + 1];
        }

        fishes[8] = to_procreate;
        fishes[6] += to_procreate;
    }

    fishes.into_iter().sum()
}

pub fn part1() -> usize {
    number_of_fishes_after_n_days(&*INPUT, 80)
}

pub fn part2() -> usize {
    number_of_fishes_after_n_days(&*INPUT, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = &[3, 4, 3, 1, 2];

    #[test]
    fn part1_example() {
        assert_eq!(number_of_fishes_after_n_days(INPUT, 18), 26);
        assert_eq!(number_of_fishes_after_n_days(INPUT, 80), 5934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(number_of_fishes_after_n_days(INPUT, 256), 26984457539);
    }
}
