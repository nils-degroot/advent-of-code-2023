pub use itertools;
pub use num;
pub use rayon;

pub mod grid {
    pub type Grid<T> = Vec<Vec<T>>;

    pub fn grid(input: &str) -> Grid<char> {
        input.lines().map(|line| line.chars().collect()).collect()
    }
}
