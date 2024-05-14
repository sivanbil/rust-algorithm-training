pub mod solution;
use crate::solution::Solution;
// https://leetcode.cn/problems/palindrome-number/description/
fn main() {
    let result = Solution::is_palindrome(11);
    println!("{}", result);
}
