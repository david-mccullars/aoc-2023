use advent_of_code::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

advent_of_code::solution!(14);

type Grid = Vec<Vec<char>>;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    slide(&mut grid);

    Some(score(&grid) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let max_cycles = 1_000_000_000;

    let mut cache: HashMap<Grid, usize> = HashMap::new();
    let mut i = 0;
    while i < max_cycles {
        slide_cycle(&mut grid);

        let key = grid.clone();
        match cache.entry(key) {
            Occupied(e) => {
                let cycle_start = e.get();
                let cycle_size = i - cycle_start;
                cache.clear();
                i += ((max_cycles - i) / cycle_size) * cycle_size + 1;
            }
            Vacant(e) => {
                e.insert(i);
                i += 1;
            }
        }
    }

    Some(score(&grid) as u32)
}

fn slide(grid: &mut Grid) {
    for col in 0..grid[0].len() {
        let mut fallrow = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                '#' => {
                    fallrow = row + 1;
                }
                '.' => {}
                'O' if fallrow < row => {
                    grid[fallrow][col] = 'O';
                    grid[row][col] = '.';
                    fallrow += 1;
                }
                'O' => {
                    fallrow = row + 1;
                }
                _ => todo!(),
            }
        }
    }
}

fn slide_cycle(grid: &mut Grid) {
    for _ in 0..4 {
        slide(grid);
        rotate_mut(grid);
    }
}

fn score(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .map(|(row, line)| (grid.len() - row) * line.iter().filter(|c| **c == 'O').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
