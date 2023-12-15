use advent_of_code::*;
use array_tool::vec::Intersect;

advent_of_code::solution!(4);

regex!(RE_LINE, r"Card\s+\d+:\s+(.*?)\s+\|\s+(.*)");

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let caps = RE_LINE.captures(line).unwrap();
        let winning: Vec<u32> = capture_to_vec(&caps, 1);
        let mine: Vec<u32> = capture_to_vec(&caps, 2);

        let overlap = winning.intersect(mine);
        if !overlap.is_empty() {
            let score = 2_u32.pow(overlap.len() as u32 - 1);
            sum += score;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<u32> = vec![1; input.lines().count()];
    for (pos, line) in input.lines().enumerate() {
        let caps = RE_LINE.captures(line).unwrap();
        let winning: Vec<u32> = capture_to_vec(&caps, 1);
        let mine: Vec<u32> = capture_to_vec(&caps, 2);

        let bonus = winning.intersect(mine).len();
        for pos2 in (pos + 1)..(pos + 1 + bonus) {
            cards[pos2] += cards[pos];
        }
    }

    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
