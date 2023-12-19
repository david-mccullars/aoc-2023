use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(19);

type Rule = (char, char, u32, String);
type Workflow = (String, Vec<Rule>);
type Part = (char, u32);
type Parts = HashMap<char, u32>;

type Workflows = HashMap<String, Vec<Rule>>;
type Ratings = Vec<Parts>;
type PartsRanges = HashMap<char, (u32, u32)>;

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, ratings) = parse(input);

    let accepted: u32 = ratings
        .iter()
        .filter(|parts| is_accepted(parts, &workflows))
        .map(|parts| parts.values().sum::<u32>())
        .sum();

    Some(accepted)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse(input);
    let mut start: PartsRanges = HashMap::new();
    start.insert('x', (1, 4000));
    start.insert('m', (1, 4000));
    start.insert('a', (1, 4000));
    start.insert('s', (1, 4000));
    let est = estimate("in".to_string(), &start, &workflows);
    Some(est)
}

fn parse(input: &str) -> (Workflows, Ratings) {
    let i = input.find("\n\n").unwrap();
    let (top, bottom) = input.split_at(i);

    let mut workflows: Workflows = HashMap::new();
    for line in top.lines() {
        let (name, workflow) = parse_Workflow(line).unwrap();
        workflows.insert(name.to_string(), workflow);
    }

    let ratings = bottom
        .trim()
        .lines()
        .map(|line| parse_Parts(line).unwrap())
        .collect();

    (workflows, ratings)
}

parser!(
    Rule,
    r"^([a-z])([<>])(\d+):([a-z]+|[AR])$",
    char_parser!(),
    char_parser!(),
    u32_parser!(),
    string_parser!()
);

parser!(
    Workflow,
    r"^([a-z]+)\{(.*)\}$",
    string_parser!(),
    list_parser!(|s| parse_Rule(s).unwrap_or((' ', '=', 0, s.to_string())))
);

parser!(Part, r"^([a-z])=(\d+)$", char_parser!(), u32_parser!());

parser!(Parts, r"^\{(.*)\}$", |g| {
    let mut parts = HashMap::new();
    for (name, value) in g.split(',').map(|s| parse_Part(s).unwrap()) {
        parts.insert(name, value);
    }
    parts
});

fn is_accepted(parts: &Parts, workflows: &Workflows) -> bool {
    let mut wf = "in".to_string();
    loop {
        let rules = &workflows[&wf];
        for rule in rules {
            match rule.1 {
                '>' => {
                    if parts[&rule.0] > rule.2 {
                        wf = rule.3.clone();
                        break;
                    }
                }
                '<' => {
                    if parts[&rule.0] < rule.2 {
                        wf = rule.3.clone();
                        break;
                    }
                }
                '=' => {
                    wf = rule.3.clone();
                    break;
                }
                _ => panic!("{}", format!("Rule mismatch: {:?} | {:?}", rule, parts)),
            }
        }
        if wf == "R" {
            return false;
        }
        if wf == "A" {
            return true;
        }
    }
}

fn estimate(wf: String, ranges: &PartsRanges, workflows: &Workflows) -> u64 {
    if wf == "R" {
        return 0;
    }
    if wf == "A" {
        return ranges
            .iter()
            .map(|(_, (i1, i2))| {
                if i2 >= i1 {
                    (i2 - i1 + 1) as u64
                } else {
                    0_u64
                }
            })
            .product();
    }

    let mut ranges = ranges.clone();
    let rules = &workflows[&wf];

    let mut sum = 0;
    for rule in rules {
        if rule.1 == '>' || rule.1 == '<' {
            let (r1, r2) = split(&ranges, rule);
            sum += estimate(rule.3.clone(), &r1, workflows);
            ranges = r2;
        } else {
            sum += estimate(rule.3.clone(), &ranges, workflows);
        }
    }

    sum
}

fn split(ranges: &PartsRanges, rule: &Rule) -> (PartsRanges, PartsRanges) {
    let mut r1 = ranges.clone();
    let mut r2 = ranges.clone();
    let rng = ranges[&rule.0];
    match rule.1 {
        '>' => {
            r1.insert(rule.0, (rule.2 + 1, rng.1));
            r2.insert(rule.0, (rng.0, rule.2));
        }
        '<' => {
            r1.insert(rule.0, (rng.0, rule.2 - 1));
            r2.insert(rule.0, (rule.2, rng.1));
        }
        _ => panic!(),
    }
    (r1, r2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_parser() {
        assert_eq!(
            parse_Workflow("px{a<2006:qkq,m>2090:A,rfg}"),
            Some((
                "px".to_string(),
                vec![
                    ('a', '<', 2006, "qkq".to_string()),
                    ('m', '>', 2090, "A".to_string()),
                    (' ', '=', 0, "rfg".to_string()),
                ],
            ))
        );
    }

    #[test]
    fn test_parts_parser() {
        assert_eq!(
            parse_Parts("{x=787,m=2655,a=1222,s=2876}"),
            Some(HashMap::from([
                ('x', 787),
                ('m', 2655),
                ('a', 1222),
                ('s', 2876)
            ]))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
