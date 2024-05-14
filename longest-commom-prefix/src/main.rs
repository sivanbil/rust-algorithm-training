mod solution;
use crate::solution::Solution;
// https://leetcode.cn/problems/longest-common-prefix/description/
fn main() {

    let arr = vec!["cir".to_string(), "car".to_string(), "cdr".to_string()];
    let result = Solution::longest_common_prefix(arr);
    println!("result:{}", result);
}
