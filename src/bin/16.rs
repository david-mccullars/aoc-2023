use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::VecDeque;

advent_of_code::solution!(16);

type Map = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pos {
    row: i8,
    col: i8,
    dr: i8,
    dc: i8,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    Some(energize(&map, Pos::new(0, 0, 0, 1)) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let rows = map.len();
    let cols = map[0].len();

    let mut starts: Vec<Pos> = vec![];
    for row in 0..rows {
        starts.push(Pos::new(row as i8, 0, 0, 1));
        starts.push(Pos::new(row as i8, cols as i8 - 1, 0, -1));
    }
    for col in 0..cols {
        starts.push(Pos::new(0, col as i8, 1, 0));
        starts.push(Pos::new(rows as i8 - 1, col as i8, -1, 0));
    }

    // Perform calculations in parallel!
    let max = starts.par_iter().map(|start| energize(&map, *start)).max();
    Some(max.unwrap() as u32)
}

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn energize(map: &Map, start: Pos) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut to_visit: VecDeque<Pos> = VecDeque::new();
    to_visit.push_front(start);

    while let Some(pos) = to_visit.pop_back() {
        if !pos.is_off_map(map) && !visited.contains(&pos) {
            visited.insert(pos);
            for pos2 in pos.visit(map) {
                to_visit.push_front(pos2);
            }
        }
    }

    visited.iter().unique_by(|pos| (pos.row, pos.col)).count()
}

impl Pos {
    fn new(row: i8, col: i8, dr: i8, dc: i8) -> Pos {
        Pos { row, col, dr, dc }
    }

    fn is_off_map(&self, map: &Map) -> bool {
        self.row < 0
            || self.col < 0
            || self.row >= map.len() as i8
            || self.col >= map[0].len() as i8
    }

	#[rustfmt::skip]
    fn visit(&self, map: &Map) -> Vec<Pos> {
        let op = map[self.row as usize][self.col as usize];
        match op {
            '.'					=> vec![self.forward()],
            '|' if self.dc == 0 => vec![self.forward()],
            '|'					=> vec![self.up(), self.down()],
            '-' if self.dr == 0 => vec![self.forward()],
            '-'					=> vec![self.left(), self.right()],
            '\\'				=> vec![self.bend_positive()],
            '/'					=> vec![self.bend_negative()],
            _					=> todo!(),
        }
    }

    fn forward(&self) -> Pos {
        Pos::new(self.row + self.dr, self.col + self.dc, self.dr, self.dc)
    }

    fn up(&self) -> Pos {
        Pos::new(self.row - 1, self.col, -1, 0)
    }

    fn down(&self) -> Pos {
        Pos::new(self.row + 1, self.col, 1, 0)
    }

    fn left(&self) -> Pos {
        Pos::new(self.row, self.col - 1, 0, -1)
    }

    fn right(&self) -> Pos {
        Pos::new(self.row, self.col + 1, 0, 1)
    }

    fn bend_positive(&self) -> Pos {
        Pos::new(self.row + self.dc, self.col + self.dr, self.dc, self.dr)
    }

    fn bend_negative(&self) -> Pos {
        Pos::new(self.row - self.dc, self.col - self.dr, -self.dc, -self.dr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
