mod solution;
use crate::solution::Solution;
fn main() {
    println!("{}", 52%26);
    let column_title = Solution::convert_to_title(52);
    println!("Hello, world!{}", column_title);
}
