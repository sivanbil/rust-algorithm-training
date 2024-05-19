mod solution;
use crate::solution::Solution;
// https://leetcode.cn/problems/median-of-two-sorted-arrays/description/
fn main() {
    println!("Hello, world!{}", Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
}
