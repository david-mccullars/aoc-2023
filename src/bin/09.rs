use advent_of_code::*;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, |n, row| n + row[row.len() - 1]))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, |n, row| row[0] - n))
}

fn solve<F>(input: &str, fold_logic: F) -> u32
where
    F: Fn(i32, Vec<i32>) -> i32,
{
    let predictions: Vec<i32> = input
        .lines()
        .map(|line| {
            let mut rows: Vec<Vec<i32>> = vec![];
            let mut row: Vec<i32> = str_to_vec(line);
            rows.push(row.clone());
            while row.iter().any(|i| *i != 0) {
                row = row.windows(2).map(|a| a[1] - a[0]).collect();
                rows.push(row.clone());
            }
            rows.reverse();

            rows.into_iter().fold(0, &fold_logic)
        })
        .collect();

    predictions.iter().sum::<i32>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
