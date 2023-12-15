use advent_of_code::*;
use std::iter;
use std::ops::Range;

advent_of_code::solution!(3);

regex!(RE_NUM, r"\d+");
regex!(RE_SYM, r"[^0-9.]");
regex!(RE_STAR, r"[*]");

pub fn part_one(input: &str) -> Option<u32> {
    let mut line2: String;
    let mut line3 = "".to_string();

    let mut sym1: Vec<i32>;
    let mut sym2: Vec<i32> = vec![];
    let mut sym3: Vec<i32> = vec![];

    let mut sum: u32 = 0;
    for line in input.lines().chain(iter::once("")) {
        line2 = line3;
        line3 = line.to_string();

        sym1 = sym2;
        sym2 = sym3;
        sym3 = RE_SYM
            .find_iter(line3.as_str())
            .map(|s| s.start() as i32)
            .collect();

        for m in RE_NUM.find_iter(line2.as_str()) {
            let n: u32 = m.as_str().parse().unwrap();
            let mut near_sym = false;

            let s = m.start() as i32 - 1;
            let e = m.end() as i32;
            if sym2.contains(&s) || sym2.contains(&e) {
                near_sym = true;
            } else {
                for i in s..=e {
                    if sym1.contains(&i) || sym3.contains(&i) {
                        near_sym = true;
                        break;
                    }
                }
            }
            if near_sym {
                sum += n;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut line2: String;
    let mut line3 = "".to_string();

    let mut num1: Vec<(Range<i32>, u32)>;
    let mut num2: Vec<(Range<i32>, u32)> = vec![];
    let mut num3: Vec<(Range<i32>, u32)> = vec![];

    let mut sum: u32 = 0;
    for line in input.lines().chain(iter::once("")) {
        line2 = line3;
        line3 = line.to_string();

        num1 = num2;
        num2 = num3;
        num3 = RE_NUM
            .find_iter(line3.as_str())
            .map(|m| {
                let n: u32 = m.as_str().parse().unwrap();
                let s = m.start() as i32;
                let e = m.end() as i32;
                ((s..e), n)
            })
            .collect();

        for m in RE_STAR.find_iter(line2.as_str()) {
            let s = m.start() as i32;
            let mut adj: Vec<u32> = [].to_vec();

            for (k, v) in num1.iter() {
                if k.contains(&(s - 1)) || k.contains(&s) || k.contains(&(s + 1)) {
                    adj.push(*v);
                }
            }
            for (k, v) in num2.iter() {
                if k.contains(&(s - 1)) || k.contains(&(s + 1)) {
                    adj.push(*v);
                }
            }
            for (k, v) in num3.iter() {
                if k.contains(&(s - 1)) || k.contains(&s) || k.contains(&(s + 1)) {
                    adj.push(*v);
                }
            }
            if adj.len() == 2 {
                sum += adj[0] * adj[1]
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
