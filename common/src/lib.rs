pub use itertools;
pub use num;
pub use rayon;

pub mod grid {
    pub type Grid<T> = Vec<Vec<T>>;

    pub fn grid(input: &str) -> Grid<char> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    pub fn digit_grid(input: &str) -> Grid<i32> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).expect("Invalid digit passed") as i32)
                    .collect()
            })
            .collect()
    }
}
