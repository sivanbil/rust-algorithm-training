mod solution;
use crate::solution::Solution;
// ,https://leetcode.cn/problems/plus-one/description/
fn main() {
    let result_vec = vec![9,8,7,6,5,4,3,2,1,0];

    let result = Solution::plus_one(result_vec);
    println!("Hello, world!{:?}", result);
}
