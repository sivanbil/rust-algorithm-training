mod solution;

use crate::solution::Solution;
// https://leetcode.cn/problems/isomorphic-strings/description/
fn main() {
    let isomorphic = Solution::is_isomorphic(String::from("badc"), String::from("baba"));
    println!("Hello, world!{}", isomorphic);
}
