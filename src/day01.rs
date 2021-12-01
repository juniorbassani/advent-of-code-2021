use crate::utils;
use once_cell::sync::Lazy;

static INPUT: Lazy<Vec<i32>> = Lazy::new(|| utils::get_input_as_vec::<i32>(INPUT_PATH));
const INPUT_PATH: &str = "input/day01";

fn count_increasings(input: &[i32], window_size: usize) -> usize {
    input
        .windows(window_size)
        .zip(input.windows(window_size).skip(1))
        .filter(|(prev, curr)| curr.iter().sum::<i32>() > prev.iter().sum::<i32>())
        .count()
}

pub fn part1() -> usize {
    count_increasings(&*INPUT, 1)
}

pub fn part2() -> usize {
    count_increasings(&*INPUT, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_example() {
        assert_eq!(count_increasings(INPUT, 1), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(count_increasings(INPUT, 3), 5);
    }
}
