use advent_of_code::*;
use pathfinding::directed::dijkstra::*;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(input, 64)
}

fn _part_one(input: &str, steps: usize) -> Option<u32> {
    let (start, map) = parse(input);

    let reached = visit_all(&map, &start, steps);
    Some(reached as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    _part_two(input, 26501365)
}

fn _part_two(input: &str, steps: usize) -> Option<u64> {
    let n = input.lines().count();
    let half_n = n / 2; // Round down

    let (start, map) = expand(input, 5);
    assert_eq!(map.len(), map[0].len()); // Map is square

    let poly = polynomial_interpolate(vec![0.0, 1.0, 2.0], |x| {
        visit_all(&map, &start, half_n + (x as usize) * n) as f64
    });

    Some(poly((steps - half_n) as f64 / (n as f64)) as u64)
}

fn parse(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut start: (usize, usize) = (0, 0);
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row, col);
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();
    (start, map)
}

fn expand(input: &str, amount: usize) -> ((usize, usize), Vec<Vec<char>>) {
    assert_eq!(amount % 2, 1); // Amount must be odd

    let (start, map) = parse(input);
    let rows = map.len();
    let cols = map[0].len();

    let map2 = map
        .iter()
        .map(|row| {
            row.iter()
                .copied()
                .cycle()
                .take(cols * amount)
                .collect::<Vec<char>>()
        })
        .cycle()
        .take(rows * amount)
        .collect();

    let start2 = (rows * (amount / 2) + start.0, cols * (amount / 2) + start.1);

    (start2, map2)
}

fn visit_all(map: &Vec<Vec<char>>, start: &(usize, usize), steps: usize) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    dijkstra_reach(start, |&node, cost| {
        [-1, 1_isize]
            .into_iter()
            .flat_map(move |delta| {
                [
                    (node.0.wrapping_add_signed(delta), node.1),
                    (node.0, node.1.wrapping_add_signed(delta)),
                ]
            })
            .filter(move |(r, c)| cost < steps && *r < rows && *c < cols && map[*r][*c] != '#')
            .map(|n2| (n2, 1))
    })
    .filter(|reached| reached.total_cost % 2 == steps % 2)
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }
}
