mod solution;
use crate::solution::Solution;
fn main() {

    // 测试给定的例子
    let nums1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let nums2 = vec![1];
    let nums3 = vec![5, 4, -1, 7, 8];

    println!("{}", Solution::max_sub_array(nums1)); // 输出: 6
    println!("{}", Solution::max_sub_array(nums2)); // 输出: 1
    println!("{}", Solution::max_sub_array(nums3)); // 输出: 23
    println!("Hello, world!");
}
