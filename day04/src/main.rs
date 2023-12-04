use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let line = &line[8..].to_string();
            let mut parts = line.split('|');
            let numbers = parts.next().unwrap().split_whitespace().collect::<Vec<_>>();
            let winning = parts
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<HashSet<_>>();

            numbers
                .into_iter()
                .filter(|num| winning.contains(num))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    let mut memo = HashMap::new();
    let mut cards = 0;

    for (index, line) in input.lines().enumerate() {
        let line = &line[8..].to_string();
        let mut parts = line.split('|');
        let numbers = parts.next().unwrap().split_whitespace().collect::<Vec<_>>();
        let winning = parts
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<HashSet<_>>();

        let matching = numbers
            .into_iter()
            .filter(|num| winning.contains(num))
            .count();

        let copies_of_card = *memo.get(&index).unwrap_or(&0) + 1;

        for increment in 1..=matching {
            match memo.get_mut(&(index + increment)) {
                Some(value) => {
                    *value += copies_of_card;
                }
                None => {
                    memo.insert(index + increment, copies_of_card);
                }
            }
        }

        cards += copies_of_card;
    }

    cards as i32
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
        assert_eq!(out, 13)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT_2);
        assert_eq!(out, 30)
    }
}
