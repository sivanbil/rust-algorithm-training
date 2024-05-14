mod solution;
use crate::solution::Solution;
// https://leetcode.cn/problems/contains-duplicate/solutions/
fn main() {
    let vec1 = vec![1, 2, 3, 1];
    let duplicate = Solution::contains_duplicate(vec1);
    println!("Hello, world!{}", duplicate);
}
