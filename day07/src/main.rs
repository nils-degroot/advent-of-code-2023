use std::{cmp::Ordering, collections::HashMap, unreachable};

use common::itertools::Itertools;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const ORDER_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn part1(input: &str) -> i32 {
    fn hand_cmp(lhs: &'_ str, rhs: &'_ str) -> Ordering {
        match (lhs, rhs) {
            (lhs, rhs) if five_of_a_kind(lhs) && !five_of_a_kind(rhs) => Ordering::Greater,
            (lhs, rhs) if !five_of_a_kind(lhs) && five_of_a_kind(rhs) => Ordering::Less,
            (lhs, rhs) if four_of_a_kind(lhs) && !four_of_a_kind(rhs) => Ordering::Greater,
            (lhs, rhs) if !four_of_a_kind(lhs) && four_of_a_kind(rhs) => Ordering::Less,
            (lhs, rhs) if full_house(lhs) && !full_house(rhs) => Ordering::Greater,
            (lhs, rhs) if !full_house(lhs) && full_house(rhs) => Ordering::Less,
            (lhs, rhs) if three_of_a_kind(lhs) && !three_of_a_kind(rhs) => Ordering::Greater,
            (lhs, rhs) if !three_of_a_kind(lhs) && three_of_a_kind(rhs) => Ordering::Less,
            (lhs, rhs) if two_pair(lhs) && !two_pair(rhs) => Ordering::Greater,
            (lhs, rhs) if !two_pair(lhs) && two_pair(rhs) => Ordering::Less,
            (lhs, rhs) if one_pair(lhs) && !one_pair(rhs) => Ordering::Greater,
            (lhs, rhs) if !one_pair(lhs) && one_pair(rhs) => Ordering::Less,
            (lhs, rhs) => {
                for (lhs_char, rhs_char) in lhs.chars().zip(rhs.chars()) {
                    let lhs_pos = ORDER.iter().position(|v| v == &lhs_char).unwrap();
                    let rhs_pos = ORDER.iter().position(|v| v == &rhs_char).unwrap();

                    match lhs_pos.cmp(&rhs_pos) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }

                Ordering::Equal
            }
        }
    }

    fn five_of_a_kind(input: &str) -> bool {
        highest_count(input) == 5
    }

    fn four_of_a_kind(input: &str) -> bool {
        highest_count(input) == 4
    }

    fn full_house(input: &str) -> bool {
        let map = fold_by_char(input);
        map.values().min().unwrap() == &2 && map.values().max().unwrap() == &3
    }

    fn three_of_a_kind(input: &str) -> bool {
        highest_count(input) == 3
    }

    fn two_pair(input: &str) -> bool {
        let map = fold_by_char(input);
        map.values().filter(|count| count == &&2).count() == 2
    }

    fn one_pair(input: &str) -> bool {
        highest_count(input) == 2
    }

    fn highest_count(input: &str) -> i32 {
        *fold_by_char(input).values().max().unwrap()
    }

    fn fold_by_char(input: &str) -> HashMap<char, i32> {
        input.chars().fold(HashMap::new(), |mut acc, v| {
            match acc.get_mut(&v) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    acc.insert(v, 1);
                }
            };
            acc
        })
    }

    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .sorted_by(|(lhs, _), (rhs, _)| hand_cmp(lhs, rhs))
        .enumerate()
        .map(|(index, (_, score))| (index as i32 + 1) * score)
        .sum()
}

