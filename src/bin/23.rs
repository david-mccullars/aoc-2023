extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(23);

type Pos = (usize, usize);
type Map = Vec<Vec<char>>;

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, end) = parse(input);
    let slope_check = |(r0, c0): &Pos, (r1, c1): &Pos| match map[*r0][*c0] {
        'v' => *r1 == r0 + 1 && c1 == c0,
        '^' => *r1 == r0 - 1 && c1 == c0,
        '>' => r1 == r0 && *c1 == c0 + 1,
        '<' => r1 == r0 && *c1 == c0 - 1,
        _ => true,
    };

    hike_until(&map, &start, slope_check, |pos| *pos == end)
        .into_iter()
        .map(|(_, d)| d as u32)
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start, end) = parse(input);
    Some(max_hike_to(&map, &start, &end) as u32)
}

fn parse(input: &str) -> (Map, Pos, Pos) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = map.len();
    let start = map[0].iter().position(|c| *c == '.').unwrap();
    let end = map[rows - 1].iter().position(|c| *c == '.').unwrap();
    (map, (0, start), (rows - 1, end))
}

fn hike_until<F, G>(map: &Map, start: &Pos, filter: F, stop: G) -> Vec<(Pos, usize)>
where
    F: Fn(&Pos, &Pos) -> bool,
    G: Fn(&Pos) -> bool,
{
    let mut to_visit: VecDeque<(Pos, HashSet<Pos>)> = VecDeque::new();
    to_visit.push_front((*start, HashSet::new()));

    let mut paths: Vec<(Pos, usize)> = vec![];
    while let Some((pos, visited)) = to_visit.pop_back() {
        if pos != *start && stop(&pos) {
            paths.push((pos, visited.len()));
            continue;
        }

        for pos2 in
            neighbors(map, &pos).filter(|pos2| !visited.contains(pos2) && filter(&pos, pos2))
        {
            let mut v = visited.clone();
            v.insert(pos);
            to_visit.push_front((pos2, v));
        }
    }

    paths
}

fn neighbors<'a>(map: &'a Map, pos: &'a Pos) -> impl Iterator<Item = Pos> + 'a {
    vec![
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0, pos.1 + 1),
    ]
    .into_iter()
    .filter(|pos2| pos2.0 < map.len() && pos2.1 < map[0].len() && map[pos2.0][pos2.1] != '#')
}

fn junctions(map: &Map) -> HashSet<Pos> {
    let mut junctions: HashSet<Pos> = HashSet::new();
    for (r, row) in map.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if *val != '#' && neighbors(map, &(r, c)).count() > 2 {
                junctions.insert((r, c));
            }
        }
    }
    junctions
}

fn distances_between_junctions(
    map: &Map,
    junctions: &HashSet<Pos>,
) -> HashMap<Pos, Vec<(Pos, usize)>> {
    let mut distances: HashMap<Pos, Vec<(Pos, usize)>> = HashMap::new();
    for junction in junctions {
        let filter = |_: &Pos, _: &Pos| true;
        let stop = |pos: &Pos| junctions.contains(pos);
        let paths: Vec<(Pos, usize)> = hike_until(map, junction, filter, stop);
        for p in paths {
            distances.entry(*junction).or_default().push(p);
        }
    }
    distances
}

fn max_hike_to(map: &Map, start: &Pos, end: &Pos) -> usize {
    let mut junctions = junctions(map);
    junctions.insert(*start);
    junctions.insert(*end);

    let distances = distances_between_junctions(map, &junctions);
    let mut visited: HashSet<Pos> = HashSet::new();

    _max_hike_to(&distances, &mut visited, start, end).unwrap()
}

fn _max_hike_to(
    distances: &HashMap<Pos, Vec<(Pos, usize)>>,
    seen: &mut HashSet<Pos>,
    pos: &Pos,
    end: &Pos,
) -> Option<usize> {
    if pos == end {
        return Some(0);
    }

    let mut max_dist = None;
    for &(pos2, d) in &distances[pos] {
        if !seen.contains(&pos2) {
            seen.insert(pos2);
            if let Some(d2) = _max_hike_to(distances, seen, &pos2, end) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + d2));
            }
            seen.remove(&pos2);
        }
    }
    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
