use pathfinding::directed::dijkstra::dijkstra;

advent_of_code::solution!(17);

type Map = Vec<Vec<usize>>;
type Pos = (i16, i16, i16, i16);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let rows = map.len() as i16;
    let cols = map[0].len() as i16;

    let start: Pos = (0, 0, 0, 0);
    let successors = |pos: &Pos| traverse(&map, pos);
    let success = |pos: &Pos| pos.0 + 1 == rows && pos.1 + 1 == cols;

    let (_, min) = dijkstra(&start, successors, success).expect("Failed to find shortest path");
    Some(min as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let rows = map.len() as i16;
    let cols = map[0].len() as i16;

    let start: Pos = (0, 0, 0, 0);
    let successors = |pos: &Pos| traverse_ultra(&map, pos);
    let success = |pos: &Pos| pos.0 + 1 == rows && pos.1 + 1 == cols && (pos.2 >= 4 || pos.3 >= 4);

    let (_, min) = dijkstra(&start, successors, success).expect("Failed to find shortest path");
    Some(min as u32)
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn traverse(map: &Map, pos: &Pos) -> Vec<(Pos, usize)> {
    let mut possibles: Vec<Pos> = vec![];

    // Up?
    if pos.0 > 0 && -2 <= pos.2 && pos.2 <= 0 {
        possibles.push((pos.0 - 1, pos.1, pos.2 - 1, 0));
    }
    // Down?
    if pos.0 + 1 < map.len() as i16 && 0 <= pos.2 && pos.2 <= 2 {
        possibles.push((pos.0 + 1, pos.1, pos.2 + 1, 0));
    }
    // Left?
    if pos.1 > 0 && -2 <= pos.3 && pos.3 <= 0 {
        possibles.push((pos.0, pos.1 - 1, 0, pos.3 - 1));
    }
    // Right?
    if pos.1 + 1 < map[0].len() as i16 && 0 <= pos.3 && pos.3 <= 2 {
        possibles.push((pos.0, pos.1 + 1, 0, pos.3 + 1));
    }

    possibles
        .into_iter()
        .map(|pos| (pos, map[pos.0 as usize][pos.1 as usize]))
        .collect()
}

fn traverse_ultra(map: &Map, pos: &Pos) -> Vec<(Pos, usize)> {
    let mut possibles: Vec<Pos> = vec![];

    if *pos == (0, 0, 0, 0) {
        possibles.push((1, 0, 1, 0));
        possibles.push((0, 1, 0, 1));
    }

    // Up?
    if pos.0 > 0 && -9 <= pos.2 && pos.2 <= 0 && (pos.2 < 0 || pos.3.abs() >= 4) {
        possibles.push((pos.0 - 1, pos.1, pos.2 - 1, 0));
    }
    // Down?
    if pos.0 + 1 < map.len() as i16 && 0 <= pos.2 && pos.2 <= 9 && (pos.2 > 0 || pos.3.abs() >= 4) {
        possibles.push((pos.0 + 1, pos.1, pos.2 + 1, 0));
    }
    // Left?
    if pos.1 > 0 && -9 <= pos.3 && pos.3 <= 0 && (pos.3 < 0 || pos.2.abs() >= 4) {
        possibles.push((pos.0, pos.1 - 1, 0, pos.3 - 1));
    }
    // Right?
    if pos.1 + 1 < map[0].len() as i16
        && 0 <= pos.3
        && pos.3 <= 9
        && (pos.3 > 0 || pos.2.abs() >= 4)
    {
        possibles.push((pos.0, pos.1 + 1, 0, pos.3 + 1));
    }

    possibles
        .into_iter()
        .map(|pos| (pos, map[pos.0 as usize][pos.1 as usize]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
