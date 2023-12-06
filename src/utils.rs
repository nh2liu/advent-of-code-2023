pub trait Solution {
    fn name(&self) -> &str;
    fn solve(&self, lines: &[String]) -> String;
}
