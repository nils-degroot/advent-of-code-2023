use common::itertools::Itertools;
use common::rayon::prelude::*;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap()[7..].to_string();
    let mut seeds = seeds
        .split_whitespace()
        .map(|it| it.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let map = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let mut numbers = line
                        .split_whitespace()
                        .map(|part| part.parse::<usize>().unwrap());

                    let destination = numbers.next().unwrap();
                    let source_start = numbers.next().unwrap();
                    let range_len = numbers.next().unwrap();

                    MapInfo {
                        source_start,
                        destination,
                        range_len,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for part in &map {
        seeds = seeds
            .iter()
            .map(|seed| {
                for info in part {
                    if seed >= &info.source_start && seed < &(info.source_start + info.range_len) {
                        return seed - info.source_start + info.destination;
                    }
                }

                *seed
            })
            .collect();
    }

    *seeds.iter().min().unwrap() as i32
}

fn part2(input: &str) -> i32 {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap()[7..].to_string();
    let mut seeds = seeds
        .split_whitespace()
        .chunks(2)
        .into_iter()
        .flat_map(|mut parts| {
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let len = parts.next().unwrap().parse::<usize>().unwrap();
            start..(start + len)
        })
        .collect::<Vec<_>>();

    seeds.dedup();

    let map = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let mut numbers = line
                        .split_whitespace()
                        .map(|part| part.parse::<usize>().unwrap());

                    let destination = numbers.next().unwrap();
                    let source_start = numbers.next().unwrap();
                    let range_len = numbers.next().unwrap();

                    MapInfo {
                        source_start,
                        destination,
                        range_len,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for part in &map {
        seeds = seeds
            .par_iter()
            .map(|seed| {
                for info in part {
                    if seed >= &info.source_start && seed < &(info.source_start + info.range_len) {
                        return seed - info.source_start + info.destination;
                    }
                }

                *seed
            })
            .collect();
    }

    seeds.into_iter().min().unwrap() as i32
}

#[derive(Debug)]
struct MapInfo {
    pub source_start: usize,
    pub range_len: usize,
    pub destination: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 35)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 46)
    }
}
