mod solution;
use crate::solution::Solution;
fn main() {
    println!("Hello, world!{}", Solution::find_duplicate(vec![1,3,4,2,2]));

    println!("Hello, world!{}", Solution::find_duplicate(vec![3,1,3,4,2]));

    println!("Hello, world!{}", Solution::find_duplicate(vec![3,3,3,3]));

}
