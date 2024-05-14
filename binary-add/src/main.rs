mod solution;

use crate::solution::Solution;
// https://leetcode.cn/problems/add-binary/description/
fn main() {
    let result = Solution::add_binary(String::from("1010"), String::from("1011"));

    println!("Hello, world!{:?}", result);
}
