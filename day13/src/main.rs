use common::{
    grid::{grid, Grid},
    itertools::Itertools,
};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    fn index_of_reflection(rows: Vec<String>) -> Option<usize> {
        rows.iter()
            .enumerate()
            .tuple_windows::<(_, _)>()
            .find(|(lhs, rhs)| {
                let mut i = 0;
                loop {
                    if lhs.0 - i == 0 || rhs.0 + i == rows.len() - 1 {
                        break rows[lhs.0 - i] == rows[rhs.0 + i];
                    }
                    if rows[lhs.0 - i] != rows[rhs.0 + i] {
                        break false;
                    }

                    i += 1;
                }
            })
            .map(|((i, _), _)| i)
    }

    fn reflection_in_row(grid: &Grid<char>) -> Option<usize> {
        let rows = grid
            .iter()
            .map(|it| it.iter().collect::<String>())
            .collect::<Vec<_>>();

        index_of_reflection(rows)
    }

    fn reflection_in_column(grid: &Grid<char>) -> Option<usize> {
        let columns = (0..grid[0].len())
            .map(|i| grid.iter().map(|row| row[i]).collect::<String>())
            .collect::<Vec<_>>();

        index_of_reflection(columns)
    }

    input
        .split("\n\n")
        .map(grid)
        .map(|grid| {
            if let Some(index) = reflection_in_column(&grid) {
                index + 1
            } else if let Some(index) = reflection_in_row(&grid) {
                (index + 1) * 100
            } else {
                panic!()
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    fn index_of_reflection(rows: Vec<String>, ignore: Option<usize>) -> Option<usize> {
        let ignore = ignore.unwrap_or(usize::MAX);

        rows.iter()
            .enumerate()
            .tuple_windows::<(_, _)>()
            .find(|(lhs, rhs)| {
                if lhs.0 == ignore {
                    false
                } else {
                    let mut i = 0;
                    loop {
                        if lhs.0 - i == 0 || rhs.0 + i == rows.len() - 1 {
                            break rows[lhs.0 - i] == rows[rhs.0 + i];
                        }
                        if rows[lhs.0 - i] != rows[rhs.0 + i] {
                            break false;
                        }

                        i += 1;
                    }
                }
            })
            .map(|((i, _), _)| i)
    }

    fn reflection_in_row(grid: &Grid<char>, ignore: Option<usize>) -> Option<usize> {
        let rows = grid
            .iter()
            .map(|it| it.iter().collect::<String>())
            .collect::<Vec<_>>();

        index_of_reflection(rows, ignore)
    }

    fn reflection_in_column(grid: &Grid<char>, ignore: Option<usize>) -> Option<usize> {
        let columns = (0..grid[0].len())
            .map(|i| grid.iter().map(|row| row[i]).collect::<String>())
            .collect::<Vec<_>>();

        index_of_reflection(columns, ignore)
    }

    fn fix_smudge(grid: Grid<char>) -> usize {
        let (original_index, original_direction) =
            if let Some(index) = reflection_in_column(&grid, None) {
                (index, Direction::Column)
            } else if let Some(index) = reflection_in_row(&grid, None) {
                (index, Direction::Row)
            } else {
                panic!()
            };

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut grid = grid.clone();
                grid[i][j] = if grid[i][j] == '.' { '#' } else { '.' };

                match original_direction {
                    Direction::Row => {
                        if let Some(index) = reflection_in_column(&grid, None) {
                            return index + 1;
                        }
                        if let Some(index) = reflection_in_row(&grid, Some(original_index)) {
                            return (index + 1) * 100;
                        }
                    }
                    Direction::Column => {
                        if let Some(index) = reflection_in_column(&grid, Some(original_index)) {
                            return index + 1;
                        }
                        if let Some(index) = reflection_in_row(&grid, None) {
                            return (index + 1) * 100;
                        }
                    }
                }
            }
        }

        panic!()
    }

    input.split("\n\n").map(grid).map(fix_smudge).sum()
}

enum Direction {
    Row,
    Column,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 405)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 400)
    }
}
