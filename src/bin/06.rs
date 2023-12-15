use advent_of_code::*;

advent_of_code::solution!(6);

regex!(RE_TIME, r"Time:\s+(.*)");
regex!(RE_DIST, r"Distance:\s+(.*)");
regex!(RE, r"\d+");

pub fn part_one(input: &str) -> Option<u32> {
    let times: Vec<u32> = capture_to_vec(&RE_TIME.captures(input).unwrap(), 1);
    let dist: Vec<u32> = capture_to_vec(&RE_DIST.captures(input).unwrap(), 1);

    let races: Vec<(&u32, &u32)> = times.iter().zip(dist.iter()).collect();

    let wiggle: Vec<u32> = races
        .iter()
        .map(|&(&t, &d)| {
            let i = ((t * t) as f32 - (4 * d) as f32).sqrt();
            let mut min: u32 = (((t as f32) - i) / 2.0).ceil() as u32;
            let mut max: u32 = (((t as f32) + i) / 2.0).floor() as u32;
            if min * t - min * min == d {
                min += 1
            }
            if max * t - max * max == d {
                max -= 1
            }
            max - min + 1
        })
        .collect();

    let result: u32 = wiggle.iter().product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let td: Vec<u64> = RE
        .captures_iter(input.replace(' ', "").as_str())
        .map(|s| s.get(0).unwrap().as_str().parse::<u64>().unwrap())
        .collect();
    let [t, d]: [u64; 2] = td.try_into().unwrap();

    let i = ((t * t) as f64 - (4 * d) as f64).sqrt();
    let mut min: u64 = (((t as f64) - i) / 2.0).ceil() as u64;
    let mut max: u64 = (((t as f64) + i) / 2.0).floor() as u64;
    if min * t - min * min == d {
        min += 1
    }
    if max * t - max * max == d {
        max -= 1
    }

    Some((max - min + 1).try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
