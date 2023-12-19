use advent_of_code::*;

advent_of_code::solution!(18);

type Pos = (i64, i64);

regex!(RE1, r"^([RLUD])\s+(\d+)\s+\(#[0-9a-f]{6}\)$");
regex!(RE2, r"^[RLUD]\s+\d+\s+\(#([0-9a-f]{5})([0-9a-f])\)$");

pub fn part_one(input: &str) -> Option<u32> {
    let area = discrete_polygon_area(vertices(input, parse_one));
    Some(area as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let area = discrete_polygon_area(vertices(input, parse_two));
    Some(area as u64)
}

fn parse_one(line: &str) -> (Pos, i64) {
    let (_, [dir, dist]) = RE1.captures(line).unwrap().extract();
    let dir = match dir {
        "R" => (0, 1),
        "L" => (0, -1),
        "U" => (-1, 0),
        "D" => (1, 0),
        _ => todo!(),
    };
    let dist = dist.parse().unwrap();
    (dir, dist)
}

fn parse_two(line: &str) -> (Pos, i64) {
    let (_, [h1, h2]) = RE2.captures(line).unwrap().extract();
    let dir = match h2 {
        "0" => (0, 1),
        "2" => (0, -1),
        "3" => (-1, 0),
        "1" => (1, 0),
        _ => todo!(),
    };
    let dist = i64::from_str_radix(h1, 16).unwrap();
    (dir, dist)
}

fn vertices<F>(input: &str, parse: F) -> Vec<Pos>
where
    F: Fn(&str) -> (Pos, i64),
{
    let mut pos: Pos = (0, 0);
    let mut all: Vec<Pos> = vec![pos];
    for (dir, dist) in input.lines().map(parse) {
        pos = (pos.0 + (dir.0 * dist), pos.1 + (dir.1 * dist));
        all.push(pos);
    }
    if pos != (0, 0) {
        panic!("We didn't end up back at 0, 0!");
    }
    all
}

fn discrete_polygon_area(vertices: Vec<Pos>) -> i64 {
    let mut inside = 0;
    let mut border = 2;
    for w in vertices.windows(2) {
        let (v1, v2) = (w[0], w[1]);
        border += ((v2.0 - v1.0) + (v2.1 - v1.1)).abs();
        inside += (v1.0 * v2.1) - (v1.1 * v2.0);
    }
    (inside.abs() + border) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
