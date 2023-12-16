pub trait Solution {
    fn name(&self) -> &str;
    fn solve(&self, lines: &[String]) -> String;
}

pub fn in_bounds<T>(grid: &[Vec<T>], i: isize, j: isize) -> bool {
    i >= 0 && j >= 0 && grid.len() as isize > i && grid[0].len() as isize > j
}
