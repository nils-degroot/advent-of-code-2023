use common::itertools::Itertools;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut position = (0, 0);

    let mut seen = vec![position];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction = match parts.next().unwrap() {
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => panic!(),
        };

        let count = parts.next().unwrap().parse::<i32>().unwrap();

        for _ in 1..=count {
            position = (position.0 + direction.0, position.1 + direction.1);
            seen.push(position);
        }
    }

    let (shoelace1, shoelace2) = seen
        .iter()
        .tuple_windows()
        .fold((0i32, 0i32), |acc, (lhs, rhs)| {
            (acc.0 + lhs.0 * rhs.1, acc.1 + lhs.1 * rhs.0)
        });

    let area = (shoelace1.abs_diff(shoelace2) as usize) / 2;

    area - seen.len() / 2 + seen.len()
}

fn part2(input: &str) -> isize {
    let mut position = (0, 0);

    let mut seen = vec![position];
    let mut border = 0;

    for line in input.lines() {
        let part = line.split_whitespace().last().unwrap();
        let direction = match &part[7..=7] {
            "3" => (-1, 0),
            "1" => (1, 0),
            "2" => (0, -1),
            "0" => (0, 1),
            _ => panic!(),
        };

        let count = isize::from_str_radix(&part[2..(part.len() - 2)], 16).unwrap();
        border += count;
        position = (
            position.0 + direction.0 * count,
            position.1 + direction.1 * count,
        );
        seen.push(position);
    }

    let area = seen.iter().tuple_windows().fold(0isize, |acc, (lhs, rhs)| {
        acc + (lhs.0 + rhs.0) * (lhs.1 - rhs.1) / 2
    });

    border + (area - border / 2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 62)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 952408144115)
    }
}
