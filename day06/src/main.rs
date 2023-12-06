use common::itertools::Itertools;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    fn possible_wins_count(duration: i32, record: i32) -> i32 {
        let mut winning = false;
        let mut count = 0;
        let mut i = 0;

        loop {
            i += 1;

            if i * (duration - i) > record {
                count += 1;
                winning = true;
                continue;
            }

            if winning {
                break;
            }
        }

        count
    }

    let mut lines = input.lines();

    let durations = lines.next().unwrap().split_whitespace().skip(1);
    let records = lines.next().unwrap().split_whitespace().skip(1);

    durations
        .zip(records)
        .map(|(duration, record)| {
            possible_wins_count(duration.parse().unwrap(), record.parse().unwrap())
        })
        .product::<i32>()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();

    let duration: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();
    let record: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();

    let mut winning = false;
    let mut count = 0;
    let mut i = 0;

    loop {
        i += 1;

        if i * (duration - i) > record {
            count += 1;
            winning = true;
            continue;
        }

        if winning {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 288)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 71503)
    }
}
