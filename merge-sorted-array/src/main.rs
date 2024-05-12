mod solution;
use crate::solution::Solution;
fn main() {

    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);

    println!("Hello, world!{:?}", nums1);
}
