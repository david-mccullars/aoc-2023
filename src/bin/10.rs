advent_of_code::solution!(10);

type Pos = (i32, i32);
type Pipe = (Pos, Pos);

const NORTH: Pos = (-1, 0);
const SOUTH: Pos = (1, 0);
const EAST: Pos = (0, 1);
const WEST: Pos = (0, -1);

pub fn part_one(input: &str) -> Option<u32> {
    let (start, start_dirs, map) = parse(input);

    let mut prev_pos: Pos = start;
    let mut pos: Pos = start_dirs[0];
    let mut step = 1;
    while pos != start_dirs[1] {
        let (d1, d2) = map[pos.0 as usize][pos.1 as usize].unwrap();
        (prev_pos, pos) = if d1 == prev_pos { (pos, d2) } else { (pos, d1) };
        step += 1;
    }
    step += 1;

    Some(step / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start, start_dirs, map) = parse(input);

    let mut prev_pos: Pos = start;
    let mut pos: Pos = start_dirs[0];

    let start_rhd = right_hand_dir((pos.0 - prev_pos.0, pos.1 - prev_pos.1));
    let mut trail: Vec<(Pos, Pos)> = vec![(start, start_rhd)];

    while pos != start_dirs[1] {
        let dir = (pos.0 - prev_pos.0, pos.1 - prev_pos.1);
        let rhd = right_hand_dir(dir);
        trail.push((pos, rhd));

        let (d1, d2) = map[pos.0 as usize][pos.1 as usize].unwrap();
        let new_pos = if d1 == prev_pos { d2 } else { d1 };

        // If we changed directions, add a second trail entry
        if (pos.0 + dir.0, pos.1 + dir.1) != new_pos {
            let dir2 = (new_pos.0 - pos.0, new_pos.1 - pos.1);
            let rhd2 = right_hand_dir(dir2);
            trail.push((pos, rhd2));
        }

        (prev_pos, pos) = (pos, new_pos);
    }
    let rhd = right_hand_dir((pos.0 - prev_pos.0, pos.1 - prev_pos.1));
    trail.push((pos, rhd));

    let mut map: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    for (pos, _) in &trail {
        map[pos.0 as usize][pos.1 as usize] = true;
    }

    let mut inside: Vec<Pos> = vec![];
    for (pos, rhd) in &trail {
        let mut pos2 = (pos.0 + rhd.0, pos.1 + rhd.1);
        let mut ss = 0;
        while !map[pos2.0 as usize][pos2.1 as usize] {
            inside.push(pos2);
            pos2 = (pos2.0 + rhd.0, pos2.1 + rhd.1);
            ss += 1;
            if ss > 100 {
                panic!();
            }
        }
    }
    inside.sort();
    inside.dedup();

    Some(inside.len() as u32)
}

fn parse(input: &str) -> (Pos, Vec<Pos>, Vec<Vec<Option<Pipe>>>) {
    let mut start: Pos = (-1, -1);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (row as i32, col as i32);
            }
        }
    }

    let mut start_dirs: Vec<Pos> = vec![];
    let map: Vec<Vec<Option<Pipe>>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let row_map: Vec<Option<Pipe>> = line
                .chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '.' || c == 'S' {
                        None
                    } else {
                        let (d1, d2) = dirs(c);
                        let d1 = (row as i32 + d1.0, col as i32 + d1.1);
                        let d2 = (row as i32 + d2.0, col as i32 + d2.1);
                        if d1 == start || d2 == start {
                            start_dirs.push((row as i32, col as i32));
                        }
                        Some((d1, d2))
                    }
                })
                .collect();
            row_map
        })
        .collect();

    (start, start_dirs, map)
}

fn dirs(c: char) -> Pipe {
    match c {
        '|' => (NORTH, SOUTH), // is a vertical pipe connecting north and south.
        '-' => (EAST, WEST),   // is a horizontal pipe connecting east and west.
        'L' => (NORTH, EAST),  // is a 90-degree bend connecting north and east.
        'J' => (NORTH, WEST),  // is a 90-degree bend connecting north and west.
        '7' => (SOUTH, WEST),  // is a 90-degree bend connecting south and west.
        'F' => (SOUTH, EAST),  // is a 90-degree bend connecting south and east.
        _ => todo!(),
    }
}

fn right_hand_dir(dir: Pos) -> Pos {
    match dir {
        NORTH => EAST,
        EAST => SOUTH,
        SOUTH => WEST,
        WEST => NORTH,
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
