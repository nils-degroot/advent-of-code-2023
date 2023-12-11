use common::itertools::Itertools;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1_000_000));
}

fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .flat_map(|line| {
            if line.contains("#") {
                vec![line]
            } else {
                vec![line, line]
            }
        })
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut to_expand = vec![];

    'outer: for (i, _) in grid[0].iter().enumerate() {
        for row in &grid {
            if row[i] == '#' {
                continue 'outer;
            }
        }

        to_expand.push(i);
    }

    for row in grid.iter_mut() {
        for value in to_expand.iter().rev() {
            row.insert(*value, '.');
        }
    }

    grid.iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(y, char)| if char == &'#' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .combinations(2)
        .map(|galexies| {
            let (lhs_x, lhs_y) = galexies[0];
            let (rhs_x, rhs_y) = galexies[1];

            let height = lhs_x.max(rhs_x) - lhs_x.min(rhs_x);
            let width = lhs_y.max(rhs_y) - lhs_y.min(rhs_y);

            height + width
        })
        .sum()
}

fn part2(input: &str, expansion: usize) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_columns = grid[0]
        .iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if grid.iter().all(|row| row[i] == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| if !row.contains(&'#') { Some(i) } else { None })
        .collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(y, char)| if char == &'#' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .combinations(2)
        .map(|galexies| {
            let (lhs_x, lhs_y) = galexies[0];
            let (rhs_x, rhs_y) = galexies[1];

            let row_extra = empty_rows
                .iter()
                .filter(|row| (lhs_x.min(rhs_x)..lhs_x.max(rhs_x)).contains(row))
                .count()
                * (expansion - 1);

            let col_extra = empty_columns
                .iter()
                .filter(|col| (lhs_y.min(rhs_y)..lhs_y.max(rhs_y)).contains(col))
                .count()
                * (expansion - 1);

            lhs_x.abs_diff(rhs_x) + lhs_y.abs_diff(rhs_y) + row_extra + col_extra
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
        assert_eq!(out, 374)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT, 10);
        assert_eq!(out, 1030);

        let out = part2(INPUT, 100);
        assert_eq!(out, 8410)
    }
}
