use std::collections::{HashSet, VecDeque};

use common::{
    grid::grid,
    rayon::prelude::{IntoParallelRefIterator, ParallelIterator},
};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &str) -> usize {
    let grid = grid(input);

    let mut visited = HashSet::with_capacity(512);

    let mut beams = Vec::with_capacity(512);
    beams.push((0, 0, Direction::Right));

    while let Some((x, y, direction)) = beams.pop() {
        if visited.contains(&(x, y, direction)) {
            continue;
        }

        visited.insert((x, y, direction));

        let new_directions = match grid[x][y] {
            '.' => {
                vec![direction]
            }
            '/' => match direction {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            '\\' => match direction {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            '-' => match direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                _ => vec![direction],
            },
            '|' => match direction {
                Direction::Left | Direction::Right => {
                    vec![Direction::Up, Direction::Down]
                }
                _ => vec![direction],
            },
            _ => panic!(),
        };

        for direction in new_directions {
            let (new_x, new_y) = match direction {
                Direction::Up => {
                    if x == 0 {
                        continue;
                    }

                    (x - 1, y)
                }
                Direction::Down => {
                    if x == grid.len() - 1 {
                        continue;
                    }

                    (x + 1, y)
                }
                Direction::Left => {
                    if y == 0 {
                        continue;
                    }

                    (x, y - 1)
                }
                Direction::Right => {
                    if y == grid[0].len() - 1 {
                        continue;
                    }

                    (x, y + 1)
                }
            };

            beams.push((new_x, new_y, direction));
        }
    }

    visited
        .into_iter()
        .fold(HashSet::<(usize, usize)>::new(), |mut acc, (x, y, _)| {
            acc.insert((x, y));
            acc
        })
        .len()
}

fn part2(input: &str) -> usize {
    let grid = grid(input);

    let mut start_options = Vec::with_capacity(512);
    for i in 0..grid.len() {
        start_options.push((i, 0, Direction::Right));
        start_options.push((i, grid[0].len() - 1, Direction::Left));
    }
    for i in 0..grid.len() {
        start_options.push((0, i, Direction::Down));
        start_options.push((grid.len() - 1, i, Direction::Up));
    }

    start_options
        .par_iter()
        .map(|start| {
            let mut visited = HashSet::with_capacity(512);

            let mut beams = vec![*start];

            while let Some((x, y, direction)) = beams.pop() {
                if visited.contains(&(x, y, direction)) {
                    continue;
                }

                visited.insert((x, y, direction));

                let new_directions = match grid[x][y] {
                    '.' => {
                        vec![direction]
                    }
                    '/' => match direction {
                        Direction::Up => vec![Direction::Right],
                        Direction::Down => vec![Direction::Left],
                        Direction::Left => vec![Direction::Down],
                        Direction::Right => vec![Direction::Up],
                    },
                    '\\' => match direction {
                        Direction::Up => vec![Direction::Left],
                        Direction::Down => vec![Direction::Right],
                        Direction::Left => vec![Direction::Up],
                        Direction::Right => vec![Direction::Down],
                    },
                    '-' => match direction {
                        Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                        _ => vec![direction],
                    },
                    '|' => match direction {
                        Direction::Left | Direction::Right => {
                            vec![Direction::Up, Direction::Down]
                        }
                        _ => vec![direction],
                    },
                    _ => panic!(),
                };

                for direction in new_directions {
                    let (new_x, new_y) = match direction {
                        Direction::Up => {
                            if x == 0 {
                                continue;
                            }

                            (x - 1, y)
                        }
                        Direction::Down => {
                            if x == grid.len() - 1 {
                                continue;
                            }

                            (x + 1, y)
                        }
                        Direction::Left => {
                            if y == 0 {
                                continue;
                            }

                            (x, y - 1)
                        }
                        Direction::Right => {
                            if y == grid[0].len() - 1 {
                                continue;
                            }

                            (x, y + 1)
                        }
                    };

                    beams.push((new_x, new_y, direction));
                }
            }

            visited
                .into_iter()
                .fold(HashSet::<(usize, usize)>::new(), |mut acc, (x, y, _)| {
                    acc.insert((x, y));
                    acc
                })
                .len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 46)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 51)
    }
}
