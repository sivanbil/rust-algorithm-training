mod solution;
use crate::solution::Solution;
fn main() {
    println!("Hello, world!{}", Solution::search(vec![4,5,6,7,0,1,2], 0));
    println!("Hello, world!{}", Solution::search(vec![4,5,6,7,0,1,2], 3));
}


