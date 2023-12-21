use advent_of_code::*;
use num::integer::lcm;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Button,
}

type Module = (ModuleType, String, Vec<String>);
type Pulse = (String, String, bool);

type Modules = HashMap<String, Module>;
type ModuleStates = HashMap<String, bool>;
type NestedModuleStates = HashMap<String, ModuleStates>;

pub fn part_one(input: &str) -> Option<u32> {
    let (modules, mut flipflop_states, mut conjunction_states) = parse(input);

    let mut high_emitters: HashSet<String> = HashSet::new();
    let (low_counts, high_counts): (Vec<u32>, Vec<u32>) = (0..1_000)
        .map(|_| {
            push_button(
                &modules,
                &mut flipflop_states,
                &mut conjunction_states,
                &mut high_emitters,
            )
        })
        .unzip();

    let low_count: u32 = low_counts.iter().sum();
    let high_count: u32 = high_counts.iter().sum();

    Some(low_count * high_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (modules, mut flipflop_states, mut conjunction_states) = parse(input);

    let rx_inputs = inputs(&modules, "rx".to_string());
    assert_eq!(rx_inputs, vec!("kl".to_string())); // By inspection, only kl leads to rx
    let kl_inputs = inputs(&modules, "kl".to_string());

    let mut cycles: HashMap<String, u64> = HashMap::new();

    let mut pushes: u64 = 0;
    while cycles.len() != kl_inputs.len() {
        pushes += 1;

        let mut high_emitters: HashSet<String> = HashSet::new();
        push_button(
            &modules,
            &mut flipflop_states,
            &mut conjunction_states,
            &mut high_emitters,
        );
        for m in &kl_inputs {
            if high_emitters.contains(m) && !cycles.contains_key(m) {
                cycles.insert(m.to_string(), pushes);
            }
        }
    }

    Some(cycles.values().copied().fold(1, lcm))
}

fn parse(input: &str) -> (Modules, ModuleStates, NestedModuleStates) {
    let mut modules: Modules = HashMap::new();
    let mut flipflop_states: ModuleStates = HashMap::new();
    let mut conjunction_states: NestedModuleStates = HashMap::new();

    for line in input.lines() {
        let module = parse_Module(line).unwrap();
        if matches!(module.0, ModuleType::FlipFlop) {
            flipflop_states.insert(module.1.clone(), false);
        }
        modules.insert(module.1.clone(), module);
    }
    modules.insert(
        "button".to_string(),
        (
            ModuleType::Button,
            "button".to_string(),
            vec!["broadcast".to_string()],
        ),
    );

    for module in modules.values() {
        for target in &module.2 {
            if let Some(target) = modules.get(target) {
                if matches!(target.0, ModuleType::Conjunction) {
                    if !conjunction_states.contains_key(&target.1) {
                        conjunction_states.insert(target.1.clone(), HashMap::new());
                    }
                    conjunction_states
                        .get_mut(&target.1)
                        .unwrap()
                        .insert(module.1.clone(), false);
                }
            }
        }
    }

    (modules, flipflop_states, conjunction_states)
}

fn inputs(modules: &Modules, name: String) -> Vec<String> {
    modules
        .values()
        .filter(|m| m.2.contains(&name))
        .map(|m| m.1.clone())
        .collect()
}

fn push_button(
    modules: &Modules,
    flipflop_states: &mut ModuleStates,
    conjunction_states: &mut NestedModuleStates,
    high_emitters: &mut HashSet<String>,
) -> (u32, u32) {
    let mut todo: VecDeque<Pulse> = VecDeque::new();
    let mut low_count: u32 = 0;
    let mut high_count: u32 = 0;

    todo.push_front(("button".to_string(), "button".to_string(), false));

    while let Some(pulse) = todo.pop_back() {
        let module = modules.get(&pulse.1);
        if module.is_none() {
            continue;
        }
        let module = module.unwrap();

        let emit = match module.0 {
            ModuleType::Button => Some(false),
            ModuleType::Broadcaster => Some(pulse.2),
            ModuleType::FlipFlop => {
                if !pulse.2 {
                    let new_state = !flipflop_states[&module.1];
                    flipflop_states.insert(module.1.clone(), new_state);
                    Some(new_state)
                } else {
                    None
                }
            }
            ModuleType::Conjunction => {
                let state = conjunction_states.get_mut(&module.1).unwrap();
                state.insert(pulse.0.clone(), pulse.2);
                Some(!state.values().all(|high| *high))
            }
        };

        if let Some(emit) = emit {
            for target in &module.2 {
                if emit {
                    high_count += 1;
                } else {
                    low_count += 1;
                }
                todo.push_front((pulse.1.clone(), target.clone(), emit));
            }
            if emit {
                high_emitters.insert(pulse.1.clone());
            }
        }
    }

    (low_count, high_count)
}

parser!(
    Module,
    r"^(broadcaster|[%&])([a-z]*) -> (.*)$",
    |g| match g {
        "broadcaster" => ModuleType::Broadcaster,
        "%" => ModuleType::FlipFlop,
        "&" => ModuleType::Conjunction,
        _ => todo!(),
    },
    |s| if s.is_empty() {
        "broadcast".to_string()
    } else {
        s.to_string()
    },
    list_parser!(|s| s.trim().to_string())
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }
}
