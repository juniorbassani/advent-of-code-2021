use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<Line>> = Lazy::new(|| utils::get_input_as_vec_with(INPUT_PATH, parse_line));
const INPUT_PATH: &str = "input/day05";

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Line {
    from: Coordinate,
    to: Coordinate,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Coordinate {
    x: u16,
    y: u16,
}

fn points_least_two_overlap(
    lines: &[Line],
    mut update_map: impl FnMut(&mut Vec<Vec<i32>>, &Line),
    filter: impl Fn(&Line) -> bool,
) -> usize {
    let mut map = vec![vec![0; 1000]; 1000];

    lines
        .into_iter()
        .filter(|line| filter(line))
        .for_each(|line| update_map(&mut map, line));

    map.into_iter()
        .flatten()
        .filter(|&point| point >= 2)
        .count()
}

fn cvt_str_to_line(lines: &[&str]) -> Vec<Line> {
    lines.into_iter().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Line {
    let mut iter = line.split(" -> ");
    let lhs = iter.next().unwrap();
    let rhs = iter.next().unwrap();

    let mut iter = lhs.split(',');
    let x = iter.next().unwrap().parse().unwrap();
    let y = iter.next().unwrap().parse().unwrap();

    let from = Coordinate { x, y };

    let mut iter = rhs.split(',');
    let x = iter.next().unwrap().parse().unwrap();
    let y = iter.next().unwrap().parse().unwrap();

    let to = Coordinate { x, y };

    Line { from, to }
}

fn horizontal_vertical(map: &mut Vec<Vec<i32>>, line: &Line) {
    let Line { from, to } = line;
    let (mut x1, mut y1) = (from.x as usize, from.y as usize);
    let (x2, y2) = (to.x as usize, to.y as usize);

    if x1 != x2 {
        while x1 != x2 {
            map[y1][x1] += 1;
            if x1 < x2 {
                x1 += 1;
            } else {
                x1 -= 1;
            }
        }
    } else {
        while y1 != y2 {
            map[y1][x1] += 1;
            if y1 < y2 {
                y1 += 1;
            } else {
                y1 -= 1;
            }
        }
    }

    // Last point.
    map[y1][x1] += 1;
}

fn horizontal_vertical_diagonal(map: &mut Vec<Vec<i32>>, line: &Line) {
    let Line { from, to } = line;
    let (mut x1, mut y1) = (from.x as usize, from.y as usize);
    let (x2, y2) = (to.x as usize, to.y as usize);

    if (x1 == x2) ^ (y1 == y2) {
        horizontal_vertical(map, line);
        return;
    }

    if x1 == y1 && x2 == y2 {
        while x1 != x2 {
            map[y1][x1] += 1;
            if x1 < x2 {
                x1 += 1;
                y1 += 1;
            } else {
                x1 -= 1;
                y1 -= 1;
            }
        }
    } else if x1 == y2 && y1 == x2 {
        while x1 != x2 {
            map[y1][x1] += 1;
            if x1 < x2 {
                x1 += 1;
                y1 -= 1;
            } else {
                x1 -= 1;
                y1 += 1;
            }
        }
    } else {
        if x1 > x2 && y1 > y2 {
            let down_to = x1 - x2;

            for _ in 0..down_to {
                map[y1][x1] += 1;
                x1 -= 1;
                y1 -= 1;
            }
        } else if x1 < x2 && y1 < y2 {
            let down_to = x2 - x1;
            for _ in 0..down_to {
                map[y1][x1] += 1;
                x1 += 1;
                y1 += 1;
            }
        } else {
            let down_to = if x1 < x2 { x2 - x1 } else { x1 - x2 };
            let xn = if x1 < x2 { 1 } else { -1 };
            let yn = if y1 < y2 { 1 } else { -1 };

            for _ in 0..down_to {
                map[y1][x1] += 1;
                if xn == 1 {
                    x1 += 1;
                } else {
                    x1 -= 1;
                }
                if yn == 1 {
                    y1 += 1;
                } else {
                    y1 -= 1;
                }
            }
        }
    }

    // Last point.
    map[y1][x1] += 1;
}

fn deny_diagonals(line: &Line) -> bool {
    let Line { from, to } = line;
    let (x1, y1) = (from.x, from.y);
    let (x2, y2) = (to.x, to.y);
    // Either vertical or horizontal lines; exclude diagonals.
    (x1 == x2) ^ (y1 == y2)
}

pub fn part1() -> usize {
    points_least_two_overlap(&*INPUT, horizontal_vertical, deny_diagonals)
}

pub fn part2() -> usize {
    points_least_two_overlap(&*INPUT, horizontal_vertical_diagonal, |_| true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn parse_str_as_line() {
        assert_eq!(
            cvt_str_to_line(INPUT),
            vec![
                Line {
                    from: Coordinate { x: 0, y: 9 },
                    to: Coordinate { x: 5, y: 9 }
                },
                Line {
                    from: Coordinate { x: 8, y: 0 },
                    to: Coordinate { x: 0, y: 8 }
                },
                Line {
                    from: Coordinate { x: 9, y: 4 },
                    to: Coordinate { x: 3, y: 4 }
                },
                Line {
                    from: Coordinate { x: 2, y: 2 },
                    to: Coordinate { x: 2, y: 1 }
                },
                Line {
                    from: Coordinate { x: 7, y: 0 },
                    to: Coordinate { x: 7, y: 4 }
                },
                Line {
                    from: Coordinate { x: 6, y: 4 },
                    to: Coordinate { x: 2, y: 0 }
                },
                Line {
                    from: Coordinate { x: 0, y: 9 },
                    to: Coordinate { x: 2, y: 9 }
                },
                Line {
                    from: Coordinate { x: 3, y: 4 },
                    to: Coordinate { x: 1, y: 4 }
                },
                Line {
                    from: Coordinate { x: 0, y: 0 },
                    to: Coordinate { x: 8, y: 8 }
                },
                Line {
                    from: Coordinate { x: 5, y: 5 },
                    to: Coordinate { x: 8, y: 2 }
                },
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            points_least_two_overlap(&cvt_str_to_line(INPUT), horizontal_vertical, deny_diagonals),
            5
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            points_least_two_overlap(
                &cvt_str_to_line(INPUT),
                horizontal_vertical_diagonal,
                |_| true
            ),
            12
        );
    }
}
