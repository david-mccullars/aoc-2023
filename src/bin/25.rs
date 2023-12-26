extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graphmap::UnGraphMap;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let mut edges = HashSet::<(&str, &str)>::new();
    for line in input.lines() {
        let (n1, s) = line.split_once(':').unwrap();
        for n2 in s.split_whitespace() {
            edges.insert((n1, n2));
        }
    }

    let graph = UnGraphMap::<&str, ()>::from_edges(edges);
    let Ok(Some((_, group))) = stoer_wagner_min_cut(&graph, |_| Ok::<_, ()>(1)) else {
        panic!("Failed to find a min cut")
    };

    Some(group.len() * (graph.node_count() - group.len()))
}

pub fn part_two(_input: &str) -> Option<u32> {
    println!("====== CLAIM THE FINAL GOLD STAR!!! ======");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
