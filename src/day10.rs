use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| utils::get_input_as_vec_with(INPUT_PATH, |line| line.to_string()));
const INPUT_PATH: &str = "input/day10";

fn syntax_error_score(input: &[&str]) -> usize {
    let score_map = HashMap::from([(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]);
    let mut total_score = 0;

    for line in input {
        if let (Some(ch), _) = illegal_character(line) {
            total_score += score_map[&ch];
        }
    }

    total_score
}

fn middle_score_of_incomplete_lines(input: &[&str]) -> usize {
    let score_map = HashMap::from([(b')', 1), (b']', 2), (b'}', 3), (b'>', 4)]);
    let mut scores = vec![];

    input
        .iter()
        .map(|line| illegal_character(line))
        .filter(|(ch, _)| ch.is_none())
        .for_each(|(_, rem_stack)| {
            let score = rem_stack
                .into_iter()
                .rev()
                .map(get_closing_character)
                .fold(0, |score, ch| score * 5 + score_map[&ch]);

            scores.push(score);
        });

    let middle = scores.len() as f32 / 2.0;
    let middle = middle.ceil() as usize - 1;
    scores.sort_unstable();

    scores[middle]
}

fn illegal_character(input: &str) -> (Option<u8>, Vec<u8>) {
    let mut stack = vec![];

    for ch in input.bytes() {
        match ch {
            c @ (b'(' | b'[' | b'{' | b'<') => stack.push(c),
            c @ (b')' | b']' | b'}' | b'>') => {
                let top = stack.pop().unwrap();
                if c != get_closing_character(top) {
                    return (Some(c), stack);
                }
            }
            _ => unreachable!(),
        }
    }

    (None, stack)
}

fn get_closing_character(ch: u8) -> u8 {
    match ch {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => unreachable!(),
    }
}

pub fn part1() -> usize {
    let input: Vec<&str> = INPUT.iter().map(|line| line.as_ref()).collect();
    syntax_error_score(&input)
}

pub fn part2() -> usize {
    let input: Vec<&str> = INPUT.iter().map(|line| line.as_ref()).collect();
    middle_score_of_incomplete_lines(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn part1_example() {
        assert_eq!(syntax_error_score(INPUT), 26397);
    }

    #[test]
    fn part2_example() {
        assert_eq!(middle_score_of_incomplete_lines(INPUT), 288957);
    }
}
