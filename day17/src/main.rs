use common::grid::{digit_grid, Grid};
use std::{
    collections::{BinaryHeap, HashMap},
    unreachable,
};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    // println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &str) -> i32 {
    fn neighbors(
        position: (usize, usize),
        direction: (Direction, usize),
        map: &Grid<i32>,
    ) -> Vec<((usize, usize), (Direction, usize))> {
        match direction.0 {
            Direction::Up => vec![Direction::Up, Direction::Left, Direction::Right],
            Direction::Down => vec![Direction::Down, Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Up, Direction::Down, Direction::Left],
            Direction::Right => vec![Direction::Up, Direction::Down, Direction::Right],
        }
        .into_iter()
        .filter_map(|new_direction| {
            if new_direction == direction.0 && direction.1 == 3 {
                return None;
            }

            let new_count = if direction.0 == new_direction {
                direction.1 + 1
            } else {
                1
            };

            match new_direction {
                Direction::Up => {
                    if position.0 > 0 {
                        Some(((position.0 - 1, position.1), (new_direction, new_count)))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if position.0 < map.len() - 1 {
                        Some(((position.0 + 1, position.1), (new_direction, new_count)))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if position.1 > 0 {
                        Some(((position.0, position.1 - 1), (new_direction, new_count)))
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if position.1 < map[0].len() - 1 {
                        Some(((position.0, position.1 + 1), (new_direction, new_count)))
                    } else {
                        None
                    }
                }
            }
        })
        .collect::<Vec<_>>()
    }

    let map = digit_grid(input);
    let end = (map.len() - 1, map[0].len() - 1);

    let mut costs = HashMap::new();
    costs.insert(((0, 0), (Direction::Right, 1)), 0);
    costs.insert(((0, 0), (Direction::Down, 1)), 0);

    let mut heap = BinaryHeap::new();
    heap.push((0, (0, 0), (Direction::Right, 1)));
    heap.push((0, (0, 0), (Direction::Down, 1)));

    while let Some((old_cost, position, (direction, direction_count))) = heap.pop() {
        if position == end {
            return -old_cost;
        }

        let neighbors = neighbors(position, (direction, direction_count), &map);

        for (new_position, new_direction) in neighbors {
            let new_cost = old_cost - map[new_position.0][new_position.1];
            let new_key = &(new_position, new_direction);

            if new_cost < *costs.get(&new_key).unwrap_or(&i32::MAX) {
                costs.insert(*new_key, new_cost);
                heap.push((new_cost, new_key.0, new_key.1));
            }
        }
    }

    unreachable!()
}

fn part2(input: &str) -> i32 {
    let map = digit_grid(input);
    let end = (map.len() - 1, map[0].len() - 1);

    let mut costs = HashMap::new();
    costs.insert(((0i32, 0i32), Direction::Up), 0);
    costs.insert(((0, 0), Direction::Left), 0);

    let mut heap = BinaryHeap::new();
    heap.push((0, (0, 0), Direction::Up));
    heap.push((0, (0, 0), Direction::Left));

    while let Some((old_cost, (old_x, old_y), old_direction)) = heap.pop() {
        if (old_x as usize, old_y as usize) == end {
            return -old_cost;
        }

        let new_directions = match old_direction {
            Direction::Up | Direction::Down => {
                vec![(-1i32, 0i32, Direction::Left), (1, 0, Direction::Right)]
            }
            Direction::Left | Direction::Right => {
                vec![(0, -1, Direction::Up), (0, 1, Direction::Down)]
            }
        };

        for new_direction in new_directions {
            let mut extra_cost = 0;

            for i in 1..=10 {
                let new_position = (old_x + new_direction.0 * i, old_y + new_direction.1 * i);
                if !(0..map.len()).contains(&(new_position.0 as usize))
                    || !(0..map[0].len()).contains(&(new_position.1 as usize))
                {
                    break;
                }

                extra_cost += map[new_position.0 as usize][new_position.1 as usize];
                let new_cost = old_cost - extra_cost;

                if i >= 4
                    && -new_cost
                        < *costs
                            .get(&(new_position, new_direction.2))
                            .unwrap_or(&i32::MAX)
                {
                    heap.push((new_cost, new_position, new_direction.2));
                    costs.insert((new_position, new_direction.2), -new_cost);
                }
            }
        }
    }

    unreachable!()
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
        assert_eq!(out, 102)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT);
        assert_eq!(out, 94)
    }

    #[test]
    fn part2_sample_input_2() {
        let out = part2(INPUT_2);
        assert_eq!(out, 71)
    }
}
