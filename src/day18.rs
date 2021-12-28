use std::fmt::{Debug, Display};
use std::ops::Add;
use std::thread::park_timeout;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<List>> = Lazy::new(|| utils::get_input_as_vec_with(INPUT_PATH, List::parse));
const INPUT_PATH: &str = "input/day18";

#[derive(Clone)]
struct List {
    inner: Elem,
}

#[derive(Clone)]
enum Elem {
    Number(usize),
    Pair { lhs: Box<Elem>, rhs: Box<Elem> },
}

impl List {
    fn parse(mut p: &str) -> Self {
        Self {
            inner: Self::elem(&mut p),
        }
    }

    fn elem(e: &mut &str) -> Elem {
        let ch = &e[..1];
        match ch {
            "[" => {
                let (lhs, rhs) = Self::pair(e);
                Elem::Pair {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
            _ => Self::number(e),
        }
    }

    fn pair(p: &mut &str) -> (Elem, Elem) {
        eat(p, "[");
        let lhs = Self::elem(p);
        eat(p, ",");
        let rhs = Self::elem(p);
        eat(p, "]");

        (lhs, rhs)
    }

    fn number(n: &mut &str) -> Elem {
        let num = &n[..1];
        eat(n, num);
        Elem::Number(num.parse().unwrap())
    }

    fn reduce(&mut self) {
        loop {
            if self.inner.more_than_four_nestings(0).is_none() && !self.inner.split() {
                break;
            }
        }
    }

    fn magnitude(&self) -> usize {
        self.inner.magnitude()
    }
}

enum ReduceOp<'p> {
    Pair {
        lhs: &'p mut Box<Elem>,
        rhs: &'p mut Box<Elem>,
    },
    Explode {
        lhs: Option<usize>,
        rhs: Option<usize>,
    },
}

impl Elem {
    fn more_than_four_nestings(&mut self, count: usize) -> Option<ReduceOp> {
        match self {
            Self::Number(_) => None,
            Self::Pair {
                lhs: outer_lhs,
                rhs: outer_rhs,
            } => {
                if count == 4 {
                    Some(ReduceOp::Pair {
                        lhs: outer_lhs,
                        rhs: outer_rhs,
                    })
                } else {
                    match outer_lhs.more_than_four_nestings(count + 1) {
                        Some(ReduceOp::Pair { lhs, rhs }) => {
                            let (lhs, rhs) = (lhs.unwrap_number(), rhs.unwrap_number());
                            *outer_lhs = Box::new(Elem::Number(0));
                            *outer_rhs.leftmost_number() += rhs;

                            return Some(ReduceOp::Explode {
                                lhs: Some(lhs),
                                rhs: None,
                            });
                        }
                        Some(ReduceOp::Explode { lhs, mut rhs }) => {
                            if let Some(rhs) = rhs.take() {
                                *outer_rhs.leftmost_number() += rhs;
                            }
                            return Some(ReduceOp::Explode { lhs, rhs });
                        }
                        None => {}
                    }

                    match outer_rhs.more_than_four_nestings(count + 1) {
                        Some(ReduceOp::Pair { lhs, rhs }) => {
                            let (lhs, rhs) = (lhs.unwrap_number(), rhs.unwrap_number());
                            *outer_rhs = Box::new(Elem::Number(0));
                            *outer_lhs.rightmost_number() += lhs;

                            Some(ReduceOp::Explode {
                                lhs: None,
                                rhs: Some(rhs),
                            })
                        }
                        Some(ReduceOp::Explode { mut lhs, rhs }) => {
                            if let Some(lhs) = lhs.take() {
                                *outer_lhs.rightmost_number() += lhs;
                            }
                            Some(ReduceOp::Explode { lhs, rhs })
                        }
                        None => None,
                    }
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Elem::Number(num) => {
                let num = *num;
                if num >= 10 {
                    *self = Elem::Pair {
                        lhs: Box::new(Elem::Number((num as f32 / 2.0).floor() as usize)),
                        rhs: Box::new(Elem::Number((num as f32 / 2.0).ceil() as usize)),
                    };
                    true
                } else {
                    false
                }
            }
            Elem::Pair { lhs, rhs } => lhs.split() || rhs.split(),
        }
    }

    fn unwrap_number(&self) -> usize {
        match self {
            Elem::Number(num) => *num,
            Elem::Pair { .. } => panic!("Expected a number; found a pair."),
        }
    }

    fn leftmost_number(&mut self) -> &mut usize {
        match self {
            Elem::Number(num) => num,
            Elem::Pair { lhs, rhs } => lhs.leftmost_number(),
        }
    }

    fn rightmost_number(&mut self) -> &mut usize {
        match self {
            Elem::Number(num) => num,
            Elem::Pair { lhs, rhs } => rhs.rightmost_number(),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Elem::Number(num) => *num,
            Elem::Pair { lhs, rhs } => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }
}

impl Add for List {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: Elem::Pair {
                lhs: Box::new(self.inner),
                rhs: Box::new(rhs.inner),
            },
        }
    }
}

fn eat(s: &mut &str, tok: &str) {
    debug_assert_eq!(&s[..tok.len()], tok);
    *s = &s[tok.len()..];
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Debug for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Number(n) => f.write_fmt(format_args!("{}", *n)),
            Elem::Pair { lhs, rhs } => f.write_fmt(format_args!(
                "[{},{}]",
                format_args!("{:?}", &*lhs),
                format_args!("{:?}", &*rhs),
            )),
        }
    }
}

fn add_all(pairs: Vec<List>) -> List {
    pairs
        .into_iter()
        .reduce(|prev, curr| {
            let mut merged = prev + curr;
            merged.reduce();
            merged
        })
        .unwrap()
}

fn largest_magnitude(pairs: Vec<List>) -> usize {
    let mut max = 0;

    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if i == j {
                continue;
            }

            let mut merged = pairs[i].clone() + pairs[j].clone();
            merged.reduce();
            let magnitude = merged.magnitude();
            if magnitude > max {
                max = magnitude;
            }
        }
    }

    max
}

pub fn part1() -> usize {
    let final_list = add_all(INPUT.clone());
    final_list.magnitude()
}

pub fn part2() -> usize {
    largest_magnitude(INPUT.clone())
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::*;

    const INPUT: &[&str] = &[
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ];

    static PAIRS: Lazy<Vec<List>> = Lazy::new(|| INPUT.iter().map(|p| List::parse(p)).collect());

    #[test]
    fn parse_and_add() {
        for (i, pair) in PAIRS.iter().enumerate() {
            assert_eq!(format!("{:?}", pair), String::from(INPUT[i]));
        }
    }

    #[test]
    fn add_pairs() {
        assert_eq!(
            format!("{:?}", List::parse("[1,2]") + List::parse("[[3,4],5]")),
            String::from("[[1,2],[[3,4],5]]")
        );
    }

    #[test]
    fn explosion() {
        let input = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (original, exploded) in input {
            let mut pair = List::parse(original);
            pair.reduce();

            assert_eq!(format!("{:?}", pair), String::from(exploded));
        }
    }

    #[test]
    fn reduce() {
        let mut pair = List::parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + List::parse("[1,1]");
        pair.reduce();
        assert_eq!(
            format!("{:?}", pair),
            String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn part1_example() {
        let final_list = add_all(PAIRS.clone());

        assert_eq!(
            format!("{:?}", final_list),
            String::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
        assert_eq!(final_list.magnitude(), 4140);
    }

    #[test]
    fn part2_example() {
        assert_eq!(largest_magnitude(PAIRS.clone()), 3993);
    }
}