fn part2(input: &str) -> i32 {
    fn hand_cmp(lhs: &'_ str, rhs: &'_ str) -> Ordering {
        match (lhs, rhs) {
            (lhs, rhs) if five_of_a_kind(lhs) && five_of_a_kind(rhs) => char_by_char_cmp(lhs, rhs),
            (lhs, rhs) if five_of_a_kind(lhs) && !five_of_a_kind(rhs) => Ordering::Greater,
            (lhs, rhs) if !five_of_a_kind(lhs) && five_of_a_kind(rhs) => Ordering::Less,
            (lhs, rhs) if four_of_a_kind(lhs) && four_of_a_kind(rhs) => char_by_char_cmp(lhs, rhs),
            (lhs, rhs) if four_of_a_kind(lhs) && !four_of_a_kind(rhs) => Ordering::Greater,
            (lhs, rhs) if !four_of_a_kind(lhs) && four_of_a_kind(rhs) => Ordering::Less,
            (lhs, rhs) if full_house(lhs) && full_house(rhs) => char_by_char_cmp(lhs, rhs),
            (lhs, rhs) if full_house(lhs) && !full_house(rhs) => Ordering::Greater,
            (lhs, rhs) if !full_house(lhs) && full_house(rhs) => Ordering::Less,
            (lhs, rhs) if three_of_a_kind(lhs) && three_of_a_kind(rhs) => {
                char_by_char_cmp(lhs, rhs)
            }
            (lhs, rhs) if three_of_a_kind(lhs) && !three_of_a_kind(rhs) => Ordering::Greater,
            (lhs, rhs) if !three_of_a_kind(lhs) && three_of_a_kind(rhs) => Ordering::Less,
            (lhs, rhs) if two_pair(lhs) && two_pair(rhs) => char_by_char_cmp(lhs, rhs),
            (lhs, rhs) if two_pair(lhs) && !two_pair(rhs) => Ordering::Greater,
            (lhs, rhs) if !two_pair(lhs) && two_pair(rhs) => Ordering::Less,
            (lhs, rhs) if one_pair(lhs) && one_pair(rhs) => char_by_char_cmp(lhs, rhs),
            (lhs, rhs) if one_pair(lhs) && !one_pair(rhs) => Ordering::Greater,
            (lhs, rhs) if !one_pair(lhs) && one_pair(rhs) => Ordering::Less,
            (lhs, rhs) => char_by_char_cmp(lhs, rhs),
        }
    }

    fn five_of_a_kind(input: &str) -> bool {
        let wat = highest_count(input);
        highest_count(input) == 5
    }

    fn four_of_a_kind(input: &str) -> bool {
        highest_count(input) == 4
    }

    fn full_house(input: &str) -> bool {
        let mut map = fold_by_char(input);
        map.remove(&'J');

        map.len() == 2
    }

    fn three_of_a_kind(input: &str) -> bool {
        highest_count(input) >= 3
    }

    fn two_pair(input: &str) -> bool {
        let map = fold_by_char(input);
        let jokers = *map.get(&'J').unwrap_or(&0);

        match jokers {
            5 | 4 | 3 | 2 => true,
            1 => map.values().max().unwrap() == &2,
            0 => map.values().filter(|count| count == &&2).count() == 2,
            _ => unreachable!(),
        }
    }

    fn one_pair(input: &str) -> bool {
        highest_count(input) == 2
    }

    fn highest_count(input: &str) -> i32 {
        let mut map = fold_by_char(input);

        let jokers = *map.get(&'J').unwrap_or(&0);
        map.remove(&'J');
        *map.values().max().unwrap_or(&0) + jokers
    }

    fn fold_by_char(input: &str) -> HashMap<char, i32> {
        input.chars().fold(HashMap::new(), |mut acc, v| {
            match acc.get_mut(&v) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    acc.insert(v, 1);
                }
            };
            acc
        })
    }

    fn char_by_char_cmp(lhs: &str, rhs: &str) -> Ordering {
        for (lhs_char, rhs_char) in lhs.chars().zip(rhs.chars()) {
            let lhs_pos = ORDER_2.iter().position(|v| v == &lhs_char).unwrap();
            let rhs_pos = ORDER_2.iter().position(|v| v == &rhs_char).unwrap();

            match lhs_pos.cmp(&rhs_pos) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Greater,
            }
        }

        Ordering::Equal
    }

    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .sorted_by(|(lhs, _), (rhs, _)| hand_cmp(lhs, rhs))
        .enumerate()
        .map(|(index, (_, score))| (index as i32 + 1) * score)
        .sum()
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
        assert_eq!(out, 6440)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT_2);
        assert_eq!(out, 5905)
    }
}
