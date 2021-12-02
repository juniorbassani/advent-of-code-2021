use std::str::FromStr;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<Command>> = Lazy::new(|| utils::get_input_as_vec(INPUT_PATH));
const INPUT_PATH: &str = "input/day02";

#[derive(Debug, PartialEq, Eq)]
struct Command {
    direction: Direction,
    units: usize,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        let (direction, units) = (
            iter.next().ok_or("Missing direction")?.parse()?,
            iter.next()
                .ok_or("Missing units")?
                .parse()
                .map_err(|_| "Not a usize")?,
        );

        Ok(Command { direction, units })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => return Err("Not a direction"),
        };

        Ok(direction)
    }
}

fn compute_position(commands: &[Command]) -> (usize, usize) {
    commands
        .iter()
        .fold((0, 0), |(x, y), cmd| match cmd.direction {
            Direction::Forward => (x + cmd.units, y),
            Direction::Down => (x, y + cmd.units),
            Direction::Up => (x, y - cmd.units),
        })
}

fn compute_position_with_aim(commands: &[Command]) -> (usize, usize) {
    let (x, y, _) = commands
        .iter()
        .fold((0, 0, 0), |(x, y, a), cmd| match cmd.direction {
            Direction::Forward => (x + cmd.units, y + a * cmd.units, a),
            Direction::Down => (x, y, a + cmd.units),
            Direction::Up => (x, y, a - cmd.units),
        });

    (x, y)
}

pub fn part1() -> usize {
    let (x, y) = compute_position(&*INPUT);
    x * y
}

pub fn part2() -> usize {
    let (x, y) = compute_position_with_aim(&*INPUT);
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn cvt_str_to_command() {
        let actual = INPUT
            .lines()
            .map(|command| command.parse::<Command>().unwrap())
            .collect::<Vec<Command>>();

        let expected = [
            Command {
                direction: Direction::Forward,
                units: 5,
            },
            Command {
                direction: Direction::Down,
                units: 5,
            },
            Command {
                direction: Direction::Forward,
                units: 8,
            },
            Command {
                direction: Direction::Up,
                units: 3,
            },
            Command {
                direction: Direction::Down,
                units: 8,
            },
            Command {
                direction: Direction::Forward,
                units: 2,
            },
        ];

        assert_eq!(&actual, &expected);
    }

    #[test]
    fn part1_example() {
        let commands = INPUT
            .lines()
            .map(|command| command.parse::<Command>().unwrap())
            .collect::<Vec<Command>>();

        assert_eq!(compute_position(&commands), (15, 10));
    }

    #[test]
    fn part2_example() {
        let commands = INPUT
            .lines()
            .map(|command| command.parse::<Command>().unwrap())
            .collect::<Vec<Command>>();

        assert_eq!(compute_position_with_aim(&commands), (15, 60));
    }
}
