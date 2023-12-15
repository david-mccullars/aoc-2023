use memoize::memoize;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 5))
}

fn solve(input: &str, expansion: usize) -> u64 {
    input
        .lines()
        .map(|line| parse(line, expansion))
        .map(|(springs, counts)| count_all(springs, counts))
        .sum::<u64>()
}

fn parse(line: &str, expansion: usize) -> (String, Vec<usize>) {
    let mut f = line.split(' ');
    let springs = expand(f.next().unwrap_or_default().to_string(), "?", expansion);
    let groups = expand(f.next().unwrap_or_default().to_string(), ",", expansion)
        .split(',')
        .map(|v| v.parse::<usize>().unwrap_or_default())
        .collect::<Vec<_>>();
    (springs, groups)
}

fn expand(s: String, join: &str, expansion: usize) -> String {
    std::iter::repeat(s)
        .take(expansion)
        .collect::<Vec<_>>()
        .join(join)
}

#[memoize]
fn count_all(springs: String, counts: Vec<usize>) -> u64 {
    match springs.chars().next() {
        Some('.') => count_all(springs[1..].to_string(), counts.clone()),
        Some('?') => {
            let maybe_spring: String = springs.replacen('?', "#", 1);
            count_all(maybe_spring, counts.clone())
                + count_all(springs[1..].to_string(), counts.clone())
        }
        Some('#') if counts.is_empty() && springs.contains('#') => 0,
        Some('#') if springs.len() < counts[0] => 0,
        Some('#') if springs[..counts[0]].contains('.') => 0,
        Some('#') if springs.len() == counts[0] => (counts.len() == 1) as u64,
        Some('#') if springs.chars().nth(counts[0]) == Some('#') => 0,
        Some('#') => count_all(springs[counts[0] + 1..].to_string(), counts[1..].to_vec()),
        None => counts.is_empty() as u64,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
