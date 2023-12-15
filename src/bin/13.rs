use advent_of_code::*;
use std::cmp::PartialEq;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let score: usize = input
        .split("\n\n")
        .map(|block| solve_either(block, solve_one))
        .sum();

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score: usize = input
        .split("\n\n")
        .map(|block| solve_either(block, solve_two))
        .sum();

    Some(score as u32)
}

fn solve_either<F>(block: &str, solver: F) -> usize
where
    F: Fn(&str) -> usize,
{
    let s = solver(block);
    if s > 0 {
        s * 100
    } else {
        solver(&transpose_text(block))
    }
}

fn solve_one(block: &str) -> usize {
    let lines: Vec<&str> = block.lines().collect();
    (1..lines.len())
        .find(|row| is_mirror(&lines, *row))
        .unwrap_or_default()
}

fn solve_two(block: &str) -> usize {
    let lines: Vec<Vec<char>> = block.lines().map(|line| line.chars().collect()).collect();
    let unfixed_reflection_row: Option<usize> = check(&lines, None);

    let mut flipped_lines = lines.clone();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            flipped_lines[i][j] = flip(*c);
            if let Some(found) = check(&flipped_lines, unfixed_reflection_row) {
                return found;
            }
            flipped_lines[i][j] = *c;
        }
    }
    0
}

fn check(lines: &[Vec<char>], unfixed_reflection_row: Option<usize>) -> Option<usize> {
    let lines_ref: Vec<&[char]> = lines.iter().map(|line| line.as_slice()).collect();
    (1..lines.len()).find(|row| unfixed_reflection_row != Some(*row) && is_mirror(&lines_ref, *row))
}

fn flip(c: char) -> char {
    if c == '.' { '#' } else { '.' }
}

fn is_mirror<T: PartialEq>(lines: &Vec<T>, row: usize) -> bool {
    (0..lines.len() - row)
        .take_while(|&i| i != row)
        .all(|i| lines[row - i - 1] == lines[row + i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
