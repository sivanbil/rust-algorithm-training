mod solution;

use crate::solution::Solution;
fn main() {
    println!("Hello, world!{:?}", Solution::third_max(vec![-2147483648,1,2]));

    println!("Hello, world!{:?}", Solution::third_max(vec![1,2,2]));
}
