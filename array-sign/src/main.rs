mod solution;

use crate::solution::Solution;
fn main() {
    println!("Hello, world!{}", Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));

    println!("Hello, world!{}", Solution::array_sign(vec![1, 3, 0, 5, 6]));
}
