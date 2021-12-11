use std::collections::VecDeque;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
    utils::get_input_as_matrix(INPUT_PATH, |b| {
        std::str::from_utf8(&[b]).unwrap().parse().unwrap()
    })
});
const INPUT_PATH: &str = "input/day09";

#[derive(Clone, Copy)]
enum Status {
    Unvisited,
    Enqueued,
    Visited,
}

fn risk_level(map: &[Vec<u8>]) -> usize {
    let mut risk_level = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_low_point(map, i, j) {
                risk_level += map[i][j] as usize + 1;
            }
        }
    }

    risk_level
}

fn three_largest_basins(map: &[Vec<u8>]) -> usize {
    let mut sizes = vec![];

    for (i, j) in find_all_low_risk_level_points(map) {
        let mut to_visit = VecDeque::new();
        let mut status = vec![vec![Status::Unvisited; map[0].len()]; map.len()];
        to_visit.push_back((i, j));
        status[i][j] = Status::Enqueued;

        let size = basin_size(map, &mut to_visit, &mut status);
        sizes.push(size);
    }

    sizes.sort_unstable();
    sizes.into_iter().rev().take(3).product()
}

fn basin_size(
    map: &[Vec<u8>],
    to_visit: &mut VecDeque<(usize, usize)>,
    status: &mut [Vec<Status>],
) -> usize {
    let mut size = 0;

    // Do a BFS to determine the size of the basin
    while let Some((i, j)) = to_visit.pop_front() {
        let my_level = map[i][j];

        for (their_level, (i, j)) in get_neighbors(map, (i, j)) {
            if let Status::Unvisited = status[i][j] {
                if their_level > my_level && their_level < 9 {
                    to_visit.push_back((i, j));
                    status[i][j] = Status::Enqueued;
                }
            }
        }

        size += 1;
        status[i][j] = Status::Visited;
    }

    size
}

fn find_all_low_risk_level_points(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut low_risk_level = vec![];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_low_point(map, i, j) {
                low_risk_level.push((i, j));
            }
        }
    }

    low_risk_level
}

fn is_low_point(map: &[Vec<u8>], i: usize, j: usize) -> bool {
    get_neighbors(map, (i, j))
        .iter()
        .all(|&(neighbor, ..)| map[i][j] < neighbor)
}

fn get_neighbors(map: &[Vec<u8>], origin: (usize, usize)) -> Vec<(u8, (usize, usize))> {
    let (i, j) = origin;
    let mut neighbors = Vec::with_capacity(4);

    if i > 0 {
        // Up
        neighbors.push((map[i - 1][j], (i - 1, j)));
    }
    if i < map.len() - 1 {
        // Down
        neighbors.push((map[i + 1][j], (i + 1, j)));
    }
    if j > 0 {
        // Left
        neighbors.push((map[i][j - 1], (i, j - 1)));
    }
    if j < map[0].len() - 1 {
        // Right
        neighbors.push((map[i][j + 1], (i, j + 1)));
    }

    neighbors
}

pub fn part1() -> usize {
    risk_level(&*INPUT)
}

pub fn part2() -> usize {
    three_largest_basins(&*INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
    });

    #[test]
    fn part1_example() {
        assert_eq!(risk_level(&*INPUT), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(three_largest_basins(&*INPUT), 1134);
    }
}
