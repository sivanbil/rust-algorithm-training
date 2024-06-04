mod solution;

use crate::solution::Solution;
fn main() {
    println!("Hello, world!{}", Solution::rotate_string("abcde".to_string(), "cdeab".to_string()));
}
