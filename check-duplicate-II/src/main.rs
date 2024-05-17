mod solution;

// https://leetcode.cn/problems/contains-duplicate-ii/description/

use crate::solution::Solution;

fn main() {
    println!("Hello, world!{}", Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
}
