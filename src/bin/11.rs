use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(11);

type Pos = (usize, usize);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 1_000_000))
}

fn solve(input: &str, expansion: usize) -> u64 {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            let row: Vec<bool> = line.chars().map(|c| c == '#').collect();
            row
        })
        .collect();

    let empty_rows: Vec<usize> = map
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.iter().any(|g| *g))
        .map(|(pos, _)| pos)
        .collect();
    let empty_cols: Vec<usize> = transpose(&map)
        .iter()
        .enumerate()
        .filter(|(_, col)| !col.iter().any(|g| *g))
        .map(|(pos, _)| pos)
        .collect();

    let mut galaxies: Vec<Pos> = vec![];
    let mut expanded_row = 0;
    for (row, gs) in map.iter().enumerate() {
        if empty_rows.contains(&row) {
            expanded_row += 1;
        }
        let mut expanded_col = 0;
        for (col, g) in gs.iter().enumerate() {
            if empty_cols.contains(&col) {
                expanded_col += 1;
            }
            if *g {
                galaxies.push((
                    row + (expansion - 1) * expanded_row,
                    col + (expansion - 1) * expanded_col,
                ))
            }
        }
    }

    let mut step = 1;
    galaxies.into_iter().combinations(2).fold(0, |n, pair| {
        let g1 = pair[0];
        let g2 = pair[1];
        step += 1;
        n + manhattan_distance(g1, g2)
    }) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
