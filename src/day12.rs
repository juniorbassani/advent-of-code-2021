use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<String>> = Lazy::new(|| utils::get_input_as_vec(INPUT_PATH));
const INPUT_PATH: &str = "input/day12";

#[derive(Debug)]
struct Graph<'a> {
    nodes: HashMap<&'a str, usize>,
    adjacency_matrix: Vec<Vec<usize>>,
}

impl<'a> Graph<'a> {
    fn build(paths: &[&'a str]) -> Self {
        let mut nodes = HashMap::new();
        let mut id = 0;
        let mut record = |node| {
            if node != "start" && node != "end" {
                nodes.entry(node).or_insert_with(|| {
                    id += 1;
                    id
                });
            }
        };

        for path in paths {
            let (lhs, rhs) = path.split_once('-').unwrap();
            record(lhs);
            record(rhs);
        }

        nodes.insert("start", 0);
        id += 1;
        nodes.insert("end", id);
        id += 1;

        let mut adjacency_matrix = vec![vec![0; id]; id];

        // Set the MSB to create an edge connecting the `lhs` and `rhs`.
        for path in paths {
            let (lhs, rhs) = path.split_once('-').unwrap();
            let (i, j) = (nodes[lhs], nodes[rhs]);
            adjacency_matrix[i][j] = 0x01 << (usize::BITS - 1);
            adjacency_matrix[j][i] = 0x01 << (usize::BITS - 1);
        }

        // Set bit 1 on all rows of nodes that represent big caves.
        for (cave, &node_id) in &nodes {
            if cave.chars().next().unwrap().is_ascii_uppercase() {
                for j in 0..adjacency_matrix.len() {
                    adjacency_matrix[node_id][j] |= 0x02;
                }
            }
        }

        Self {
            nodes,
            adjacency_matrix,
        }
    }

    fn all_paths(&mut self) -> usize {
        let mut count = 0;
        self.visit(self.nodes["start"], &mut count);
        count
    }

    fn visit(&mut self, node_id: usize, count: &mut usize) {
        if node_id == self.nodes["end"] {
            *count += 1;
            return;
        }

        let nodes = &self.adjacency_matrix[node_id];
        // Iteration starts at 1 so we don't visit the "start" node.
        for v in 1..nodes.len() {
            if self.has_edge(node_id, v) && (self.is_big(v) || !self.visited(v)) {
                self.mark_visited(v);
                self.visit(v, count);
                self.mark_unvisited(v);
            }
        }
    }

    fn all_paths2(&mut self) -> usize {
        let mut count = 0;
        self.visit2(self.nodes["start"], &mut count, &mut None);
        count
    }

    fn visit2(&mut self, node_id: usize, count: &mut usize, small_cave_twice: &mut Option<usize>) {
        if node_id == self.nodes["end"] {
            *count += 1;
            return;
        }

        let nodes = &self.adjacency_matrix[node_id];
        // Iteration starts at 1 so we don't visit the "start" node.
        for v in 1..nodes.len() {
            if self.has_edge(node_id, v) {
                if small_cave_twice.is_none() && self.is_small(v) && self.visited(v) {
                    *small_cave_twice = Some(v);
                } else if self.is_small(v) && self.visited(v) {
                    continue;
                }
                self.mark_visited(v);
                self.visit2(v, count, small_cave_twice);
                if let Some(visited) = small_cave_twice {
                    if *visited == v {
                        *small_cave_twice = None;
                        continue;
                    }
                }
                self.mark_unvisited(v);
            }
        }
    }

    // If the MSB is set, then there is an edge connecting the nodes.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.adjacency_matrix[u][v] & (0x01 << (usize::BITS - 1)) != 0
    }

    // If the LSB is set, then this node has already been visited.
    fn visited(&self, node_id: usize) -> bool {
        self.adjacency_matrix[0][node_id] & 0x01 != 0
    }

    // Set the LSB of all rows in this node's column to mark it as visited.
    fn mark_visited(&mut self, node_id: usize) {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i][node_id] |= 0x01;
        }
    }

    // Unset the LSB of all rows in this node's column to mark it as unvisited.
    fn mark_unvisited(&mut self, node_id: usize) {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i][node_id] &= !0x01;
        }
    }

    // If bit 1 is set, then this node represents a big cave.
    fn is_big(&self, node_id: usize) -> bool {
        self.adjacency_matrix[node_id][0] & 0x02 != 0
    }

    fn is_small(&self, node_id: usize) -> bool {
        !self.is_big(node_id)
    }
}

pub fn part1() -> usize {
    let paths: Vec<&str> = INPUT.iter().map(|path| path.as_ref()).collect();
    Graph::build(&paths).all_paths()
}

pub fn part2() -> usize {
    let paths: Vec<&str> = INPUT.iter().map(|path| path.as_ref()).collect();
    Graph::build(&paths).all_paths2()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &[&str] = &["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
    const INPUT2: &[&str] = &[
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];
    const INPUT3: &[&str] = &[
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn part1_example() {
        assert_eq!(Graph::build(INPUT1).all_paths(), 10);
        assert_eq!(Graph::build(INPUT2).all_paths(), 19);
        assert_eq!(Graph::build(INPUT3).all_paths(), 226);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Graph::build(INPUT1).all_paths2(), 36);
        assert_eq!(Graph::build(INPUT2).all_paths2(), 103);
        assert_eq!(Graph::build(INPUT3).all_paths2(), 3509);
    }
}
