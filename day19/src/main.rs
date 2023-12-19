use std::collections::HashMap;

use common::itertools::Itertools;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Clone, Copy)]
enum Rule {
    GreaterThen((char, usize)),
    LessThen((char, usize)),
    Default,
}

#[derive(Debug, Clone)]
enum Action {
    ToWorkflow(String),
    Reject,
    Accept,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Part {
    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    pub fn value_of(&self, char: char) -> usize {
        match char {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Constraint {
    pub more: usize,
    pub less: usize,
}

impl Default for Constraint {
    fn default() -> Self {
        Self {
            more: 0,
            less: 4001,
        }
    }
}

impl Constraint {
    pub fn more_then(&self, value: usize) -> Self {
        Self {
            more: self.more.max(value),
            less: self.less,
        }
    }

    pub fn less_then(&self, value: usize) -> Self {
        Self {
            more: self.more,
            less: self.less.min(value),
        }
    }

    pub fn valid_values(&self) -> usize {
        if self.more > self.less {
            0
        } else {
            self.less - self.more - 1
        }
    }
}

fn part1(input: &str) -> usize {
    let (instructions, parts) = input.split("\n\n").collect_tuple().unwrap();

    fn parse_action(input: &str) -> Action {
        match input {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::ToWorkflow(input.to_string()),
        }
    }

    let instructions = instructions
        .lines()
        .map(|line| {
            let open_postition = line.chars().position(|c| c == '{').unwrap();

            let name = &line[..open_postition];

            let parts = line[(open_postition + 1)..(line.len() - 1)]
                .split(",")
                .map(|rule_part| match rule_part {
                    r if !r.contains(":") => (Rule::Default, parse_action(rule_part)),
                    r if r.contains("<") => {
                        let (part, number, goto) =
                            rule_part.split(&['<', ':']).collect_tuple().unwrap();

                        (
                            Rule::LessThen((part.chars().next().unwrap(), number.parse().unwrap())),
                            parse_action(goto),
                        )
                    }
                    r if r.contains(">") => {
                        let (part, number, goto) =
                            rule_part.split(&['>', ':']).collect_tuple().unwrap();

                        (
                            Rule::GreaterThen((
                                part.chars().next().unwrap(),
                                number.parse().unwrap(),
                            )),
                            parse_action(goto),
                        )
                    }
                    _ => panic!(),
                })
                .collect::<Vec<_>>();

            (name, parts)
        })
        .collect::<HashMap<_, _>>();

    let parts = &parts
        .lines()
        .map(|part| {
            let (x, m, a, s) = part[1..(part.len() - 1)]
                .split(",")
                .collect_tuple()
                .unwrap();

            Part {
                x: x[2..].parse().unwrap(),
                m: m[2..].parse().unwrap(),
                a: a[2..].parse().unwrap(),
                s: s[2..].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut accepted = vec![];

    for part in parts {
        let mut next = "in";

        'instruction: loop {
            let instruction = &instructions[next];

            for rule in instruction {
                match rule.0 {
                    Rule::GreaterThen((char, v)) => {
                        if !(part.value_of(char) > v) {
                            continue;
                        }
                    }
                    Rule::LessThen((char, v)) => {
                        if !(part.value_of(char) < v) {
                            continue;
                        }
                    }
                    Rule::Default => (),
                }

                match &rule.1 {
                    Action::ToWorkflow(value) => {
                        next = value;
                        continue 'instruction;
                    }
                    Action::Reject => {
                        break 'instruction;
                    }
                    Action::Accept => {
                        accepted.push(*part);
                        break 'instruction;
                    }
                }
            }

            unreachable!()
        }
    }

    accepted.into_iter().map(|p| p.sum()).sum()
}

fn part2(input: &str) -> usize {
    let (instructions, _) = input.split("\n\n").collect_tuple().unwrap();

    fn parse_action(input: &str) -> Action {
        match input {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::ToWorkflow(input.to_string()),
        }
    }

    let instructions = instructions
        .lines()
        .map(|line| {
            let open_postition = line.chars().position(|c| c == '{').unwrap();

            let name = &line[..open_postition];

            let parts = line[(open_postition + 1)..(line.len() - 1)]
                .split(",")
                .map(|rule_part| match rule_part {
                    r if !r.contains(":") => (Rule::Default, parse_action(rule_part)),
                    r if r.contains("<") => {
                        let (part, number, goto) =
                            rule_part.split(&['<', ':']).collect_tuple().unwrap();

                        (
                            Rule::LessThen((part.chars().next().unwrap(), number.parse().unwrap())),
                            parse_action(goto),
                        )
                    }
                    r if r.contains(">") => {
                        let (part, number, goto) =
                            rule_part.split(&['>', ':']).collect_tuple().unwrap();

                        (
                            Rule::GreaterThen((
                                part.chars().next().unwrap(),
                                number.parse().unwrap(),
                            )),
                            parse_action(goto),
                        )
                    }
                    _ => panic!(),
                })
                .collect::<Vec<_>>();

            (name, parts)
        })
        .collect::<HashMap<_, _>>();

    fn valid_count(
        next: &str,
        instructions: &HashMap<&str, Vec<(Rule, Action)>>,
        constraints: &mut HashMap<char, Constraint>,
    ) -> usize {
        instructions[next]
            .iter()
            .map(|(rule, action)| match rule {
                Rule::GreaterThen((char, value)) => {
                    let constraint = constraints[char];
                    let mut updated_constraints = constraints.clone();
                    *updated_constraints.get_mut(char).unwrap() = constraint.more_then(*value);
                    *constraints.get_mut(char).unwrap() = constraint.less_then(*value + 1);

                    check_constraints(instructions, &mut updated_constraints, action)
                }
                Rule::LessThen((char, value)) => {
                    let constraint = constraints[char];
                    let mut updated_constraints = constraints.clone();
                    *updated_constraints.get_mut(char).unwrap() = constraint.less_then(*value);
                    *constraints.get_mut(char).unwrap() = constraint.more_then(*value - 1);

                    check_constraints(instructions, &mut updated_constraints, action)
                }
                Rule::Default => check_constraints(instructions, constraints, action),
            })
            .sum()
    }

    fn check_constraints(
        instructions: &HashMap<&str, Vec<(Rule, Action)>>,
        constraints: &mut HashMap<char, Constraint>,
        action: &Action,
    ) -> usize {
        match action {
            Action::ToWorkflow(next) => valid_count(&next, instructions, constraints),
            Action::Reject => 0,
            Action::Accept => constraints.values().map(Constraint::valid_values).product(),
        }
    }

    let mut constraints = HashMap::new();
    constraints.insert('x', Constraint::default());
    constraints.insert('m', Constraint::default());
    constraints.insert('a', Constraint::default());
    constraints.insert('s', Constraint::default());

    valid_count("in", &instructions, &mut constraints)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 19114)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 167409079868000)
    }
}
