use std::collections::HashMap;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let lhs = line.chars().find(|c| c.is_numeric()).unwrap();
            let rhs = line.chars().rfind(|c| c.is_numeric()).unwrap();
            format!("{lhs}{rhs}").parse::<i32>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    input
        .lines()
        .map(|line| {
            let (lhs, _) = map
                .iter()
                .flat_map(|(text, digit)| {
                    vec![
                        line.find(&digit.to_string()).map(|pos| (digit, pos)),
                        line.find(text).map(|pos| (digit, pos)),
                    ]
                })
                .flatten()
                .min_by_key(|(_, p)| *p)
                .unwrap();

            let (rhs, _) = map
                .iter()
                .flat_map(|(text, digit)| {
                    vec![
                        line.rfind(&digit.to_string()).map(|pos| (digit, pos)),
                        line.rfind(text).map(|pos| (digit, pos)),
                    ]
                })
                .flatten()
                .max_by_key(|(_, p)| *p)
                .unwrap();

            format!("{lhs}{rhs}").parse::<i32>().unwrap()
        })
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
        assert_eq!(out, 142)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT_2);
        assert_eq!(out, 281)
    }
}
