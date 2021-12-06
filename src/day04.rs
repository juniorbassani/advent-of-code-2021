use once_cell::sync::Lazy;

use crate::utils;

static CHOSEN_NUMBERS: Lazy<Vec<i32>> = Lazy::new(|| utils::parse_one_line(INPUT_PATH));
static BOARDS: Lazy<Vec<Board>> = Lazy::new(|| {
    let contents = std::fs::read_to_string(INPUT_PATH).unwrap();
    let pos = contents.find(|ch: char| ch.is_ascii_whitespace()).unwrap();
    let contents = contents[pos..].trim_start();
    let mut board = vec![];

    for b in contents.split("\n\n") {
        let mut raw: Board = Default::default();
        let b = b.trim_end();
        for (i, row) in b.split('\n').enumerate() {
            for (j, elem) in row.split_ascii_whitespace().enumerate() {
                raw[i][j].0 = elem.parse().unwrap();
            }
        }
        board.push(raw);
    }

    board
});
const INPUT_PATH: &str = "input/day04";

type Board = [[(i32, bool); 5]; 5];

fn find_first_winning_board(boards: &mut [Board], numbers: &[i32]) -> usize {
    for &number in numbers {
        mark(boards, number);

        if let Some((_, board)) = winner(boards) {
            let sum_unmarked = sum_unmarked_numbers(board);
            return sum_unmarked * number as usize;
        }
    }

    panic!("No winner");
}

fn find_last_winning_board(boards: &[Board], numbers: &[i32]) -> usize {
    let mut boards = boards.to_vec();
    let next_idx;

    'outer: loop {
        for (idx, &number) in numbers.iter().enumerate() {
            mark(&mut boards, number);

            while let Some((i, _)) = winner(&boards) {
                boards.remove(i);
                if boards.len() == 1 {
                    next_idx = idx + 1;
                    break 'outer;
                }
            }
        }
    }

    for &number in &numbers[next_idx..] {
        mark(&mut boards, number);

        if let Some((_, board)) = winner(&boards) {
            let sum_unmarked = sum_unmarked_numbers(board);
            return sum_unmarked * number as usize;
        }
    }

    panic!("No last winner");
}

fn mark(boards: &mut [Board], drawn_number: i32) {
    for board in boards {
        for row in board {
            for (num, mark) in row {
                if *num == drawn_number {
                    *mark = true;
                }
            }
        }
    }
}

// Returns index + &Board of the winning board, if any.
fn winner(boards: &[Board]) -> Option<(usize, &Board)> {
    // Any winning row?
    for (i, board) in boards.iter().enumerate() {
        let row_match = board.iter().any(|row| row.iter().all(|(_, mark)| *mark));

        if row_match {
            return Some((i, board));
        }
    }

    // Any winning column?
    for (i, board) in boards.iter().enumerate() {
        'row: for row in 0..5 {
            let mut count = 0;

            #[allow(clippy::needless_range_loop)]
            for col in 0..5 {
                let (_, mark) = board[col][row];
                if !mark {
                    continue 'row;
                }
                count += 1;
            }

            if count == 5 {
                return Some((i, board));
            }
        }
    }

    None
}

fn sum_unmarked_numbers(board: &Board) -> usize {
    let mut count = 0;

    for row in board {
        for &(num, mark) in row {
            if !mark {
                count += num;
            }
        }
    }

    count as usize
}

pub fn part1() -> usize {
    find_first_winning_board(&mut BOARDS.to_vec(), &*CHOSEN_NUMBERS)
}

pub fn part2() -> usize {
    find_last_winning_board(&BOARDS.to_vec(), &*CHOSEN_NUMBERS)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHOSEN_NUMBERS: &[i32] = &[
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    static BOARDS: &[Board] = &[
        [
            [
                (22, false),
                (13, false),
                (17, false),
                (11, false),
                (0, false),
            ],
            [(8, false), (2, false), (23, false), (4, false), (24, false)],
            [
                (21, false),
                (9, false),
                (14, false),
                (16, false),
                (7, false),
            ],
            [(6, false), (10, false), (3, false), (18, false), (5, false)],
            [
                (1, false),
                (12, false),
                (20, false),
                (15, false),
                (19, false),
            ],
        ],
        [
            [(3, false), (15, false), (0, false), (2, false), (22, false)],
            [
                (9, false),
                (18, false),
                (13, false),
                (17, false),
                (5, false),
            ],
            [
                (19, false),
                (8, false),
                (7, false),
                (25, false),
                (23, false),
            ],
            [
                (20, false),
                (11, false),
                (10, false),
                (24, false),
                (4, false),
            ],
            [
                (14, false),
                (21, false),
                (16, false),
                (12, false),
                (6, false),
            ],
        ],
        [
            [
                (14, false),
                (21, false),
                (17, false),
                (24, false),
                (4, false),
            ],
            [
                (10, false),
                (16, false),
                (15, false),
                (9, false),
                (19, false),
            ],
            [
                (18, false),
                (8, false),
                (23, false),
                (26, false),
                (20, false),
            ],
            [
                (22, false),
                (11, false),
                (13, false),
                (6, false),
                (5, false),
            ],
            [(2, false), (0, false), (12, false), (3, false), (7, false)],
        ],
    ];

    #[test]
    fn part1_example() {
        assert_eq!(
            find_first_winning_board(&mut BOARDS.to_vec(), CHOSEN_NUMBERS),
            4512
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            find_last_winning_board(&BOARDS.to_vec(), CHOSEN_NUMBERS),
            1924
        );
    }
}
