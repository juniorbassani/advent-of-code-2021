use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<usize>> = Lazy::new(|| utils::parse_one_line(INPUT_PATH));
const INPUT_PATH: &str = "input/day07";

fn align_least_fuel(positions: &[usize]) -> usize {
    let mut fuel_cost = vec![0; *positions.iter().max().unwrap() + 1];

    for &pos in positions.iter() {
        for (i, cost) in fuel_cost.iter_mut().enumerate() {
            *cost += (pos as isize - i as isize).abs() as usize;
        }
    }

    fuel_cost.into_iter().min().unwrap()
}

fn align_least_fuel2(positions: &[usize]) -> usize {
    let mut fuel_cost = vec![0; *positions.iter().max().unwrap() + 1];

    for &pos in positions.iter() {
        let mut prev = 0;
        // Right side
        for (i, fuel_cost) in fuel_cost.iter_mut().enumerate().skip(pos + 1) {
            let cost = i - pos + prev;
            *fuel_cost += cost;
            prev = cost;
        }

        prev = 0;
        // Left side
        for i in (0..pos).rev() {
            let cost = pos - i + prev;
            fuel_cost[i] += cost;
            prev = cost;
        }
    }

    fuel_cost.into_iter().min().unwrap()
}

pub fn part1() -> usize {
    align_least_fuel(&*INPUT)
}

pub fn part2() -> usize {
    align_least_fuel2(&*INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[usize] = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn part1_example() {
        assert_eq!(align_least_fuel(INPUT), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(align_least_fuel2(INPUT), 168);
    }
}
