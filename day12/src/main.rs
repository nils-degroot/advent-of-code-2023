use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    fn arrangements(line: &str, rules: &VecDeque<usize>) -> usize {
        if rules.is_empty() && line.contains('#') {
            return 0;
        }
        if rules.is_empty() {
            return 1;
        }
        if line.len() < rules[0] {
            return 0;
        }

        match line.chars().next().unwrap() {
            '.' => arrangements(&line[1..], rules),
            '#' => {
                let mut updated_rules = rules.clone();
                let dropped = updated_rules.pop_front().unwrap();

                if line[..dropped].contains('.')
                    || line.chars().skip(dropped).next().unwrap_or('.') == '#'
                {
                    0
                } else if dropped == line.len() {
                    arrangements("", &updated_rules)
                } else {
                    arrangements(&line[(dropped + 1)..], &updated_rules)
                }
            }
            '?' => {
                arrangements(&format!(".{}", &line[1..]), &rules.clone())
                    + arrangements(&format!("#{}", &line[1..]), &rules.clone())
            }
            _ => panic!(),
        }
    }

    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            arrangements(
                parts.next().unwrap(),
                &parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|number| number.parse().unwrap())
                    .collect(),
            )
        })
        .sum()
}

fn part2(input: &str) -> usize {
    fn arrangements(
        line: &str,
        rules: &VecDeque<usize>,
        rule_index: usize,
        memo: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        if rule_index >= rules.len() && line.contains('#') {
            return 0;
        }
        if rule_index >= rules.len() {
            return 1;
        }
        if line.len() < rules[rule_index] {
            return 0;
        }
        if let Some(value) = memo.get(&(line.to_string(), rule_index)) {
            return *value;
        }

        let value = match line.chars().next().unwrap() {
            '.' => arrangements(&line[1..], rules, rule_index, memo),
            '#' => {
                let dropped = rules[rule_index];

                if line[..dropped].contains('.')
                    || line.chars().skip(dropped).next().unwrap_or('.') == '#'
                {
                    0
                } else if dropped == line.len() {
                    arrangements("", rules, rule_index + 1, memo)
                } else {
                    arrangements(&line[(dropped + 1)..], rules, rule_index + 1, memo)
                }
            }
            '?' => {
                arrangements(
                    &format!(".{}", &line[1..]),
                    &rules.clone(),
                    rule_index,
                    memo,
                ) + arrangements(
                    &format!("#{}", &line[1..]),
                    &rules.clone(),
                    rule_index,
                    memo,
                )
            }
            _ => panic!(),
        };

        memo.insert((line.to_string(), rule_index), value);
        value
    }

    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let line = parts.next().unwrap();
            let line = vec![line, line, line, line, line].join("?");

            let numbers = parts.next().unwrap();
            let numbers = vec![numbers, numbers, numbers, numbers, numbers].join(",");

            let mut memo = HashMap::new();

            arrangements(
                &line,
                &numbers
                    .split(',')
                    .map(|number| number.parse().unwrap())
                    .collect::<VecDeque<_>>(),
                0,
                &mut memo,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 21)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 525152)
    }
}
