mod solution;

use crate::solution::Solution;
fn main() {
    let isomorphic = Solution::is_isomorphic(String::from("badc"), String::from("baba"));
    println!("Hello, world!{}", isomorphic);
}
