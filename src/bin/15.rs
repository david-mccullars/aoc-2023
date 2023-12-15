use advent_of_code::*;

advent_of_code::solution!(15);

regex!(RE, r"^(.*)([=-])(\d*)$");

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(hash).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let boxes = box_those_lenses(input);
    let focusing_power = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_i, bx)| {
            bx.iter()
                .enumerate()
                .map(move |(slot, (_, length))| (box_i + 1) as u32 * (slot + 1) as u32 * *length)
        })
        .sum();

    Some(focusing_power)
}

fn hash(text: &str) -> u32 {
    text.chars().fold(0, |n, c| ((n + c as u32) * 17) % 256)
}

fn box_those_lenses(input: &str) -> Vec<Vec<(&str, u32)>> {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec!(); 256];
    for lens in input.trim().split(',') {
        let (_, [label, op, length]) = RE.captures(lens).unwrap().extract();
        let bx_i = hash(label) as usize;
        let mut bx = boxes[bx_i].clone();
        let length = length.parse::<u32>().unwrap_or_default();
        match (op, bx.iter().position(|(s, _)| *s == label)) {
            ("=", Some(j)) => {
                bx[j] = (label, length);
            }
            ("=", None) => {
                bx.push((label, length));
            }
            ("-", Some(j)) => {
                bx.remove(j);
            }
            ("-", None) => {}
            _ => panic!("Invalid operation"),
        }
        boxes[bx_i] = bx;
    }
    boxes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
