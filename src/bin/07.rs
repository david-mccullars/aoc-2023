use itertools::Itertools;

advent_of_code::solution!(7);

type Hand = Vec<u16>;

const FIVE_OF_A_KIND: u8 = 7;
const FOUR_OF_A_KIND: u8 = 6;
const FULL_HOUSE: u8 = 5;
const THREE_OF_A_KIND: u8 = 4;
const TWO_PAIR: u8 = 3;
const ONE_PAIR: u8 = 2;
const HIGH_CARD: u8 = 1;

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(&input.replace('J', "*")))
}

fn solve(input: &str) -> u32 {
    let mut hands: Vec<(u8, Hand, u32)> = parse(input)
        .map(|(hand, bid)| (score(&hand), hand, bid))
        .collect();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |n, (pos, (_, _, bid))| n + (pos as u32 + 1) * bid)
}

fn parse(input: &str) -> impl Iterator<Item = (Hand, u32)> + '_ {
    input.lines().map(|line| {
        let line: Vec<_> = line.split_whitespace().collect();
        let hand: Hand = line[0].chars().map(card).collect();
        let bid: u32 = line[1].parse().unwrap();

        (hand, bid)
    })
}

fn card(c: char) -> u16 {
    match c {
        '*' => 1, // Wild!
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c => c.to_digit(10).unwrap() as u16,
    }
}

fn score(cards: &Hand) -> u8 {
    let mut sorted_cards = cards.clone();
    sorted_cards.sort();

    let mut wild: u16 = 0;
    let mut groups: Vec<u16> = vec![];
    for (key, group) in &sorted_cards.into_iter().group_by(|c| *c) {
        let len = group.collect::<Vec<u16>>().len() as u16;
        if key == 1 {
            wild = len;
        } else {
            groups.push(len);
        }
    }
    groups.sort();
    groups.reverse();

    match (
        groups.first().unwrap_or(&0) + wild,
        groups.get(1).unwrap_or(&0),
    ) {
        (5, _) => FIVE_OF_A_KIND,
        (4, _) => FOUR_OF_A_KIND,
        (3, 2) => FULL_HOUSE,
        (3, _) => THREE_OF_A_KIND,
        (2, 2) => TWO_PAIR,
        (2, _) => ONE_PAIR,
        _ => HIGH_CARD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
