use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
    utils::get_input_as_matrix(INPUT_PATH, |b| {
        std::str::from_utf8(&[b]).unwrap().parse().unwrap()
    })
});
const INPUT_PATH: &str = "input/day15";

#[derive(Eq)]
struct Node {
    pos: (usize, usize),
    cost: usize,
}

impl Node {
    fn new(pos: (usize, usize), cost: usize) -> Self {
        Self { pos, cost }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn lowest_total_risk(map: &[&[u8]]) -> usize {
    let mut cost_map = vec![vec![usize::MAX; map[0].len()]; map.len()];
    cost_map[0][0] = 0;
    let mut queue = BinaryHeap::with_capacity(map.len() * map[0].len());
    queue.push(Reverse(Node::new((0, 0), cost_map[0][0])));

    while let Some(Reverse(node)) = queue.pop() {
        for (edge_weight, (x, y)) in get_neighbors(map, node.pos) {
            if relax(&mut cost_map, &node, (x, y), edge_weight as usize) {
                queue.push(Reverse(Node::new((x, y), node.cost + edge_weight as usize)));
            }
        }
    }

    *cost_map.last().unwrap().last().unwrap()
}

fn relax(
    cost_map: &mut [Vec<usize>],
    node: &Node,
    (x, y): (usize, usize),
    edge_weight: usize,
) -> bool {
    if cost_map[x][y] > node.cost + edge_weight {
        cost_map[x][y] = node.cost + edge_weight;
        return true;
    }
    false
}

fn get_neighbors(map: &[&[u8]], origin: (usize, usize)) -> Vec<(u8, (usize, usize))> {
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
    let map: Vec<&[u8]> = INPUT.iter().map(|e| e.as_ref()).collect();
    lowest_total_risk(&map)
}

pub fn part2() -> usize {
    let expanded: Vec<&[u8]> = INPUT.iter().map(|e| e.as_ref()).collect();
    let expanded = expand_matrix(&expanded);
    let expanded: Vec<&[u8]> = expanded.iter().map(|e| e.as_ref()).collect();
    lowest_total_risk(&expanded)
}

fn expand_matrix(matrix: &[&[u8]]) -> Vec<Vec<u8>> {
    let original_width = matrix[0].len();
    let original_height = matrix.len();
    let expanded: Vec<&[u8]> = matrix.iter().copied().collect();
    let expanded = expanded.repeat(5);
    let mut expanded: Vec<_> = expanded.iter().map(|&row| row.repeat(5)).collect();

    for (x, row) in expanded.iter_mut().enumerate() {
        for (y, elem) in row.iter_mut().enumerate() {
            let amount = x / original_height + y / original_width;
            let mut res = *elem + amount as u8;
            if res > 9 {
                res -= 9;
            }

            *elem = res;
        }
    }

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&[u8]] = &[
        &[1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
        &[1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
        &[2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
        &[3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
        &[7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
        &[1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
        &[1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
        &[3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
        &[1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
        &[2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
    ];

    #[test]
    fn part1_example() {
        assert_eq!(lowest_total_risk(INPUT), 40);
    }

    #[test]
    fn part2_example() {
        let expanded = expand_matrix(INPUT);
        let expanded: Vec<&[u8]> = expanded.iter().map(|e| e.as_ref()).collect();
        assert_eq!(lowest_total_risk(&expanded), 315);
    }
}
