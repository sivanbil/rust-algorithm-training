mod solution;

use crate::solution::Solution;
fn main() {
    let result = Solution::add_binary(String::from("1010"), String::from("1011"));

    println!("Hello, world!{:?}", result);
}
