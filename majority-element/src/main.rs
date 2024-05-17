mod solution;
use crate::solution::Solution;
fn main() {
    println!("Hello, world!{}", Solution::majority_element(vec![3, 3, 2]));
}
