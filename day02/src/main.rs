use core::panic;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');

            let id = parts.next().unwrap();
            let id = id[5..].parse::<i32>().unwrap();

            let sets = parts.next().unwrap().trim();
            let over = sets
                .split([',', ';'])
                .map(|it| {
                    let mut parts = it.trim().split_whitespace();
                    (
                        parts.next().unwrap().parse::<i32>().unwrap(),
                        parts.next().unwrap(),
                    )
                })
                .any(|(count, color)| match color {
                    "red" => count > 12,
                    "green" => count > 13,
                    "blue" => count > 14,
                    _ => panic!(),
                });

            if over {
                0
            } else {
                id
            }
        })
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let sets = line.split(':').skip(1).next().unwrap();
            let (red, green, blue) = sets
                .split([',', ';'])
                .map(|it| {
                    let mut parts = it.trim().split_whitespace();
                    (
                        parts.next().unwrap().parse::<i32>().unwrap(),
                        parts.next().unwrap(),
                    )
                })
                .fold((0, 0, 0), |acc, (count, color)| match color {
                    "red" => (acc.0.max(count), acc.1, acc.2),
                    "green" => (acc.0, acc.1.max(count), acc.2),
                    "blue" => (acc.0, acc.1, acc.2.max(count)),
                    _ => panic!(),
                });

            red * green * blue
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 8)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 2286)
    }
}
