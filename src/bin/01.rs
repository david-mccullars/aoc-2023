use advent_of_code::*;
use regex::Regex;

advent_of_code::solution!(1);

regex!(RE_FIRST, r"(\d)");
regex!(RE_LAST, r".*(\d)");

regex!(
    RE_FIRST_FULL,
    r"(\d|one|two|three|four|five|six|seven|eight|nine)"
);
regex!(
    RE_LAST_FULL,
    r".*(\d|one|two|three|four|five|six|seven|eight|nine)"
);

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum(input, &RE_FIRST, &RE_LAST))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum(input, &RE_FIRST_FULL, &RE_LAST_FULL))
}

fn sum(input: &str, first: &Regex, last: &Regex) -> u32 {
    input
        .lines()
        .map(|line| {
            let n1 = find_num(line, first);
            let n2 = find_num(line, last);
            n1 * 10 + n2
        })
        .sum()
}

fn find_num(line: &str, re: &Regex) -> u32 {
    let cap = re.captures(line).unwrap();
    let n = cap.get(1).unwrap().as_str();
    name_to_digit(n).unwrap_or_else(|| n.parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
