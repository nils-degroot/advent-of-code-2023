use common::num::Integer;
use std::collections::{BTreeMap, BinaryHeap, HashSet};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const VALID_GOING_UP: [char; 3] = ['|', '7', 'F'];
const VALID_GOING_DOWN: [char; 3] = ['|', 'J', 'L'];
const VALID_GOING_LEFT: [char; 3] = ['-', 'L', 'F'];
const VALID_GOING_RIGHT: [char; 3] = ['-', 'J', '7'];

fn part1(input: &str) -> usize {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = map.len();
    let width = map[0].len();

    let start_x = map.iter().position(|line| line.contains(&'S')).unwrap();
    let start_y = map[start_x].iter().position(|field| field == &'S').unwrap();

    let mut costs = BTreeMap::new();
    costs.insert((start_x, start_y), 0);

    let mut heap = BinaryHeap::new();
    heap.push(((start_x, start_y), 0));

    while let Some(((x, y), old_cost)) = heap.pop() {
        let new_cost = old_cost + 1;

        let up = (x.checked_sub(1).unwrap_or(0), y);
        if VALID_GOING_UP.contains(&map[up.0][up.1])
            && &new_cost <= costs.get(&up).unwrap_or(&usize::MAX)
        {
            costs.insert(up, new_cost);
            heap.push((up, new_cost));
        }

        let down = (x + 1, y);
        if down.0 < height
            && VALID_GOING_DOWN.contains(&map[down.0][down.1])
            && &new_cost <= costs.get(&down).unwrap_or(&usize::MAX)
        {
            costs.insert(down, new_cost);
            heap.push((down, new_cost));
        }

        let left = (x, y.checked_sub(1).unwrap_or(0));
        if VALID_GOING_LEFT.contains(&map[left.0][left.1])
            && &new_cost <= costs.get(&left).unwrap_or(&usize::MAX)
        {
            costs.insert(left, new_cost);
            heap.push((left, new_cost));
        }

        let right = (x, y + 1);
        if right.1 < width
            && VALID_GOING_RIGHT.contains(&map[right.0][right.1])
            && &new_cost <= costs.get(&right).unwrap_or(&usize::MAX)
        {
            costs.insert(right, new_cost);
            heap.push((right, new_cost));
        }
    }

    *costs.values().max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = map.len();
    let width = map[0].len();

    let start_x = map.iter().position(|line| line.contains(&'S')).unwrap();
    let start_y = map[start_x].iter().position(|field| field == &'S').unwrap();

    let mut main_loop_tiles = HashSet::new();
    main_loop_tiles.insert((start_x, start_y));

    let mut heap = BinaryHeap::new();
    heap.push((start_x, start_y));

    let up = (start_x.checked_sub(1).unwrap_or(0), start_y);
    let down = (start_x + 1, start_y);
    let left = (start_x, start_y.checked_sub(1).unwrap_or(0));
    let right = (start_x, start_y + 1);

    if VALID_GOING_UP.contains(&map[up.0][up.1]) && VALID_GOING_DOWN.contains(&map[down.0][down.1])
    {
        map[start_x][start_y] = '|'
    }
    if VALID_GOING_LEFT.contains(&map[left.0][left.1])
        && VALID_GOING_RIGHT.contains(&map[right.0][right.0])
    {
        map[start_x][start_y] = '-'
    }
    if VALID_GOING_UP.contains(&map[up.0][up.1]) && VALID_GOING_LEFT.contains(&map[left.0][left.1])
    {
        map[start_x][start_y] = 'J'
    }
    if VALID_GOING_UP.contains(&map[up.0][up.1])
        && VALID_GOING_RIGHT.contains(&map[right.0][right.1])
    {
        map[start_x][start_y] = 'L'
    }
    if VALID_GOING_DOWN.contains(&map[down.0][down.1])
        && VALID_GOING_LEFT.contains(&map[left.0][left.1])
    {
        map[start_x][start_y] = '7'
    }
    if VALID_GOING_DOWN.contains(&map[down.0][down.1])
        && VALID_GOING_RIGHT.contains(&map[right.0][right.1])
    {
        map[start_x][start_y] = 'F'
    }

    while let Some((x, y)) = heap.pop() {
        let current = &map[x][y];

        let up = (x.checked_sub(1).unwrap_or(0), y);
        if VALID_GOING_DOWN.contains(current)
            && VALID_GOING_UP.contains(&map[up.0][up.1])
            && !main_loop_tiles.contains(&up)
        {
            main_loop_tiles.insert(up);
            heap.push(up);
        }

        let down = (x + 1, y);
        if down.0 < height
            && VALID_GOING_UP.contains(current)
            && VALID_GOING_DOWN.contains(&map[down.0][down.1])
            && !main_loop_tiles.contains(&down)
        {
            main_loop_tiles.insert(down);
            heap.push(down);
        }

        let left = (x, y.checked_sub(1).unwrap_or(0));
        if VALID_GOING_RIGHT.contains(current)
            && VALID_GOING_LEFT.contains(&map[left.0][left.1])
            && !main_loop_tiles.contains(&left)
        {
            main_loop_tiles.insert(left);
            heap.push(left);
        }

        let right = (x, y + 1);
        if right.1 < width
            && VALID_GOING_LEFT.contains(current)
            && VALID_GOING_RIGHT.contains(&map[right.0][right.1])
            && !main_loop_tiles.contains(&right)
        {
            main_loop_tiles.insert(right);
            heap.push(right);
        }
    }

    let map_normalized: Vec<Vec<char>> = map
        .into_iter()
        .enumerate()
        .map(|(x, line)| {
            line.into_iter()
                .enumerate()
                .map(|(y, char)| {
                    if main_loop_tiles.contains(&(x, y)) {
                        char
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();

    let mut enclosed = 0;

    fn left_to_right(line: Vec<char>) -> bool {
        let line = line
            .into_iter()
            .collect::<String>()
            .replace("-", "")
            .replace("L7", "|")
            .replace("FJ", "|")
            .replace("LJ", "||")
            .replace("F7", "||");

        line.chars()
            .filter(|symbol| symbol == &'|')
            .count()
            .is_odd()
    }

    for x in 1..(height - 1) {
        for y in (1..(width - 1)).filter(|y| map_normalized[x][*y] == '.') {
            let from_y = left_to_right((0..=y).map(|y| map_normalized[x][y]).collect());
            if from_y {
                enclosed += 1;
            }
        }
    }

    enclosed
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample.in"));
    const INPUT_2: &'_ str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-sample-2.in"));

    const INPUT_2_1: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-2-1.in"));
    const INPUT_2_2: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-2-2.in"));
    const INPUT_2_3: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-2-3.in"));
    const INPUT_2_4: &'_ str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input-2-4.in"));

    #[test]
    fn part1_sample_input() {
        let out = part1(INPUT);
        assert_eq!(out, 4);

        let out = part1(INPUT_2);
        assert_eq!(out, 8);
    }

    #[test]
    fn part2_sample_input() {
        let out = part2(INPUT_2_1);
        assert_eq!(out, 4);

        let out = part2(INPUT_2_2);
        assert_eq!(out, 4);

        let out = part2(INPUT_2_3);
        assert_eq!(out, 8);

        let out = part2(INPUT_2_4);
        assert_eq!(out, 10);
    }
}
