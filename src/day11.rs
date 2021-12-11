use std::collections::VecDeque;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
    utils::get_input_as_matrix(INPUT_PATH, |b| {
        std::str::from_utf8(&[b]).unwrap().parse().unwrap()
    })
});
const INPUT_PATH: &str = "input/day11";

#[derive(Clone, Copy)]
enum Status {
    Unvisited,
    Enqueued,
}

fn flashes_after_n_steps(mut octopuses: Vec<Vec<u8>>, n: usize, sync: bool) -> (usize, usize) {
    let mut count = 0;

    for curr in 0..n {
        for row in &mut octopuses {
            for octopus in row {
                *octopus += 1;
            }
        }

        for i in 0..octopuses.len() {
            for j in 0..octopuses[0].len() {
                if octopuses[i][j] > 9 {
                    let mut to_visit = VecDeque::new();
                    let mut status =
                        vec![vec![Status::Unvisited; octopuses[0].len()]; octopuses.len()];
                    to_visit.push_back((i, j));
                    status[i][j] = Status::Enqueued;

                    count += flash(&mut octopuses, &mut to_visit, &mut status);

                    if sync && octopuses.iter().flatten().all(|&a| a == 0) {
                        return (count, curr + 1);
                    }
                }
            }
        }
    }

    (count, 0)
}

fn flash(
    octopuses: &mut [Vec<u8>],
    to_visit: &mut VecDeque<(usize, usize)>,
    status: &mut [Vec<Status>],
) -> usize {
    let mut count = 0;

    while let Some((i, j)) = to_visit.pop_front() {
        octopuses[i][j] = 0;
        count += 1;

        for (i, j) in get_neighbors(octopuses, (i, j)) {
            let neighbor = &mut octopuses[i][j];
            // "< 10" comparison is only to prevent overflow
            if *neighbor > 0 && *neighbor < 10 {
                *neighbor += 1;
            }
            if *neighbor > 9 {
                if let Status::Unvisited = status[i][j] {
                    to_visit.push_back((i, j));
                    status[i][j] = Status::Enqueued;
                }
            }
        }
    }

    count
}

fn get_neighbors(octopuses: &[Vec<u8>], origin: (usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = origin;
    let mut neighbors = Vec::with_capacity(8);

    for x in i.checked_sub(1).unwrap_or_default()..=i + 1 {
        for y in j.checked_sub(1).unwrap_or_default()..=j + 1 {
            if (x != i || y != j) && x < octopuses.len() && y < octopuses[0].len() {
                neighbors.push((x, y));
            }
        }
    }

    neighbors
}

pub fn part1() -> usize {
    flashes_after_n_steps(INPUT.clone(), 100, false).0
}

pub fn part2() -> usize {
    flashes_after_n_steps(INPUT.clone(), usize::MAX, true).1
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
        vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
    });

    #[test]
    fn part1_example() {
        assert_eq!(flashes_after_n_steps(INPUT.clone(), 100, false).0, 1656);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            flashes_after_n_steps(INPUT.clone(), usize::MAX, true).1,
            195
        );
    }
}
