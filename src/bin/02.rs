use advent_of_code::*;
use regex::Regex;

advent_of_code::solution!(2);

regex!(RE_GAME, r"^Game (\d+): (.*)");
regex!(RE_COLOR, r"(\d+) (blue|red|green)");

pub fn part_one(input: &str) -> Option<u32> {
    let possible = Rgb {
        r: 12,
        g: 13,
        b: 14,
    };

    let sum: u32 = parse(input)
        .filter(|(_, sets)| sets.iter().all(|set| set.is_less_or_equal(&possible)))
        .map(|(id, _)| id)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum: u32 = parse(input)
        .map(|(_, sets)| {
            let mut max = Rgb { r: 0, g: 0, b: 0 };
            for set in sets {
                if max.r < set.r {
                    max.r = set.r
                }
                if max.g < set.g {
                    max.g = set.g
                }
                if max.b < set.b {
                    max.b = set.b
                }
            }
            max.power()
        })
        .sum();
    Some(sum)
}

fn parse(input: &str) -> impl Iterator<Item = (u32, Vec<Rgb>)> + '_ {
    input.lines().map(|line| {
        let (_, [id, games]) = RE_GAME.captures(line).unwrap().extract();
        let id = id.parse::<u32>().unwrap();
        let sets: Vec<Rgb> = games.split("; ").map(|g| Rgb::new(g, &RE_COLOR)).collect();
        (id, sets)
    })
}

#[derive(Debug, Clone, Copy)]
struct Rgb {
    r: u32,
    g: u32,
    b: u32,
}

impl Rgb {
    fn new(s: &str, re: &Regex) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for (_, [v, c]) in re.captures_iter(s).map(|c| c.extract()) {
            let v = v.parse::<u32>().unwrap();
            match c {
                "red" => r = v,
                "green" => g = v,
                "blue" => b = v,
                &_ => {}
            }
        }

        Rgb { r, g, b }
    }

    fn is_less_or_equal(&self, other: &Self) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
