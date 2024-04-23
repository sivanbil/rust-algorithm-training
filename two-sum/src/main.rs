pub mod solution;

use crate::solution::Solution;

fn main() {
    //let nums = vec![0,4,3,0];
    let nums = vec![-1,-2,-3,-4,-5];
    let target = -8;
    let result = Solution::two_sum(nums, target);

    println!("{:?}", result);
}
