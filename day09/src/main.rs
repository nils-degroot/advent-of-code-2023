use common::itertools::Itertools;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    fn next_value_in_seq(values: Vec<i32>) -> i32 {
        if values.iter().all(|n| n == &0) {
            return 0;
        }

        let last = *values.last().unwrap();

        let differences = values
            .into_iter()
            .tuple_windows()
            .map(|(now, next)| next - now)
            .collect::<Vec<_>>();

        next_value_in_seq(differences) + last
    }

    input
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            next_value_in_seq(values)
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    fn next_value_in_seq(values: Vec<i32>) -> i32 {
        if values.iter().all(|n| n == &0) {
            return 0;
        }

        let last = *values.last().unwrap();

        let differences = values
            .into_iter()
            .tuple_windows()
            .map(|(now, next)| now - next)
            .collect::<Vec<_>>();

        last - next_value_in_seq(differences)
    }

    input
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .rev()
                .collect::<Vec<_>>();

            next_value_in_seq(values)
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
        assert_eq!(out, 114)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 2)
    }
}
