use std::collections::{HashMap, VecDeque};

use common::{itertools::Itertools, num::Integer};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Clone)]
enum Module<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, u8>),
    Broadcaster,
    Button,
}

fn part1(input: &str) -> usize {
    let mut modules = input
        .lines()
        .map(|line| {
            let (header, targets) = line.split(" -> ").collect_tuple().unwrap();

            let (name, module) = match header {
                "broadcaster" => ("broadcaster", Module::Broadcaster),
                s if s.starts_with("%") => (&s[1..], Module::FlipFlop(false)),
                s if s.starts_with("&") => {
                    let sources = input
                        .lines()
                        .filter(|l| l.contains(&s[1..]) && !l.starts_with(s))
                        .map(|s| &s.split(" -> ").next().unwrap()[1..])
                        .collect::<Vec<_>>();

                    (
                        &s[1..],
                        Module::Conjunction(sources.iter().map(|t| (*t, 0)).collect()),
                    )
                }
                _ => panic!(),
            };

            let targets = targets.split(",").map(|t| t.trim()).collect::<Vec<_>>();

            (name, (module, targets))
        })
        .collect::<HashMap<_, _>>();

    modules.insert("button", (Module::Button, vec!["broadcaster"]));

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::with_capacity(512);
        pulses.push_back(("", "button", 0));

        while let Some((source, target, pulse)) = pulses.pop_front() {
            let module = match modules.get_mut(target) {
                Some(v) => v,
                None => continue,
            };
            let targets = module.1.clone();

            let output_pulse = match &module.0 {
                Module::FlipFlop(module_active) => {
                    if pulse == 1 {
                        None
                    } else if *module_active {
                        *module = (Module::FlipFlop(false), targets);
                        Some(0)
                    } else {
                        *module = (Module::FlipFlop(true), targets);
                        Some(1)
                    }
                }
                Module::Conjunction(state) => {
                    let mut state = state.clone();
                    state.insert(source, pulse);
                    *module = (Module::Conjunction(state.clone()), targets);

                    Some(if state.values().all(|v| v == &1) {
                        0
                    } else {
                        1
                    })
                }
                Module::Broadcaster => Some(pulse),
                Module::Button => Some(0),
            };

            if let Some(pulse) = output_pulse {
                for t in &module.1 {
                    if pulse == 0 {
                        low_count += 1;
                    } else {
                        high_count += 1;
                    }

                    pulses.push_back((target, t, pulse))
                }
            }
        }
    }

    low_count * high_count
}

fn part2(input: &str) -> usize {
    let mut modules = input
        .lines()
        .map(|line| {
            let (header, targets) = line.split(" -> ").collect_tuple().unwrap();

            let (name, module) = match header {
                "broadcaster" => ("broadcaster", Module::Broadcaster),
                s if s.starts_with("%") => (&s[1..], Module::FlipFlop(false)),
                s if s.starts_with("&") => {
                    let sources = input
                        .lines()
                        .filter(|l| l.contains(&s[1..]) && !l.starts_with(s))
                        .map(|s| &s.split(" -> ").next().unwrap()[1..])
                        .collect::<Vec<_>>();

                    (
                        &s[1..],
                        Module::Conjunction(sources.iter().map(|t| (*t, 0)).collect()),
                    )
                }
                _ => panic!(),
            };

            let targets = targets.split(",").map(|t| t.trim()).collect::<Vec<_>>();

            (name, (module, targets))
        })
        .collect::<HashMap<_, _>>();

    modules.insert("button", (Module::Button, vec!["broadcaster"]));

    let mut i = HashMap::<&str, usize>::new();
    let mut button_press = 1;

    'outer: loop {
        let mut pulses = VecDeque::with_capacity(512);
        pulses.push_back(("", "button", 0));

        while let Some((source, target, pulse)) = pulses.pop_front() {
            let module = match modules.get_mut(target) {
                Some(v) => v,
                None => continue,
            };
            let targets = module.1.clone();

            let output_pulse = match &module.0 {
                Module::FlipFlop(module_active) => {
                    if pulse == 1 {
                        None
                    } else if *module_active {
                        *module = (Module::FlipFlop(false), targets);
                        Some(0)
                    } else {
                        *module = (Module::FlipFlop(true), targets);
                        Some(1)
                    }
                }
                Module::Conjunction(state) => {
                    let mut state = state.clone();
                    state.insert(source, pulse);
                    *module = (Module::Conjunction(state.clone()), targets);

                    if target == "bn" && pulse == 1 {
                        if !i.contains_key(&source) {
                            i.insert(source, button_press);
                        }

                        if i.len() == 4 {
                            break 'outer;
                        }
                    }

                    Some(if state.values().all(|v| v == &1) {
                        0
                    } else {
                        1
                    })
                }
                Module::Broadcaster => Some(pulse),
                Module::Button => Some(0),
            };

            if let Some(pulse) = output_pulse {
                for t in &module.1 {
                    pulses.push_back((target, t, pulse))
                }
            }
        }

        button_press += 1;
    }

    i.values().fold(1, |lhs, rhs| lhs.lcm(rhs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    const INPUT_2: &'_ str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample-2.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 32000000)
    }

    #[test]
    fn part1_sample_input_2() {
        let out = part1(INPUT_2);
        assert_eq!(out, 11687500)
    }
}
