use std::collections::HashMap;
use std::fs;

use once_cell::sync::Lazy;

use crate::utils;

static FILE: Lazy<String> = Lazy::new(|| fs::read_to_string(INPUT_PATH).unwrap());
static INPUT: Lazy<Digits<'_>> = Lazy::new(|| {
    let contents = FILE.lines().collect::<Vec<&str>>();
    parse_input(&contents)
});
const INPUT_PATH: &str = "input/day08";

type Digits<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

fn unscramble_numbers(mut digits: Digits<'_>) -> usize {
    let mut sum = 0;
    let mut map = HashMap::with_capacity(7);

    for (observed, output) in &mut digits {
        observed.sort_unstable_by_key(|o| o.len());

        let one = observed[0];
        let four = observed[2];
        let seven = observed[1];
        let eight = observed[9];

        let to_a = diff(seven, one).unwrap();
        map.insert(to_a, 'a');

        let four_and_seven = merge(four, seven);
        let mut zero_six_nine = Vec::from(&observed[6..9]);
        let mut to_g;
        let mut i = 0;

        // Pick a 6-wire entry; it will be either 0, 6 or 9. Only 9 will have exactly one wire
        // difference when intersected with 4 merged with 7. The intersection will map to g.
        loop {
            to_g = diff(zero_six_nine[i], &four_and_seven);
            if to_g.is_some() {
                break;
            }
            i += 1;
        }

        map.insert(to_g.unwrap(), 'g');

        let nine = zero_six_nine.remove(i);
        let zero_six = zero_six_nine;

        map.insert(diff(eight, nine).unwrap(), 'e');

        let c_or_d = diff(zero_six[0], zero_six[1]).unwrap();
        let c_or_d2 = diff(zero_six[1], zero_six[0]).unwrap();

        if one.contains(c_or_d) {
            map.insert(c_or_d, 'c');
            map.insert(c_or_d2, 'd');
        } else {
            map.insert(c_or_d, 'd');
            map.insert(c_or_d2, 'c');
        };

        let to_f = seven.chars().find(|&ch| !map.contains_key(&ch)).unwrap();
        map.insert(to_f, 'f');
        let to_b = four.chars().find(|&ch| !map.contains_key(&ch)).unwrap();
        map.insert(to_b, 'b');

        sum += decode(&map, output);
        map.clear();
    }

    sum
}

fn decode(map: &HashMap<char, char>, output: &[&str]) -> usize {
    let mut num = 0;

    for (i, &output) in output.iter().rev().enumerate() {
        let mut pattern = 0;
        output
            .chars()
            .map(|ch| map[&ch] as u8 - b'a')
            .for_each(|bit| pattern |= 1 << bit);

        let n = match pattern {
            0b1110111 => 0,
            0b0100100 => 1,
            0b1011101 => 2,
            0b1101101 => 3,
            0b0101110 => 4,
            0b1101011 => 5,
            0b1111011 => 6,
            0b0100101 => 7,
            0b1111111 => 8,
            0b1101111 => 9,
            _ => unreachable!(),
        };

        num += n * 10usize.pow(i as u32);
    }

    num
}

// Returns character difference if `a` is exactly one char longer than `b`.
fn diff(a: &str, b: &str) -> Option<char> {
    let diff: Vec<char> = a.chars().filter(|&ch| !b.contains(ch)).collect();

    match *diff {
        [d] => Some(d),
        _ => None,
    }
}

fn merge(a: &str, b: &str) -> String {
    let a = a.to_string();
    let mut b = b.to_string();
    b.retain(|ch| !a.contains(ch));
    a + &b
}

fn unique_num_of_segments(digits: Digits<'_>) -> usize {
    let unique_nums = [2, 3, 4, 7];

    digits
        .iter()
        .flat_map(|(_, output)| output)
        .filter(|output| unique_nums.contains(&output.len()))
        .count()
}

fn parse_input<'a>(input: &[&'a str]) -> Digits<'a> {
    input
        .iter()
        .map(|segment| {
            let (observed, output) = segment.split_once(" | ").unwrap();
            let observed = observed.split_ascii_whitespace().collect();
            let output = output.split_ascii_whitespace().collect();
            (observed, output)
        })
        .collect()
}

pub fn part1() -> usize {
    unique_num_of_segments(INPUT.clone())
}

pub fn part2() -> usize {
    unscramble_numbers(INPUT.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_input(&[INPUT[0]]),
            vec![(
                vec![
                    "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                    "fabcd", "edb"
                ],
                vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]
            )]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(unique_num_of_segments(parse_input(INPUT)), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(unscramble_numbers(parse_input(INPUT)), 61229);
    }
}
