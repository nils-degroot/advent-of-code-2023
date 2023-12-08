use common::itertools::Itertools;
use common::num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let mut seperated = line.split(" = ");
            let title = seperated.next().unwrap().to_string();
            let rest = seperated.next().unwrap();
            let (lhs, rhs) = rest[1..(rest.len() - 1)]
                .split(", ")
                .collect_tuple::<(_, _)>()
                .unwrap();

            (title, (lhs.to_string(), rhs.to_string()))
        })
        .collect::<HashMap<_, _>>();

    let instruction_set = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    let mut instructions_done = 0;
    let mut location = "AAA".to_string();

    loop {
        let instruction = instruction_set[instructions_done % instruction_set.len()];
        location = match instruction {
            'L' => map.get(&location).unwrap().0.to_string(),
            'R' => map.get(&location).unwrap().1.to_string(),
            _ => panic!(),
        };

        instructions_done += 1;

        if location == "ZZZ" {
            break instructions_done;
        }
    }
}

fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let mut seperated = line.split(" = ");
            let title = seperated.next().unwrap();
            let rest = seperated.next().unwrap();
            let (lhs, rhs) = rest[1..(rest.len() - 1)]
                .split(", ")
                .collect_tuple::<(_, _)>()
                .unwrap();

            (title, (lhs, rhs))
        })
        .collect::<HashMap<_, _>>();

    let instruction_set = input.lines().next().unwrap().chars().collect::<Vec<_>>();

    map.keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .map(|mut location| {
            let mut instructions_done = 0;

            loop {
                let instruction = instruction_set[instructions_done % instruction_set.len()];

                location = match instruction {
                    'L' => map.get(location).unwrap().0,
                    'R' => map.get(location).unwrap().1,
                    _ => panic!(),
                };

                instructions_done += 1;

                if location.ends_with('Z') {
                    break instructions_done;
                }
            }
        })
        .reduce(|lhs, rhs| lcm(lhs, rhs))
        .unwrap()
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
        assert_eq!(out, 2)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT_2);
        assert_eq!(out, 6)
    }
}
