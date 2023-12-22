use advent_of_code::*;
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

advent_of_code::solution!(22);

#[derive(Debug, Clone, Copy)]
struct Brick {
    id: usize,
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
    z0: usize,
    z1: usize,
}

impl Brick {
    fn new(id: usize, b: BrickTuple) -> Self {
        Brick {
            id,
            x0: min(b.0.0, b.1.0),
            x1: max(b.0.0, b.1.0),
            y0: min(b.0.1, b.1.1),
            y1: max(b.0.1, b.1.1),
            z0: min(b.0.2, b.1.2),
            z1: max(b.0.2, b.1.2),
        }
    }

    fn intersect(&self, other: &Brick) -> bool {
        self.id != other.id
            && self.x0 <= other.x1
            && other.x0 <= self.x1
            && self.y0 <= other.y1
            && other.y0 <= self.y1
            && self.z0 <= other.z1
            && other.z0 <= self.z1
    }

    fn intersect_any(&self, others: &Bricks) -> bool {
        others.iter().any(|b| self.intersect(b))
    }

    fn fall_one(&self) -> Option<Brick> {
        if self.z0 == 1 {
            None
        } else {
            let mut brick2 = *self;
            brick2.z0 -= 1;
            brick2.z1 -= 1;
            Some(brick2)
        }
    }
}

type Ids = Vec<usize>;
type BrickInfo = HashMap<usize, Ids>;
type Bricks = Vec<Brick>;

pub fn part_one(input: &str) -> Option<u32> {
    let (bricks, supports, supported_by) = let_the_bricks_fall(input);

    let disintegrateable = bricks
        .iter()
        .filter(|brick| {
            supports[&brick.id]
                .iter()
                .all(|supported_id| supported_by[supported_id].len() > 1)
        })
        .count();

    Some(disintegrateable as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (bricks, supports, supported_by) = let_the_bricks_fall(input);

    let total_fallen: usize = bricks
        .iter()
        .map(|brick| {
            let mut fallen: Ids = vec![];
            collect_fallen_if_removed(brick.id, &supports, &supported_by, &mut fallen);
            fallen.len() - 1
        })
        .sum();

    Some(total_fallen as u32)
}

fn let_the_bricks_fall(input: &str) -> (Bricks, BrickInfo, BrickInfo) {
    let mut bricks = parse(input);
    bricks.sort_by(|a, b| a.z0.cmp(&b.z0));

    for i in 0..bricks.len() {
        loop {
            let brick = bricks[i].fall_one();
            if brick.is_none() || brick.unwrap().intersect_any(&bricks) {
                break;
            }
            bricks[i] = brick.unwrap();
        }
    }

    let mut supports: BrickInfo = HashMap::new();
    let mut supported_by: BrickInfo = HashMap::new();

    for brick in &bricks {
        supports.entry(brick.id).or_default();
        supported_by.entry(brick.id).or_default();
        if let Some(test) = brick.fall_one() {
            for support in bricks.iter().filter(|b| test.intersect(b)) {
                supports.entry(support.id).or_default().push(brick.id);
                supported_by.entry(brick.id).or_default().push(support.id);
            }
        }
    }

    (bricks, supports, supported_by)
}

fn collect_fallen_if_removed(
    brick_id: usize,
    supports: &BrickInfo,
    supported_by: &BrickInfo,
    fallen: &mut Ids,
) {
    fallen.push(brick_id);
    for supported_id in &supports[&brick_id] {
        if supported_by[supported_id]
            .iter()
            .all(|id| fallen.contains(id))
        {
            collect_fallen_if_removed(*supported_id, supports, supported_by, fallen);
        }
    }
}

fn parse(input: &str) -> Bricks {
    input
        .lines()
        .zip(0..)
        .map(|(line, id)| Brick::new(id, parse_BrickTuple(line).unwrap()))
        .collect()
}

type Coord = (usize, usize, usize);
type BrickTuple = (Coord, Coord);

parser!(Coord, r"^(\d+,\d+,\d+)$", |g| g
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect_tuple()
    .unwrap());

parser!(
    BrickTuple,
    r"^([0-9,]+).(.*)$",
    |g| parse_Coord(g).unwrap(),
    |g| parse_Coord(g).unwrap()
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
