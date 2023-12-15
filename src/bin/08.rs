use advent_of_code::*;
use num::integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

regex!(RE, r"(...) = \((...), (...)\)");

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, instructions) = parse(input);

    let start = to_id("AAA");
    let finish = to_id("ZZZ");

    let mut pos = start;
    let mut step = 0;
    while pos != finish {
        let dir = directions[step % directions.len()];
        pos = if dir == 'L' {
            instructions[&pos].0
        } else {
            instructions[&pos].1
        };
        step += 1;
    }

    Some(step.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, instructions) = parse(input);

    let starts: Vec<&u16> = instructions.keys().filter(|id| *id % 26 == 0).collect();
    let cycles: Vec<usize> = starts
        .into_iter()
        .map(|start| {
            let mut pos = *start;
            let mut step = 0;
            while pos % 26 < 25 {
                let dir = directions[step % directions.len()];
                pos = if dir == 'L' {
                    instructions[&pos].0
                } else {
                    instructions[&pos].1
                };
                step += 1;
            }
            step
        })
        .collect();

    Some(cycles.into_iter().fold(1, lcm).try_into().unwrap())
}

fn parse(input: &str) -> (Vec<char>, HashMap<u16, (u16, u16)>) {
    let directions: Vec<char> = input.lines().next().unwrap_or_default().chars().collect();

    let mut instructions: HashMap<u16, (u16, u16)> = HashMap::new();
    for line in input.lines().skip(2) {
        let (_, [id, left, right]) = RE.captures(line).unwrap().extract();
        instructions.insert(to_id(id), (to_id(left), to_id(right)));
    }

    (directions, instructions)
}

fn to_id(s: &str) -> u16 {
    s.chars()
        .fold(0, |a, c| 26 * a + (c.to_ascii_lowercase() as u16) - 97)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
