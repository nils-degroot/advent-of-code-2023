use std::collections::HashMap;

use common::grid::{grid, Grid};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut grid = grid(input);

    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            if grid[x][y] == 'O' {
                let mut new_x = x;
                while new_x > 0 {
                    if grid[new_x - 1][y] == '.' {
                        grid[new_x][y] = '.';
                        grid[new_x - 1][y] = 'O';
                        new_x -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    grid.into_iter()
        .rev()
        .enumerate()
        .map(|(i, row)| (i + 1) * row.into_iter().filter(|c| c == &'O').count())
        .sum()
}

fn part2(input: &str) -> usize {
    let mut grid = grid(input);
    let mut iterations = Vec::<Grid<char>>::with_capacity(1000);
    iterations.push(grid.clone());

    let mut seen = HashMap::<Grid<char>, usize>::new();
    seen.insert(grid.clone(), 0);

    let (start, end) = loop {
        for y in 0..grid[0].len() {
            for x in 0..grid.len() {
                if grid[x][y] == 'O' {
                    let mut x = x;
                    while x > 0 {
                        if grid[x - 1][y] == '.' {
                            grid[x][y] = '.';
                            grid[x - 1][y] = 'O';
                            x -= 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == 'O' {
                    let mut y = y;
                    while y > 0 {
                        if grid[x][y - 1] == '.' {
                            grid[x][y] = '.';
                            grid[x][y - 1] = 'O';
                            y -= 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        for y in 0..grid[0].len() {
            for x in (0..grid.len()).rev() {
                if grid[x][y] == 'O' {
                    let mut x = x;
                    while x < grid.len() - 1 {
                        if grid[x + 1][y] == '.' {
                            grid[x][y] = '.';
                            grid[x + 1][y] = 'O';
                            x += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        for x in 0..grid.len() {
            for y in (0..grid[0].len()).rev() {
                if grid[x][y] == 'O' {
                    let mut y = y;
                    while y < grid[0].len() - 1 {
                        if grid[x][y + 1] == '.' {
                            grid[x][y] = '.';
                            grid[x][y + 1] = 'O';
                            y += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        if let Some(start) = seen.get(&grid) {
            break (start, iterations.len());
        }

        seen.insert(grid.clone(), iterations.len());
        iterations.push(grid.clone());
    };

    iterations[(1000000000 - start) % (end - start) + start]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| (i + 1) * row.into_iter().filter(|c| c == &&'O').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 136)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 64)
    }
}
