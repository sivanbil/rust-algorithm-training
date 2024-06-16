mod solution;
use crate::solution::Solution;
fn main() {
    println!("Hello, world!{}", Solution::distribute_candies(vec![1,1,2,2,3,3]));
}
