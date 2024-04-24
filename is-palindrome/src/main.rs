pub mod solution;
use crate::solution::Solution;
fn main() {
    let result = Solution::is_palindrome(11);
    println!("{}", result);
}
