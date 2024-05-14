mod solution;
use crate::solution::Solution;
// https://leetcode.cn/problems/remove-element/description/
fn main() {
    let mut init_vec = vec![0,1,2,2,3,0,4,2];
    let result = Solution::remove_element(&mut init_vec, 2);

    println!("{:?}", init_vec);

    println!("result:{}", result);
}
