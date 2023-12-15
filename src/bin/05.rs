use advent_of_code::*;

advent_of_code::solution!(5);

regex!(RE_SEEDS, r"seeds: ([0-9 ]+)");
regex!(RE_MAPS, r"(\S+)-to-(\S+) map:\s+([0-9\s]+)");

pub fn part_one(input: &str) -> Option<u32> {
    let mut ids: Vec<u64> = capture_to_vec(&RE_SEEDS.captures(input).unwrap(), 1);
    for cap in RE_MAPS.captures_iter(input) {
        let (_, [_, _, data]) = cap.extract();
        let maps: Vec<(u64, u64, i64)> = str_to_vec::<u64>(data)
            .chunks(3)
            .map(|chunk| {
                let min = chunk[1];
                let max = chunk[1] + chunk[2];
                let delta = (chunk[0] as i64) - (chunk[1] as i64);
                (min, max, delta)
            })
            .collect();

        for id in &mut ids {
            for (min, max, delta) in &maps {
                if (*min..*max).contains(id) {
                    *id = ((*id as i64) + delta) as u64;
                    break;
                }
            }
        }
    }

    Some(ids.into_iter().min().unwrap().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let seed_pairs: Vec<u64> = capture_to_vec(&RE_SEEDS.captures(input).unwrap(), 1);
    let mut ids: Vec<(u64, u64)> = seed_pairs
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();

    for cap in RE_MAPS.captures_iter(input) {
        let (_, [_, _, data]) = cap.extract();
        let maps: Vec<((u64, u64), i64)> = str_to_vec::<u64>(data)
            .chunks(3)
            .map(|chunk| {
                let min = chunk[1];
                let max = chunk[1] + chunk[2];
                let delta = (chunk[0] as i64) - (chunk[1] as i64);
                ((min, max), delta)
            })
            .collect();

        let ranges: Vec<(u64, u64)> = maps.iter().map(|(rng, _)| *rng).collect();
        ids = range_split(ids, ranges.clone());

        ids = ids
            .iter()
            .map(|x| {
                let mut changed = *x;
                for (rng, delta) in &maps {
                    if range_overlap1(x, rng) {
                        changed = ((x.0 as i64 + delta) as u64, (x.1 as i64 + delta) as u64);
                        break;
                    }
                }
                changed
            })
            .collect();
    }
    Some(ids.iter().map(|x| x.0).min().unwrap().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
