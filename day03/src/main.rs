use std::{collections::VecDeque, print};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut matrix = input
        .lines()
        .map(|it| format!(".{}.", it).chars().collect::<Vec<_>>())
        .collect::<VecDeque<_>>();

    let dots = (0..matrix[0].len()).map(|_| '.').collect::<Vec<_>>();
    matrix.push_front(dots.clone());
    matrix.push_back(dots);

    let mut numbers = vec![];

    for (row, line) in matrix.iter().enumerate() {
        let mut active = String::new();

        for (col, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                active.push(*char);
            } else if !active.is_empty() {
                let mut validation = Vec::with_capacity(100);

                validation.push(matrix[row][col - active.len() - 1]);
                validation.push(matrix[row][col]);
                validation.extend_from_slice(
                    &matrix[row - 1][col.saturating_sub(active.len() + 1)..=col],
                );
                validation.extend_from_slice(
                    &matrix[row + 1][col.saturating_sub(active.len() + 1)..=col],
                );

                if validation.iter().any(|char| char != &'.') {
                    numbers.push(active.parse().unwrap());
                }

                active = String::new();
            }
        }
    }

    numbers.iter().sum()
}

fn part2(input: &str) -> i32 {
    let mut matrix = input
        .lines()
        .map(|it| format!(".{}.", it).chars().collect::<Vec<_>>())
        .collect::<VecDeque<_>>();

    let dots = (0..matrix[0].len()).map(|_| '.').collect::<Vec<_>>();
    matrix.push_front(dots.clone());
    matrix.push_back(dots);

    let mut numbers = vec![];

    for (row, line) in matrix.iter().enumerate() {
        let mut active = String::new();

        for (col, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                active.push(*char);
            } else if !active.is_empty() {
                numbers.push((
                    row,
                    (col - active.len() - 1)..(col + 1),
                    active.parse::<i32>().unwrap(),
                ));
                active = String::new();
            }
        }
    }

    let mut out = 0;

    for (row, line) in matrix.iter().enumerate() {
        for (col, _) in line.iter().enumerate().filter(|(_, c)| c == &&'*') {
            let count = numbers
                .iter()
                .filter(|(inner_row, range, _)| {
                    ((inner_row - 1)..=(inner_row + 1)).contains(&row) && range.contains(&col)
                })
                .collect::<Vec<_>>();

            if count.len() == 2 {
                out += count[0].2 * count[1].2;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 4361)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 467835)
    }
}
