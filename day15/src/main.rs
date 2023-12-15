fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.in"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, char| (acc + char as usize) * 17 % 256)
}

fn part1(input: &str) -> usize {
    input.trim().split(",").map(hash).sum()
}

fn part2(input: &str) -> usize {
    let mut boxes = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(vec![]);
    }

    for item in input.trim().split(",") {
        let key = item.split(&['-', '=']).next().unwrap();
        let relevant_box = &mut boxes[hash(key)];

        let old_lens_position = relevant_box
            .iter()
            .position(|(inner_key, _)| inner_key == &key);

        if item.contains("-") {
            if let Some(position) = old_lens_position {
                relevant_box.remove(position);
            }

            continue;
        }

        let new_pair = (
            key,
            item.chars().last().unwrap().to_digit(10).unwrap() as u8,
        );

        if let Some(position) = old_lens_position {
            relevant_box.remove(position);
            relevant_box.insert(position, new_pair);
        } else {
            relevant_box.push(new_pair);
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, values)| {
            values
                .iter()
                .enumerate()
                .map(|(j, (_, strength))| (i + 1) * (j + 1) * *strength as usize)
                .collect::<Vec<_>>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let out = part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(out, 1320)
    }

    #[test]
    fn part2_sample_input() {
        let out = part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(out, 145)
    }
}
